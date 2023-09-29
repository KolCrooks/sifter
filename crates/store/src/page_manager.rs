use crate::{source::{SourceTag, Source}, page::{Page, PageId}};

pub struct PageManager {}

impl PageManager {
    pub fn new() -> Self {
        PageManager {}
    }

    fn list_pages<'a>(&'a self, tag: SourceTag) -> Vec<&'a Page> {
        todo!()
    }

    /// get or create a new source block
    fn get_source<'a>(&'a self) -> Source<'a> {
        todo!()
    }

    /// Get a page by id
    fn get_page<'a>(&'a self, id: PageId) -> &'a Page {
        todo!()
    }

    pub(crate) fn new_page<'a>(&'a self) -> &'a Page {
        todo!()
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