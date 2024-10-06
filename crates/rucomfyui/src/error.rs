//! All error types exposed by this library.

use std::{collections::HashMap, str::FromStr};

use serde::Deserialize;
use thiserror::Error;

use crate::workflow::WorkflowNodeId;

#[derive(Error, Debug)]
/// Errors that can occur when using the client.
pub enum ClientError {
    #[error("reqwest error: {0}")]
    /// Reqwest error.
    Reqwest(#[from] reqwest::Error),

    #[error("parse error: {0}")]
    /// Parse int error.
    ParseInt(#[from] std::num::ParseIntError),

    #[error("JSON error: {0}")]
    /// Serde JSON error.
    Json(#[from] serde_json::Error),

    #[error("validation error: {error}, node errors: {node_errors:?}")]
    /// Validation error.
    Validation {
        /// The main error.
        error: ValidationError,

        /// Any node errors that may have occurred.
        node_errors: Option<HashMap<WorkflowNodeId, NodeError>>,
    },
}

#[derive(Debug, Deserialize)]
/// An error occurred, but this library does not know how to handle it.
pub struct UnknownValidationError {
    /// The type of error.
    #[serde(rename = "type")]
    pub type_: String,
    /// The error message.
    pub message: String,
    /// Additional details.
    pub details: String,
    /// Extra info.
    pub extra_info: serde_json::Value,
}
impl std::fmt::Display for UnknownValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {} (details: {})",
            self.type_, self.message, self.details
        )
    }
}

#[derive(Error, Debug)]
/// ComfyUI failed to validate the request.
pub enum ValidationError {
    /// A required input was missing.
    #[error("required input missing: {input_name}")]
    RequiredInputMissing {
        /// The original message.
        message: String,
        /// The name of the missing input.
        input_name: String,
    },

    /// An input was incorrectly specified; it must be a length-2 list of `[node_id, slot_index]`.
    #[error("bad linked input: {input_name}")]
    BadLinkedInput {
        /// The original message.
        message: String,
        /// The name of the input.
        input_name: String,
        /// The configuration of the input.
        input_config: serde_json::Value,
        /// The received value.
        received_value: serde_json::Value,
    },

    /// The return type of a linked node doesn't match the expected input type.
    #[error("return type mismatch: {input_name}, expected {expected_type}, got {received_type}")]
    ReturnTypeMismatch {
        /// The original message.
        message: String,
        /// The name of the input.
        input_name: String,
        /// The configuration of the input.
        input_config: serde_json::Value,
        /// The type that was received.
        received_type: String,
        /// The type that was expected.
        expected_type: String,
        /// The value from the linked node.
        linked_node: Vec<serde_json::Value>,
    },

    /// An exception occurred during the validation of an inner node.
    #[error("exception during inner validation: {input_name}")]
    ExceptionDuringInnerValidation {
        /// The original message.
        message: String,
        /// The name of the input.
        input_name: String,
        /// The configuration of the input.
        input_config: serde_json::Value,
        /// The exception message.
        exception_message: String,
        /// The type of the exception.
        exception_type: String,
        /// The traceback of the exception.
        traceback: Vec<String>,
        /// The value from the linked node.
        linked_node: Vec<serde_json::Value>,
    },

    /// Failed to convert an input value to the expected type.
    #[error("invalid input type: {input_name}")]
    InvalidInputType {
        /// The original message.
        message: String,
        /// The name of the input.
        input_name: String,
        /// The configuration of the input.
        input_config: serde_json::Value,
        /// The received value.
        received_value: serde_json::Value,
        /// The exception message.
        exception_message: String,
    },

    /// The input value is smaller than the minimum allowed value.
    #[error("value smaller than min: {input_name}")]
    ValueSmallerThanMin {
        /// The original message.
        message: String,
        /// The name of the input.
        input_name: String,
        /// The configuration of the input.
        input_config: serde_json::Value,
        /// The received value.
        received_value: serde_json::Value,
    },

    /// The input value is bigger than the maximum allowed value.
    #[error("value bigger than max: {input_name}")]
    ValueBiggerThanMax {
        /// The original message.
        message: String,
        /// The name of the input.
        input_name: String,
        /// The configuration of the input.
        input_config: serde_json::Value,
        /// The received value.
        received_value: serde_json::Value,
    },

    /// The input value is not in the list of allowed values.
    #[error("value not in list: {input_name}")]
    ValueNotInList {
        /// The original message.
        message: String,
        /// The name of the input.
        input_name: String,
        /// The configuration of the input.
        input_config: Option<serde_json::Value>,
        /// The received value.
        received_value: serde_json::Value,
    },

    /// Custom validation defined by the node failed.
    #[error("custom validation failed: {input_name}")]
    CustomValidationFailed {
        /// The original message.
        message: String,
        /// The name of the input.
        input_name: String,
        /// Additional details about the validation failure.
        details: String,
    },

    /// The prompt has no outputs.
    #[error("prompt has no outputs")]
    PromptNoOutputs,

    /// At least one prompt output failed validation.
    #[error("at least one prompt output failed validation")]
    PromptOutputsFailedValidation {
        /// Details about the validation failures.
        details: String,
    },

    #[error("unknown error: {0}")]
    /// An error occurred, but this library does not know how to handle it.
    Unknown(UnknownValidationError),
}

