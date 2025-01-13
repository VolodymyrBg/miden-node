use miden_objects::{crypto::hash::rpo::RpoDigest, transaction::TransactionId};

use crate::{errors::ConversionError, generated as proto};

// FROM TRANSACTION ID
// ================================================================================================

impl From<&TransactionId> for proto::digest::Digest {
    fn from(value: &TransactionId) -> Self {
        (*value).inner().into()
    }
}

impl From<TransactionId> for proto::digest::Digest {
    fn from(value: TransactionId) -> Self {
        value.inner().into()
    }
}

impl From<&TransactionId> for proto::transaction::TransactionId {
    fn from(value: &TransactionId) -> Self {
        proto::transaction::TransactionId { id: Some(value.into()) }
    }
}

impl From<TransactionId> for proto::transaction::TransactionId {
    fn from(value: TransactionId) -> Self {
        (&value).into()
    }
}

// INTO TRANSACTION ID
// ================================================================================================

impl TryFrom<proto::digest::Digest> for TransactionId {
    type Error = ConversionError;

    fn try_from(value: proto::digest::Digest) -> Result<Self, Self::Error> {
        let digest: RpoDigest = value.try_into()?;
        Ok(digest.into())
    }
}

impl TryFrom<proto::transaction::TransactionId> for TransactionId {
    type Error = ConversionError;

    fn try_from(value: proto::transaction::TransactionId) -> Result<Self, Self::Error> {
        value
            .id
            .ok_or(ConversionError::MissingFieldInProtobufRepresentation {
                entity: "TransactionId",
                field_name: "id",
            })?
            .try_into()
    }
}
