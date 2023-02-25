use std::fmt::Debug;

use mentat_server::{
    serde::{de::DeserializeOwned, Deserialize},
    tracing,
};
use mentat_types::Result;

use crate::ErrorResponse;

#[derive(Clone, Debug, Deserialize)]
#[serde(crate = "mentat_server::serde")]
pub struct InnerResponse<I> {
    // jsonrpc: String,
    pub result: I,
    // id: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(crate = "mentat_server::serde")]
#[serde(untagged)]
pub enum Response<O> {
    Ok(InnerResponse<O>),
    Err(ErrorResponse),
}

impl<O> Response<O>
where
    O: Debug + DeserializeOwned,
{
    pub fn unwrap_response(self) -> Result<O> {
        tracing::debug!("res: {self:#?}");
        match self {
            Response::Ok(res) => Ok(res.result),
            Response::Err(err) => err.into(),
        }
    }
}
