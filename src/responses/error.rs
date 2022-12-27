// {"jsonrpc":"2.0","error":{"code":-32000,"message":"Odd number of
// digits"},"id":"1"}

use mentat_server::{indexmap::IndexMap, serde::Deserialize, serde_json::Value};
use mentat_types::{MentatError, Result};

#[derive(Clone, Debug, Deserialize)]
#[serde(crate = "mentat_server::serde")]
pub struct ErrorResponse {
    pub jsonrpc: String,
    pub error: IndexMap<String, Value>,
    pub id: String,
}

impl<R> From<ErrorResponse> for Result<R> {
    fn from(response: ErrorResponse) -> Self {
        Err(MentatError {
            code: 500,
            status_code: 500,
            message: "Snarkos JsonRPC Error.".to_string(),
            description: None,
            retriable: true,
            details: response.error,
        })
    }
}
