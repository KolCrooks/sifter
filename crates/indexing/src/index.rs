use enum_dispatch::enum_dispatch;

use crate::binary_index::{IndexBinary, IndexBinaryChunked};

/// Trait for defining a vector that can be added to a Faiss binary index.
pub trait Vector: Send {
    fn id(&self) -> i64;
    fn as_bytes(&self) -> &[u8];
}

/// Result from searching an index.
/// Contains the ids of the vectors and the distances from the search vector.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SearchResult {
    pub id: i64,
    pub distance: i32,
}

impl PartialOrd for SearchResult {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.distance.cmp(&other.distance))
    }
}

impl Ord for SearchResult {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.cmp(&other.distance)
    }
}

#[async_trait::async_trait]
#[enum_dispatch]
pub trait Index {
    async fn add(&mut self, vector: &[u8]) -> error::Result<Box<[i64]>>;
    async fn search(&self, search_vec: &[u8], k: usize) -> error::Result<Vec<SearchResult>>;
    async fn search_range(
        &self,
        search_vec: &[u8],
        radius: usize,
    ) -> error::Result<Vec<SearchResult>>;
}
