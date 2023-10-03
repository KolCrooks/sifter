use std::pin::Pin;

use tokio::{io::AsyncRead, pin};

use crate::{page::Page, page_manager::PageManager};

pub type SourceTag = u64;


pub struct Source<'a> {
    tag: SourceTag,
    cur_page: &'a Page,
    page_store: &'a PageManager,
}

impl<'a> Source<'a> {
    pub fn new(tag: SourceTag, page_size: usize, cur_page: &'a Page, page_store: &'a PageManager) -> Self {
        Source {
            tag,
            cur_page,
            page_store
        }
    }
}

pub struct SourceReader<'a> {
    source: Source<'a>,
    offset: usize,
}

impl<'a> AsyncRead for SourceReader<'a> {
    fn poll_read(
            self: std::pin::Pin<&mut Self>,
            cx: &mut std::task::Context<'_>,
            buf: &mut tokio::io::ReadBuf<'_>,
        ) -> std::task::Poll<std::io::Result<()>> {
        let pinned_page = Pin::new(&mut self.source.cur_page);
        // AsyncRead::poll_read(pinned_page, cx, buf)
        todo!()
    }
}