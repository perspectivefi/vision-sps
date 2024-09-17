mod pb;
mod abi;
mod calls;
mod constants;
mod utils;

use substreams::store::{StoreGet, StoreGetProto, StoreNew, StoreSetProto, StoreSet};
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables;
use substreams_ethereum::pb::eth;
use substreams_ethereum::{Event};
use crate::abi::erc4626::events::Deposit;
use crate::calls::get_erc4626token;
use crate::pb::vision::{Erc4626Token, Erc4626Tokens, Rate, Rates};

use crate::constants::ROUNDED_ONE_DAY_IN_SECONDS;
use crate::utils::{calculate_apr, format_key, get_day_timestamp};

#[substreams::handlers::map]
fn map_erc4626_tokens(block: eth::v2::Block) -> Result<Erc4626Tokens, substreams::errors::Error> {
    let tokens = block
        .transactions()
        .map(|transaction| {
            let mut tokens = Erc4626Tokens::default();

            for (log, _) in transaction.logs_with_calls() {
                match Deposit::match_and_decode(&log) {
                    Some(_deposit) => {
                        let erc4626_token: Option<Erc4626Token> = get_erc4626token(
                            log.address.clone(),
                            log.ordinal.clone(),
                        );

                        if erc4626_token.is_some() {
                            tokens.erc4626_tokens.push(erc4626_token.unwrap());
                        }
                    }

                    None => {}
                };
            }

            return tokens.erc4626_tokens;
        })
        .flatten()
        .collect();
    Ok(Erc4626Tokens { erc4626_tokens: tokens })
}

#[substreams::handlers::store]
fn store_rates_for_last_7_days(
    tokens: Erc4626Tokens,
    block: eth::v2::Block,
    store: StoreSetProto<String>,
) {
    tokens.erc4626_tokens.into_iter().for_each(|erc4626| {
        let timestamp: u64 = block.timestamp_seconds();
        let daily_standardized_timestamp: u64 = get_day_timestamp(timestamp);

        let key = format_key(erc4626.address, daily_standardized_timestamp);

        store.set(erc4626.ordinal, key, &erc4626.convert_to_assets_rate.to_string());
    });
}

#[substreams::handlers::map]
fn map_rates(
    tokens: Erc4626Tokens,
    block: eth::v2::Block,
    store: StoreGetProto<String>,
) -> Result<Rates, substreams::errors::Error> {
    let mut rates: Vec<Rate> = Vec::new();

    tokens.erc4626_tokens.into_iter().for_each(|erc4626| {
        // Find the most recent rate within the last 7 days
        let previous_rate = (1..=7).rev().find_map(|days_ago| {
            let past_key = format_key(erc4626.address.clone(), get_day_timestamp(block.timestamp_seconds() - (days_ago * ROUNDED_ONE_DAY_IN_SECONDS)));
            store.get_last(past_key)
        });

        let apr = previous_rate.unwrap_or_else(|| "0".to_string());

        rates.push(Rate {
            id: format!("{}-{}", erc4626.address, block.number),
            timestamp: block.timestamp_seconds(),
            block_number: block.number,
            erc4626: erc4626.address,
            convert_to_assets_rate: erc4626.convert_to_assets_rate.clone(),
            apr: calculate_apr(apr, erc4626.convert_to_assets_rate.clone(), ROUNDED_ONE_DAY_IN_SECONDS),
        });
    });

    Ok(Rates { rates })
}

#[substreams::handlers::map]
pub fn graph_out(
    tokens: Erc4626Tokens,
    rates: Rates,
) -> Result<EntityChanges, substreams::errors::Error> {
    let mut tables = Tables::new();

    for contract in tokens.erc4626_tokens.into_iter() {
        tables
            .create_row("ERC4626", contract.address)
            .set("decimals", contract.decimals)
            .set("symbol", contract.symbol)
            .set("asset", contract.asset)
            .set("totalAssets", contract.total_assets)
            .set("totalSupply", contract.total_supply);
    }

    for rate in rates.rates.into_iter() {
        tables
            .create_row("Rate", rate.id)
            .set("timestamp", rate.timestamp)
            .set("blockNumber", rate.block_number)
            .set("erc4626", rate.erc4626)
            .set("convertToAssetsRate", rate.convert_to_assets_rate)
            .set("apr", rate.apr);
    }

    Ok(tables.to_entity_changes())
}
