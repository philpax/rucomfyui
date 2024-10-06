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

#[derive(Error, Debug, Deserialize)]
/// ComfyUI failed to validate the request.
#[serde(tag = "type")]
pub enum ValidationError {
    /// An error occurred, but this library does not know how to handle it.
    #[error("unknown error: {type_}: {message}: {details}")]
    #[serde(untagged)]
    Unknown {
        /// The type of error.
        #[serde(rename = "type")]
        type_: String,
        /// The error message.
        message: String,
        /// Additional details.
        details: String,
        /// Extra info.
        extra_info: serde_json::Value,
    },
}
impl ValidationError {
    pub(crate) fn from_value(value: &serde_json::Value) -> Option<Self> {
        serde_json::from_value(value.clone()).ok()
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
            .filter_map(|e| ValidationError::from_value(e))
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
        if let Some(error) = object
            .get("error")
            .and_then(|e| ValidationError::from_value(e))
        {
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
