// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NoteMetadata {
    #[prost(message, optional, tag = "1")]
    pub sender: ::core::option::Option<super::account::AccountId>,
    #[prost(uint32, tag = "2")]
    pub note_type: u32,
    #[prost(fixed32, tag = "3")]
    pub tag: u32,
    #[prost(fixed64, tag = "4")]
    pub execution_hint: u64,
    #[prost(fixed64, tag = "5")]
    pub aux: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Note {
    #[prost(fixed32, tag = "1")]
    pub block_num: u32,
    #[prost(uint32, tag = "2")]
    pub note_index: u32,
    #[prost(message, optional, tag = "3")]
    pub note_id: ::core::option::Option<super::digest::Digest>,
    #[prost(message, optional, tag = "4")]
    pub metadata: ::core::option::Option<NoteMetadata>,
    #[prost(message, optional, tag = "5")]
    pub merkle_path: ::core::option::Option<super::merkle::MerklePath>,
    /// This field will be present when the note is on-chain.
    /// details contain the `Note` in a serialized format.
    #[prost(bytes = "vec", optional, tag = "6")]
    pub details: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NoteSyncRecord {
    #[prost(uint32, tag = "1")]
    pub note_index: u32,
    #[prost(message, optional, tag = "2")]
    pub note_id: ::core::option::Option<super::digest::Digest>,
    #[prost(message, optional, tag = "3")]
    pub metadata: ::core::option::Option<NoteMetadata>,
    #[prost(message, optional, tag = "4")]
    pub merkle_path: ::core::option::Option<super::merkle::MerklePath>,
}
