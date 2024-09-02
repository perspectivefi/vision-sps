mod pb;
mod abi;
mod calls;

use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables;
use substreams_ethereum::pb::eth;
use substreams_ethereum::{Event};
use crate::abi::erc4626::events::Deposit;
use crate::calls::get_erc4626token;
use crate::pb::vision::{Erc4626Token, Erc4626Tokens};

#[substreams::handlers::map]
fn map_erc4626_tokens(block: eth::v2::Block) -> Result<Erc4626Tokens, substreams::errors::Error> {
    let tokens = block
        .transactions()
        .map(|transaction| {
            let mut tokens = Erc4626Tokens::default();

            for (log, _) in transaction.logs_with_calls() {
                match Deposit::match_and_decode(&log) {
                    Some(_deposit) => {
                        let erc4626_token: Option<Erc4626Token> = get_erc4626token(log.address.clone());

                        if erc4626_token.is_some() {
                            tokens.erc4626_tokens.push(erc4626_token.unwrap());
                        }
                    }

                    None => {}
                };
            }

            return tokens.erc4626_tokens;
        }).flatten().collect();

    Ok(Erc4626Tokens { erc4626_tokens: tokens })
}

#[substreams::handlers::map]
pub fn graph_out(tokens: Erc4626Tokens) -> Result<EntityChanges, substreams::errors::Error> {
    // hash map of name to a table
    let mut tables = Tables::new();

    for contract in tokens.erc4626_tokens.into_iter() {
        tables
            .create_row("ERC4626", contract.address)
            .set("decimals", contract.decimals)
            .set("symbol", contract.symbol)
            .set("convertToAssetsRate", contract.convert_to_assets_rate)
            .set("convertToSharesRate", contract.convert_to_shares_rate);
    }

    Ok(tables.to_entity_changes())
}
