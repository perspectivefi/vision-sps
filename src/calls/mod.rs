use substreams::{Hex, scalar::BigInt};
use substreams_ethereum::rpc::RpcBatch;
use crate::abi::erc4626;
use crate::pb::vision::{Erc4626Token};

pub fn get_erc4626token(token_address: Vec<u8>) -> Option<Erc4626Token> {
    let batch = RpcBatch::new();

    let decimals = erc4626::functions::Decimals {}.call(token_address.clone()).unwrap_or(BigInt::from(0u64));

    let responses = batch
        .add(
            erc4626::functions::Symbol {},
            token_address.clone(),
        )
        .add(
            erc4626::functions::Asset {},
            token_address.clone(),
        )
        .add(
            erc4626::functions::TotalAssets {},
            token_address.clone(),
        )
        .add(
            erc4626::functions::TotalSupply {},
            token_address.clone(),
        )
        .add(
            erc4626::functions::ConvertToShares { assets: BigInt::from(10).pow(decimals.clone().into()) },
            token_address.clone(),
        )
        .add(
            erc4626::functions::ConvertToAssets { shares: BigInt::from(10).pow(decimals.clone().into()) },
            token_address.clone(),
        )
        .execute()
        .unwrap()
        .responses;

    let symbol: String;
    match RpcBatch::decode::<_, erc4626::functions::Symbol>(&responses[0]) {
        Some(decoded_symbol) => {
            symbol = decoded_symbol;
        }
        None => {
            symbol = String::default();
        }
    };

    let asset: String;
    match RpcBatch::decode::<_, erc4626::functions::Asset>(&responses[1]) {
        Some(decoded_asset) => {
            asset = format!("0x{}", Hex(&decoded_asset.clone()));
        }
        None => {
            asset = String::default();
        }
    };

    let total_assets: BigInt;
    match RpcBatch::decode::<_, erc4626::functions::TotalAssets>(&responses[2]) {
        Some(decoded_total_assets) => {
            total_assets = decoded_total_assets
        }
        None => {
            total_assets = BigInt::default();
        }
    };

    let total_supply: BigInt;
    match RpcBatch::decode::<_, erc4626::functions::TotalSupply>(&responses[3]) {
        Some(decoded_total_supply) => {
            total_supply = decoded_total_supply;
        }
        None => {
            total_supply = BigInt::default();
        }
    };


    let convert_to_shares_rate: String;
    match RpcBatch::decode::<_, erc4626::functions::ConvertToShares>(&responses[4]) {
        Some(decoded_convert_to_shares) => {
            convert_to_shares_rate = decoded_convert_to_shares.to_string();
        }
        None => {
            convert_to_shares_rate = String::default();
        }
    };

    let convert_to_assets_rate: String;
    match RpcBatch::decode::<_, erc4626::functions::ConvertToAssets>(&responses[5]) {
        Some(decoded_convert_to_assets) => {
            convert_to_assets_rate = decoded_convert_to_assets.to_string();
        }
        None => {
            convert_to_assets_rate = String::default();
        }
    };

    if !symbol.is_empty()
        && !decimals.is_zero()
        && !asset.is_empty()
        && !total_assets.is_zero()
        && !total_supply.is_zero()
        && !convert_to_shares_rate.is_empty()
        && !convert_to_assets_rate.is_empty() {
        Some(Erc4626Token {
            address: format!("0x{}", Hex(&token_address.clone())),
            symbol,
            decimals: decimals.to_u64(),
            asset,
            total_assets: total_assets.to_string(),
            total_supply: total_supply.to_string(),
            convert_to_shares_rate: convert_to_shares_rate.clone(),
            convert_to_assets_rate: convert_to_assets_rate.clone(),
        })
    } else {
        None
    }
}

