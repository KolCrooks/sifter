use faiss::{MetricType, index::IndexImpl};


struct Index {
    dims: usize,
    index: IndexImpl,
    metric: MetricType,
}

impl Index {
    fn foo(&self) {
        self.index
    }
}

fn faiss() {
    faiss::index_binary_factor()
}