impl ValidationError {
    pub(crate) fn from_value(value: &serde_json::Value) -> Option<Self> {
        let unknown_error: UnknownValidationError = serde_json::from_value(value.clone()).ok()?;

        let info = &unknown_error.extra_info;
        match unknown_error.type_.as_str() {
            "required_input_missing" => Some(ValidationError::RequiredInputMissing {
                message: unknown_error.message,
                input_name: info.get("input_name")?.as_str()?.to_string(),
            }),
            "bad_linked_input" => Some(ValidationError::BadLinkedInput {
                message: unknown_error.message,
                input_name: info.get("input_name")?.as_str()?.to_string(),
                input_config: info.get("input_config")?.clone(),
                received_value: info.get("received_value")?.clone(),
            }),
            "return_type_mismatch" => Some(ValidationError::ReturnTypeMismatch {
                message: unknown_error.message,
                input_name: info.get("input_name")?.as_str()?.to_string(),
                input_config: info.get("input_config")?.clone(),
                received_type: info.get("received_type")?.as_str()?.to_string(),
                expected_type: info.get("input_config")?[0].as_str()?.to_string(),
                linked_node: info.get("linked_node")?.as_array()?.clone(),
            }),
            "exception_during_inner_validation" => {
                Some(ValidationError::ExceptionDuringInnerValidation {
                    message: unknown_error.message,
                    input_name: info.get("input_name")?.as_str()?.to_string(),
                    input_config: info.get("input_config")?.clone(),
                    exception_message: info.get("exception_message")?.as_str()?.to_string(),
                    exception_type: info.get("exception_type")?.as_str()?.to_string(),
                    traceback: info
                        .get("traceback")?
                        .as_array()?
                        .iter()
                        .map(|v| v.as_str().unwrap().to_string())
                        .collect(),
                    linked_node: info.get("linked_node")?.as_array()?.clone(),
                })
            }
            "invalid_input_type" => Some(ValidationError::InvalidInputType {
                message: unknown_error.message,
                input_name: info.get("input_name")?.as_str()?.to_string(),
                input_config: info.get("input_config")?.clone(),
                received_value: info.get("received_value")?.clone(),
                exception_message: info.get("exception_message")?.as_str()?.to_string(),
            }),
            "value_smaller_than_min" => Some(ValidationError::ValueSmallerThanMin {
                message: unknown_error.message,
                input_name: info.get("input_name")?.as_str()?.to_string(),
                input_config: info.get("input_config")?.clone(),
                received_value: info.get("received_value")?.clone(),
            }),
            "value_bigger_than_max" => Some(ValidationError::ValueBiggerThanMax {
                message: unknown_error.message,
                input_name: info.get("input_name")?.as_str()?.to_string(),
                input_config: info.get("input_config")?.clone(),
                received_value: info.get("received_value")?.clone(),
            }),
            "value_not_in_list" => Some(ValidationError::ValueNotInList {
                message: unknown_error.message,
                input_name: info.get("input_name")?.as_str()?.to_string(),
                input_config: info.get("input_config").cloned(),
                received_value: info.get("received_value")?.clone(),
            }),
            "custom_validation_failed" => Some(ValidationError::CustomValidationFailed {
                message: unknown_error.message,
                input_name: info.get("input_name")?.as_str()?.to_string(),
                details: unknown_error.details,
            }),
            "prompt_no_outputs" => Some(ValidationError::PromptNoOutputs),
            "prompt_outputs_failed_validation" => {
                Some(ValidationError::PromptOutputsFailedValidation {
                    details: unknown_error.details,
                })
            }
            _ => Some(ValidationError::Unknown(unknown_error)),
        }
    }
}

#[derive(Debug)]
/// Errors that can occur when validating a node.
pub struct NodeError {
    /// The errors that occurred.
    pub errors: Vec<ValidationError>,
    /// The nodes that this node depends on.
    pub dependent_outputs: Vec<WorkflowNodeId>,
    /// The type of this node.
    pub class_type: String,
}
impl NodeError {
    pub(crate) fn from_value(value: &serde_json::Value) -> Option<Self> {
        let error = value.as_object()?;
        let errors = error
            .get("errors")?
            .as_array()?
            .iter()
            .filter_map(ValidationError::from_value)
            .collect();
        let dependent_outputs = error
            .get("dependent_outputs")?
            .as_array()?
            .iter()
            .filter_map(|v| v.as_str())
            .filter_map(|s| WorkflowNodeId::from_str(s).ok())
            .collect();
        let class_type = error.get("class_type")?.as_str()?.to_string();
        Some(Self {
            errors,
            dependent_outputs,
            class_type,
        })
    }
}

pub(crate) async fn parse_response<T: serde::de::DeserializeOwned>(
    response: reqwest::Response,
) -> Result<T, ClientError> {
    let value: serde_json::Value = response.json().await?;
    if let Some(object) = value.as_object() {
        if let Some(error) = object.get("error").and_then(ValidationError::from_value) {
            let node_errors = object
                .get("node_errors")
                .and_then(|v| v.as_object())
                .map(|o| {
                    o.iter()
                        .filter_map(|(k, v)| {
                            Some((k.parse::<WorkflowNodeId>().ok()?, NodeError::from_value(v)?))
                        })
                        .collect()
                });

            return Err(ClientError::Validation { error, node_errors });
        }
    }
    Ok(serde_json::from_value(value)?)
}
