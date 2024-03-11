use unc_primitives::types::{AccountId};
use serde_json::Value;

use unc_client_primitives::types::{MinerChipsListError};
use unc_jsonrpc_primitives::errors::RpcParseError;
use unc_jsonrpc_primitives::types::miner_chips_list::{
    RpcMinerChipsListError, RpcMinerChipsListRequest,
};

use super::{RpcFrom, RpcRequest};

impl RpcRequest for RpcMinerChipsListRequest {
    // fn parse(value: Value) -> Result<Self, RpcParseError> {
    //     let block_height = value
    //             .get("block_height")
    //             .and_then(|v| v.as_u64())
    //             .ok_or_else(|| RpcParseError("block_height not found or not a u64".parse().unwrap()))?;
    //     Ok(Self { block_height })
    // }
    fn parse(value: Value) -> Result<Self, RpcParseError> {
        // Extract block_hash_str from value
        let account_id_str = value
            .get("account_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| RpcParseError("account_id not found or not a string".parse().unwrap()))?;

        // Construct the CryptoHash from the decoded bytes
        let account_id : AccountId = account_id_str
            .parse()
            .map_err(|_| RpcParseError("Failed to parse epoch_id from base58".parse().unwrap()))?;

        Ok(Self { account_id })
    }

}

impl RpcFrom<actix::MailboxError> for RpcMinerChipsListError {
    fn rpc_from(error: actix::MailboxError) -> Self {
        Self::InternalError { error_message: error.to_string() }
    }
}

impl RpcFrom<MinerChipsListError> for RpcMinerChipsListError {
    fn rpc_from(error: MinerChipsListError) -> Self {
        match error {
            MinerChipsListError::UnknownAccount{ error_message }  => Self::UnknownAccount { error_message },
            MinerChipsListError::ChipsInfoUnavailable => Self::ChipsInfoUnavailable,
            MinerChipsListError::IOError{ error_message } => Self::InternalError { error_message },
            MinerChipsListError::Unreachable{ ref error_message } => {
                tracing::warn!(target: "jsonrpc", "Unreachable error occurred: {}", error_message);
                crate::metrics::RPC_UNREACHABLE_ERROR_COUNT
                    .with_label_values(&["MinerChipsListError"])
                    .inc();
                Self::InternalError { error_message: error.to_string() }
            }
        }
    }
}
