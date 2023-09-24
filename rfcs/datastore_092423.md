# Datastore

## Motivation
We need a way of interfacing with data that allows for the fastest possible reads and writes. This is difficult because it means juggling data in and out of memory while trying to not increase the complexity of the system.

## Design
There will be two parts. We will have a `DataSource`, which allows us to create and query for tagged blocks of data. It will be interfaced by asking `DataSource` for a tag, and if the tag doesn't exist, it will create a new blocks for it.

A block of data is a fixed size page that can be brought into memory or written to disk. It can contain partial data, and it can be written to disk in a partial state.

`DataSources` will loan an object called `SourceBlocks` that will provide a native interface for reading and writing, allowing the user to not need to worry about the actual loading in and out of memory. `SourceBlocks` can contain a mix of in memory and on disk data, will load and unload data as needed, and will allocate new blocks as needed.

A `SourceBlock` will also be a standard size, so optimizations like storing each block as a btree node can be done, and also will maintain it's own `RwLock`. 


## Implementation

```rs
/// Interface for accessing the underlying data associated with a block
struct SourceBlocks<'a> {...}

impl<'a> SourceBlocks<'a> {
    fn tag(&self) -> SourceTag;
    /// Can be used for determining the size of btree nodes to create
    fn block_size(&self) -> usize;
}

impl AsyncWrite for SourceBlocks<'a> {...};
impl AsyncRead for SourceBlocks<'a> {...};

trait DataSource {
    fn list_source_blocks(&self) -> Vec<BlockId>;
    /// Create a new source block
    fn make_source_block<'a>(&'a self) -> SourceBlocks<'a>;
    /// Get a source block by id
    fn get_source_block(&self, id: BlockId) -> SourceBlocks<'a>;

    fn alloc_block(&self) -> BlockId;
    fn free_block(&self, id: BlockId);
    fn flush(&self);
    /// Force blocks into memory
    fn force_load_blocks(&self, ids: &[BlockId]);
}
```