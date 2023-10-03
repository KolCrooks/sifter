use super::RawPage;
use tokio::sync::RwLock;

struct CacheItem {
    dirty: bool,
    data: RwLock<RawPage>
}

struct LFUPageCache {
    items: Box<[CacheItem]>,
}

impl LFUPageCache {
    // fn get(&self, key: PageId);
}