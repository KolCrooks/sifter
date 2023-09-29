use std::{cmp::Reverse, collections::BinaryHeap, mem::size_of};

use crate::{
    distance_metrics::{DistanceMetric, DistanceMetricFn, VecChunk},
    index::{Index, SearchResult},
};
use enum_dispatch::enum_dispatch;
use tokio::sync::RwLock;

/// Safe wrapper around the faiss IndexBinary. Currently only uses the IndexBinaryFlat implementation.
pub struct IndexBinaryChunked<ChunkT: VecChunk> {
    vector_bytes: u32,
    chunks_per_vec: usize,
    distance_metric: DistanceMetricFn<ChunkT>,
    data: RwLock<Vec<ChunkT>>,
}

#[inline]
/// Casts u8 to T while also updating the alignment to the size of T.
fn cast_slice_to<T: Sized>(x: &[u8]) -> Box<[T]> {
    let ret_cnt = x.len() / size_of::<T>();
    let mut v = Vec::with_capacity(ret_cnt);
    // Safety: This is safe because the size of the vector is correct.
    unsafe {
        std::ptr::copy_nonoverlapping(x.as_ptr(), v.as_ptr() as *mut u8, x.len());
        v.set_len(ret_cnt);
    }
    v.into_boxed_slice()
}

impl<ChunkT: VecChunk> IndexBinaryChunked<ChunkT> {
    pub fn new(vector_bytes: u32, distance_metric: DistanceMetric) -> Self {
        let chunks_per_vec = vector_bytes as usize / size_of::<ChunkT>();
        Self {
            vector_bytes,
            chunks_per_vec,
            distance_metric: distance_metric.into_fn(chunks_per_vec),
            data: RwLock::new(Vec::new()),
        }
    }

    pub async fn add_raw(&mut self, n: usize, x: &[u8]) -> error::Result<Box<[i64]>> {
        self.vec_size_check(n, x)?;

        // Safety: This is safe because x passed vec size_check
        let x = cast_slice_to::<ChunkT>(x);

        let mut lock = self.data.write().await;
        let original_size = lock.len();
        lock.reserve(x.len());
        unsafe {
            lock.set_len(original_size + x.len());
        }
        lock[original_size..].copy_from_slice(&x);

        Ok((original_size as i64..(original_size + n) as i64).collect())
    }

    fn vec_size_check(&self, n: usize, x: &[u8]) -> error::Result<()> {
        if x.len() != n * self.vector_bytes as usize {
            return Err(error::CustomErrors::InvalidArguments(format!(
                "Expected {} bytes, got {}",
                self.vector_bytes * n as u32,
                x.len()
            ))
            .into());
        }
        Ok(())
    }
}

#[async_trait::async_trait]
impl<ChunkT: VecChunk> Index for IndexBinaryChunked<ChunkT> {
    async fn add(&mut self, vector: &[u8]) -> error::Result<Box<[i64]>> {
        self.add_raw(1, vector).await
    }

    async fn search(&self, search_vec: &[u8], k: usize) -> error::Result<Vec<SearchResult>> {
        self.vec_size_check(1, search_vec)?;

        // Safety: This is safe because search_vec passed vec size_check
        let search_vec = cast_slice_to::<ChunkT>(search_vec);

        let mut heap = BinaryHeap::new();
        let lock = self.data.read().await;
        let mut offset = 0;

        while offset < lock.len() {
            // SAFETY: This is safe because the data and the search vector are of the same length,
            // and the offset is always aligned to the size of the type.
            let distance =
                (self.distance_metric)(&lock[offset..offset + self.chunks_per_vec], &search_vec);

            heap.push(Reverse(SearchResult {
                id: offset as i64 / self.chunks_per_vec as i64,
                distance: distance as i32,
            }));
            offset += self.chunks_per_vec;
        }

        let mut ret = Vec::with_capacity(k);
        for _ in 0..k {
            if let Some(res) = heap.pop() {
                ret.push(res.0);
            } else {
                break;
            }
        }

        Ok(ret)
    }

    async fn search_range(
        &self,
        search_vec: &[u8],
        radius: usize,
    ) -> error::Result<Vec<SearchResult>> {
        self.vec_size_check(1, search_vec)?;

        let search_vec = cast_slice_to::<ChunkT>(search_vec);
        let mut result = Vec::new();
        let lock = self.data.read().await;
        let mut offset = 0;

        while offset < lock.len() {
            // SAFETY: This is safe because the data and the search vector are of the same length,
            // and the offset is always aligned to the size of the type.
            let distance =
                (self.distance_metric)(&lock[offset..offset + self.chunks_per_vec], &search_vec);
            if distance <= radius as u32 {
                result.push(SearchResult {
                    id: offset as i64 / self.chunks_per_vec as i64,
                    distance: distance as i32,
                });
            }
            offset += self.chunks_per_vec;
        }
        Ok(result)
    }
}

#[enum_dispatch(Index, IndexSerde)]
pub enum IndexBinary {
    IndexBinaryStd8(IndexBinaryChunked<u8>),
    IndexBinaryStd16(IndexBinaryChunked<u16>),
    IndexBinaryStd32(IndexBinaryChunked<u32>),
    IndexBinaryStd64(IndexBinaryChunked<u64>),
}

impl IndexBinary {
    pub fn new(vec_dims: u32, metric: DistanceMetric) -> Self {
        if vec_dims % 64 == 0 {
            IndexBinary::IndexBinaryStd64(IndexBinaryChunked::new(vec_dims / 8, metric))
        } else if vec_dims % 32 == 0 {
            IndexBinary::IndexBinaryStd32(IndexBinaryChunked::new(vec_dims / 8, metric))
        } else if vec_dims % 16 == 0 {
            IndexBinary::IndexBinaryStd16(IndexBinaryChunked::new(vec_dims / 8, metric))
        } else {
            IndexBinary::IndexBinaryStd8(IndexBinaryChunked::new(vec_dims / 8, metric))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::index::Vector;

    use super::*;

    struct HashIndex {
        id: i64,
        data: [u8; 64],
    }

    impl Vector for HashIndex {
        #[inline]
        fn id(&self) -> i64 {
            self.id
        }
        #[inline]
        fn as_bytes(&self) -> &[u8] {
            &self.data
        }
    }

    macro_rules! index {
        [$({id: $id:expr, data: $data:expr }),*] => {
            {
                let mut idx = IndexBinary::new(512, DistanceMetric::Hamming);
                $(
                    idx.add(&HashIndex{id: $id, data: $data}.as_bytes()).await.unwrap();
                )*
                idx
            }
        };
    }

    #[tokio::test]
    async fn search() {
        let mut data2 = [0; 64];
        data2[0] = 1;
        let idx = index![{id: 1, data: [0; 64]}, {id: 2, data: data2}];
        let res = idx.search(&[0; 64], 2).await.unwrap();
        dbg!(&res);

        assert!(res.len() == 2);
        assert_eq!(res[0].id, 0);
        assert_eq!(res[1].id, 1);
        assert_eq!(res[0].distance, 0);
        assert_eq!(res[1].distance, 1);
    }

    #[tokio::test]
    async fn search_range() {
        let mut data2 = [0; 64];
        data2[0] = 1;
        let idx = index![{id: 1, data: [0; 64]}, {id: 2, data: data2}];
        let res = idx.search_range(&[0; 64], 1).await.unwrap();
        dbg!(&res);
        assert!(res.len() == 2);
    }
}
