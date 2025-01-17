mod pb;
use pb::contract::v1::{self as contract, BlockContracts, Contract};

use substreams::Hex;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables;
use substreams_ethereum::pb::eth::v2::{Block, CallType};

#[allow(unused_imports)]
use num_traits::cast::ToPrimitive;

substreams_ethereum::init!();

#[substreams::handlers::map]
fn map_contract_index(blk: Block) -> contract::BlockContracts {
    let block_number = blk.number;
    let contracts = blk
        .transaction_traces
        .into_iter()
        .flat_map(|trace| {
            trace
                .calls
                .into_iter()
                .filter(|c| matches!(c.call_type(), CallType::Create) && !c.status_failed)
                .map(move |c| Contract {
                    tx_hash: Hex::encode(trace.hash.clone()),
                    block_number,
                    address: Hex::encode(c.address),
                    parent: Hex::encode(trace.to.clone()),
                })
        })
        .collect();

    BlockContracts { contracts }
}

#[substreams::handlers::map]
pub fn graph_out(contracts: BlockContracts) -> Result<EntityChanges, substreams::errors::Error> {
    // hash map of name to a table
    let mut tables = Tables::new();

    for Contract {
        tx_hash,
        block_number,
        address,
        parent,
    } in contracts.contracts.into_iter()
    {
        tables
            .create_row("Contract", address)
            .set("txHash", tx_hash)
            .set("parent", parent)
            .set("blockNumber", block_number);
    }

    Ok(tables.to_entity_changes())
}
