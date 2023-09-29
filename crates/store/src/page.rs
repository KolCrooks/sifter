use std::{task::Poll, pin::Pin, future::Future};

use tokio::{
    io::AsyncRead,
    sync::{RwLock, RwLockReadGuard}, pin,
};

use crate::{source::SourceTag, page_manager::{self, PageManager}};

pub type PageId = u64;

pub struct InMemoryData {
    data: Box<[u8]>,
    dirty: bool,
}

pub enum PageData {
    InMemory(RwLock<InMemoryData>),
    OnDisk,
}

struct PageHeader {
    size: u64,
    id: PageId,
    tag: SourceTag,
}

pub struct Page {
    header: PageHeader,
    disk_offset: u64,
    data: PageData,
}

enum PageReadState<'a> {
    Done,
    AcquiringRead(Pin<Box<dyn Future<Output = RwLockReadGuard<'a, InMemoryData>>>>),
    AcquiringPage(Pin<Box<dyn Future<Output = Result<(), error::Error>>>>),
}

pub struct PageReader<'a> {
    page: &'a Page,
    state: PageReadState<'a>,
}

impl<'a> PageReader<'a> {
    fn new(page: &'a Page, page_manager: PageManager) -> Self {
       let state = match page.data {
            PageData::InMemory(ref data) => {
                let state = data.read();
                PageReadState::AcquiringRead(Box::pin(state))
            }
            PageData::OnDisk => {
                let state = page_manager.aquire_page_loan(page.header.id);
                PageReadState::AcquiringPage(Box::pin(state))
            }
        };

        PageReader {
            page,
            state
        }
    }
}

impl<'a> AsyncRead for PageReader<'a> {
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        let mut this = *self;
        match this.state {
            PageReadState::Done => Poll::Ready(Ok(())),
            PageReadState::AcquiringRead(mut fut) => {
                let mut guard = match fut.as_mut().poll(cx) {
                    Poll::Ready(guard) => guard,
                    Poll::Pending => return Poll::Pending,
                };
                buf.put_slice(&guard.data);
                this.state = PageReadState::Done;
                Poll::Ready(Ok(()))
            }
            PageReadState::LoadingPage(fut) => {

            },
        }
    }
}
