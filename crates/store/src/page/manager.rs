use std::{collections::{BTreeMap, HashMap}, mem::size_of, path::Path, sync::atomic::{AtomicU64, Ordering}};

use tokio::fs::{File, self};

use crate::{source::{SourceTag, Source}};
use serde::{Serialize, Deserialize};

type PageId = u64;

const GB: usize = 1_000_000_000; 

/// 4 GB
const MAX_FILE_SIZE: usize = 4 * GB;



pub struct PageManager {
    page_offsets: BTreeMap<PageId, u64>,
    files: BTreeMap<u64, File>,
    source_tag_lookups: HashMap<SourceTag, Vec<PageId>>,
    last_page_id: AtomicU64
}

#[derive(Serialize, Deserialize)]
pub struct SerializedPageManagerData {
    last_page_id: u64,
    page_offsets: BTreeMap<PageId, u64>,
    source_tag_lookups: HashMap<SourceTag, Vec<PageId>>
}

async fn load_page_manager_file(dir: impl AsRef<Path>) -> error::Result<SerializedPageManagerData> {
    let file_path = dir.as_ref().join("/page_manager_info.json");
    let file_data = fs::read(file_path).await?;
    let data: SerializedPageManagerData = serde_json::from_slice(&file_data)?;
    Ok(data)
}

impl PageManager {
    pub async fn new() -> error::Result<Self> {
        let config = config::get_config().await;
        let manager_data = load_page_manager_file(config.data_directory).await?;
        Ok(PageManager {
            files: BTreeMap::new(),
            page_offsets: manager_data.page_offsets,
            source_tag_lookups: manager_data.source_tag_lookups,
            last_page_id: AtomicU64::new(manager_data.last_page_id)
        })
    }

    fn list_pages(&self, tag: SourceTag) -> &Vec<PageId> {
        self.source_tag_lookups
            .entry(tag)
            .or_default()
    }

    /// get or create a new source block
    fn get_source<'a>(&'a self) -> Source<'a> {
        todo!()
    }

    /// Get a page by id
    async fn get_page(&self, id: PageId) -> error::Result<Option<RawPage>> {
        todo!()
    }

    pub(crate) fn new_page(&self) -> &RawPage {
        let id = self.last_page_id.fetch_add(1, Ordering::)
    }

    pub(crate) fn delete_page(&self, id: PageId) {
        todo!()
    }

    pub fn flush(&self) {
        todo!()
    }
    /// Force pages into memory
    fn force_load_pages(&self, ids: &[PageId]) {
        todo!()
    }
    
    /// Load a new page into memory and free old pages if needed
    pub(crate) async fn aquire_page_loan(&self, id: PageId) -> error::Result<()> {
        todo!()
    }
}