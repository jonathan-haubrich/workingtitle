// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DispatchMessage {
    #[prost(string, tag = "1")]
    pub module_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub function_id: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "3")]
    pub payload: ::prost::alloc::vec::Vec<u8>,
}