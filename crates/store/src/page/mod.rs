use crate::source::SourceTag;

mod cache;
mod manager;
mod rwa_lock;


/// 4096 KB
const PAGE_SIZE: usize = 4096 * 1000;
struct PageHeader {
    id: u64,
    tag: SourceTag,
}

// Align data to 64 so that it can be index into as if it were collections of 64bit integers
#[repr(align(64))]
struct RawPage {
    offset: u64,
    header: PageHeader,
    data: [u8; PAGE_SIZE - std::mem::size_of::<PageHeader>()],
}