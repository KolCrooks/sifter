use std::ops::BitXor;

pub trait VecChunk: BitXor<Output = Self> + Sized + Copy + Send + Sync {
    fn count_ones(self) -> u32;
}

/// Distance metric for binary vectors.
pub type DistanceMetricFn<T> = fn(a: &[T], b: &[T]) -> u32;


macro_rules! impl_vec_chunk {
    ($($t:ty)*) => ($(
        impl VecChunk for $t {
            #[inline]
            fn count_ones(self) -> u32 {
                self.count_ones()
            }
        }
    )*)
}
impl_vec_chunk!(u8 u16 u32 u64);

#[inline]
fn distance_const<const CHUNKS_PER_VEC: usize, ChunkT: VecChunk>(a: &[ChunkT], b: &[ChunkT]) -> u32 {
    let mut dist = 0;
    for i in 0..CHUNKS_PER_VEC {
        dist += (a[i] ^ b[i] ).count_ones();
    }
    dist
}

fn distance_dyn<ChunkT: VecChunk>(a: &[ChunkT], b: &[ChunkT]) -> u32 {
    let mut dist = 0;
    for i in 0..a.len() {
        dist += (a[i] ^ b[i] ).count_ones();
    }
    dist
}


/// Distance metrics for binary vectors.
pub enum DistanceMetric {
    Hamming
}

impl DistanceMetric {
    /// Returns a boxed distance metric so that it can be used in indexing.
    pub fn into_fn<ChunkT: VecChunk>(self, chunks_per_vec: usize) -> DistanceMetricFn<ChunkT> {
        match self {
            DistanceMetric::Hamming => {
                macro_rules! dist {
                    ($num: expr) => {
                        distance_const::<$num, ChunkT>
                    }
                }
                match chunks_per_vec {
                    1 => dist!(1),
                    2 => dist!(2),
                    3 => dist!(3),
                    4 => dist!(4),
                    5 => dist!(5),
                    6 => dist!(6),
                    7 => dist!(7),
                    8 => dist!(8),
                    9 => dist!(9),
                    10 => dist!(10),
                    _ => distance_dyn::<ChunkT>,
                }
            }
        }
    }
}