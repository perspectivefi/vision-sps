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

    let convert_to_assets_rate: BigInt;
    match RpcBatch::decode::<_, erc4626::functions::ConvertToAssets>(&responses[1]) {
        Some(decoded_convert_to_assets) => {
            convert_to_assets_rate = decoded_convert_to_assets;
        }
        None => {
            convert_to_assets_rate = BigInt::default();
        }
    };

    if !convert_to_assets_rate.is_zero() && !symbol.is_empty() {
        Some(Erc4626Token {
            address: format!("0x{}", Hex(&token_address.clone())),
            symbol,
            decimals: decimals.to_u64(),
            convert_to_assets_rate: convert_to_assets_rate.to_u64().clone(),
            convert_to_shares_rate: 0,
        })
    } else {
        None
    }
}

