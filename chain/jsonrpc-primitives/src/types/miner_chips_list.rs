use serde_json::Value;
use unc_primitives::types::{AccountId};
use unc_primitives::views::ChipView;

#[derive(thiserror::Error, Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "name", content = "info", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RpcMinerChipsListError {
    #[error("Account not found")]
    UnknownAccount { error_message: String },
    #[error("Chips info unavailable")]
    ChipsInfoUnavailable,
    #[error("The node reached its limits. Try again later. More details: {error_message}")]
    InternalError { error_message: String },
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq)]
pub struct RpcMinerChipsListRequest {
    pub account_id: AccountId,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct RpcMinerChipsListResponse {
    pub account_id: AccountId,
    pub chips: Vec<ChipView>,
}

impl From<RpcMinerChipsListError> for crate::errors::RpcError {
    fn from(error: RpcMinerChipsListError) -> Self {
        let error_data = match &error {
            RpcMinerChipsListError::UnknownAccount{ error_message } => Some(Value::String(error_message.to_string())),
            RpcMinerChipsListError::ChipsInfoUnavailable => {
                Some(Value::String("Chips info unavailable".to_string()))
            }
            RpcMinerChipsListError::InternalError { .. } => Some(Value::String(error.to_string())),
        };

        let error_data_value = match serde_json::to_value(error) {
            Ok(value) => value,
            Err(err) => {
                return Self::new_internal_error(
                    None,
                    format!("Failed to serialize RpcValidatorError: {:?}", err),
                )
            }
        };

        Self::new_internal_or_handler_error(error_data, error_data_value)
    }
}
