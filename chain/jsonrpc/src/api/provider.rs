use serde_json::Value;

use near_client_primitives::types::{GetProviderError, GetProviderInfoError};
use near_jsonrpc_primitives::errors::RpcParseError;
use near_jsonrpc_primitives::types::provider::{
    RpcProviderError, RpcProviderRequest,
};

use super::{RpcFrom, RpcRequest};

impl RpcRequest for RpcProviderRequest {
    fn parse(value: Value) -> Result<Self, RpcParseError> {
        let block_height = value
                .get("block_height")
                .and_then(|v| v.as_u64())
                .ok_or_else(|| RpcParseError("block_height not found or not a u64".parse().unwrap()))?;
        Ok(Self { block_height })
    }
    // fn parse(value: Value) -> Result<Self, RpcParseError> {
    //     // Extract block_hash_str from value
    //     let block_hash_str = value
    //         .get("block_hash")
    //         .and_then(|v| v.as_str())
    //         .ok_or_else(|| RpcParseError("block_hash not found or not a string".parse().unwrap()))?;
    //
    //     // Decode the base58-encoded string to bytes
    //     let bytes = bs58::decode(block_hash_str)
    //         .into_vec()
    //         .map_err(|_| RpcParseError("Invalid base58-encoded hash".parse().unwrap()))?;
    //
    //     // Ensure the decoded bytes have the correct length for a CryptoHash
    //     if bytes.len() != 32 {
    //         return Err(RpcParseError("Decoded hash does not match expected length".parse().unwrap()));
    //     }
    //
    //     // Construct the CryptoHash from the decoded bytes
    //     let block_hash = CryptoHash::try_from(bytes.as_slice())
    //         .map_err(|_| RpcParseError("Failed to convert bytes to CryptoHash".parse().unwrap()))?;
    //
    //     Ok(Self { block_hash })
    // }

}

impl RpcFrom<actix::MailboxError> for RpcProviderError {
    fn rpc_from(error: actix::MailboxError) -> Self {
        Self::InternalError { error_message: error.to_string() }
    }
}

impl RpcFrom<GetProviderInfoError> for RpcProviderError {
    fn rpc_from(error: GetProviderInfoError) -> Self {
        match error {
            GetProviderInfoError::UnknownBlock => Self::UnknownBlock,
            GetProviderInfoError::ProviderInfoUnavailable => Self::ProviderInfoUnavailable,
            GetProviderInfoError::IOError(error_message) => Self::InternalError { error_message },
            GetProviderInfoError::Unreachable(ref error_message) => {
                tracing::warn!(target: "jsonrpc", "Unreachable error occurred: {}", error_message);
                crate::metrics::RPC_UNREACHABLE_ERROR_COUNT
                    .with_label_values(&["RpcProviderError"])
                    .inc();
                Self::InternalError { error_message: error.to_string() }
            }
        }
    }
}

impl RpcFrom<GetProviderError> for RpcProviderError {
    fn rpc_from(error: GetProviderError) -> Self {
        match error {
            GetProviderError::UnknownBlock{.. } => Self::UnknownBlock{},
            GetProviderError::NotSyncedYet{.. }  => Self::ProviderInfoUnavailable,
            GetProviderError::IOError{error_message} => Self::InternalError { error_message },
            GetProviderError::Unreachable{ref error_message} => {
                tracing::warn!(target: "jsonrpc", "Unreachable error occurred: {}", error_message);
                crate::metrics::RPC_UNREACHABLE_ERROR_COUNT
                    .with_label_values(&["RpcProviderError"])
                    .inc();
                Self::InternalError { error_message: error.to_string() }
            }
        }
    }
}
