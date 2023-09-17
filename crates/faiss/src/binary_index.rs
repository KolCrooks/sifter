use autocxx::{WithinUniquePtr, c_int};
use cxx::UniquePtr;
use crate::error;

pub trait BinaryVector {
    const SIZE: usize;
    fn id(&self) -> i64;
    fn as_bytes(&self) -> &[u8];
}

pub struct IndexBinaryWrapper<VT: BinaryVector> {
    bytes: usize,
    inner: UniquePtr<faiss_sys::IndexBinaryHash>,
    _marker: std::marker::PhantomData<VT>,
}

/// Result from searching an index.
/// Contains the ids of the vectors and the distances from the search vector.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SearchResult {
    pub ids: Vec<i64>,
    pub distances: Vec<i32>,
}

impl SearchResult {
    pub(crate) fn new(mut raw_distances: Vec<i32>, mut raw_ids: Vec<i64>) -> Self {
        let mut len = raw_distances.len();
        for i in 0..len {
            if raw_distances[i] == -1 {
                len = i;
                break;
            }
        }
        raw_distances.truncate(len);
        raw_ids.truncate(len);
        Self {
            distances: raw_distances,
            ids: raw_ids,
        }
    }
}


impl<VT: BinaryVector> IndexBinaryWrapper<VT> {
    /// Create a new index with bytes*8 dimensions and nbits bits for each vector hash per vector.
    /// To learn about the effects of nbits, see the [faiss documentation](https://github.com/facebookresearch/faiss/wiki/Binary-indexes#the-indexbinaryhash-and-indexbinarymultihash).
    pub fn new() -> Self {
        let index = faiss_sys::IndexBinaryHash::new(c_int(VT::SIZE as i32 * 8), c_int(12))
            .within_unique_ptr();
        Self {
            bytes: VT::SIZE,
            inner: index,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn add_raw(&mut self, id: i64, x: &[u8]) -> error::Result<()> {
        if x.len() != self.bytes as usize {
            return Err(error::FaissError::InvalidArguments(format!(
                "Expected {} bytes, got {}",
                self.bytes,
                x.len()
            ))
            .into());
        }
        // SAFETY: This is only unsafe if the x pointer references data smaller than the vector size.
        // This is checked above.
        Ok(unsafe {
            self.inner
                .pin_mut()
                .add(id, x.as_ptr());
        })
    }

    pub fn add(&mut self, vector: &VT) -> error::Result<()> {
        self.add_raw(vector.id(), vector.as_bytes())
    }

    pub fn search(&mut self, search_vec: &[u8], n: usize, k: usize) -> error::Result<SearchResult> {
        if search_vec.len() != self.bytes as usize {
            return Err(error::FaissError::InvalidArguments(format!(
                "Expected {} bytes, got {}",
                self.bytes,
                search_vec.len()
            ))
            .into());
        }
        let mut distances = vec![0i32; k];
        let mut labels = vec![0i64; k];
        let search_vec_ptr = search_vec.as_ptr();

        unsafe {
            self.inner.pin_mut().search(
                n as i64,
                search_vec_ptr,
                k as i64,
                distances.as_mut_ptr(),
                labels.as_mut_ptr(),
                std::ptr::null(),
            );
        }

        Ok(SearchResult::new(distances, labels))
    }
}
