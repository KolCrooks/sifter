pub mod error;
mod binary_index;
pub use binary_index::*;

pub use faiss_sys;

#[cfg(test)]
mod tests {
    use super::*;

    struct HashIndex {
        id: i64,
        data: [u8; 64]
    }

    impl BinaryVector for HashIndex {
        const SIZE: usize = 64;
        #[inline]
        fn id(&self) -> i64 {
            self.id
        }
        #[inline]
        fn as_bytes(&self) -> &[u8] {
            &self.data
        }
    }

    #[test]
    fn it_works() {
        let mut idx = IndexBinaryWrapper::new();
        idx.add(&HashIndex{id: 1, data: [0; 64]}).unwrap();
        let mut dat2 = [0; 64];
        dat2[0] = 1;
        idx.add(&HashIndex{id: 2, data: dat2}).unwrap();
        let res = idx.search(&[0;64], 2, 2).unwrap();
        dbg!(&res);
        assert_eq!(res.ids[0], 1);
        assert_eq!(res.ids[1], 2);
        assert_eq!(res.ids.len(), 2);
        assert_eq!(res.distances[0], 0);
        assert_eq!(res.distances[1], 1);
    }
}
