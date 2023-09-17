#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Test {
    #[prost(string, tag = "1")]
    pub a: ::prost::alloc::string::String,
}
