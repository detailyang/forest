// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

use super::client::Filecoin;
use crypto::signature::json::signature_type::SignatureTypeJson;
use jsonrpc_v2::Error as JsonRpcError;
use jsonrpsee::{raw::RawClient, transport::http::HttpTransportClient};
use wallet::json::KeyInfoJson;

pub async fn wallet_new(
    client: &mut RawClient<HttpTransportClient>,
    signature_type: SignatureTypeJson,
) -> Result<String, JsonRpcError> {
    Ok(Filecoin::wallet_new(client, signature_type).await?)
}

pub async fn wallet_default_address(
    client: &mut RawClient<HttpTransportClient>,
) -> Result<String, JsonRpcError> {
    Ok(Filecoin::wallet_has(client).await?)
}

pub async fn wallet_balance(
    client: &mut RawClient<HttpTransportClient>,
) -> Result<String, JsonRpcError> {
    Ok(Filecoin::wallet_balance(client).await?)
}

pub async fn wallet_export(
    client: &mut RawClient<HttpTransportClient>,
) -> Result<KeyInfoJson, JsonRpcError> {
    Ok(Filecoin::wallet_export(client).await?)
}

pub async fn wallet_list(
    client: &mut RawClient<HttpTransportClient>,
) -> Result<Vec<String>, JsonRpcError> {
    let _result = Filecoin::wallet_list(client).await?;
    Ok(vec![])
}
