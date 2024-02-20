use serde_json::Value;

use near_client_primitives::types::{GetProviderError, GetProviderInfoError};
use near_jsonrpc_primitives::errors::RpcParseError;
use near_jsonrpc_primitives::types::provider::{
    RpcProviderError, RpcProviderRequest,
};

use super::{Params, RpcFrom, RpcRequest};

impl RpcRequest for RpcProviderRequest {
    fn parse(value: Value) -> Result<Self, RpcParseError> {
        OK(Self { value } )
    }

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
