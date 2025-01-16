mod pb;
use pb::contract::v1::{self as contract, BlockContracts, Contract};

use substreams::Hex;
use substreams_ethereum::pb::eth::v2::{Block, CallType};

#[allow(unused_imports)]
use num_traits::cast::ToPrimitive;

substreams_ethereum::init!();

#[substreams::handlers::map]
fn map_my_data(blk: Block) -> contract::BlockContracts {
    let block_number = blk.number;
    let contracts = blk
        .transaction_traces
        .into_iter()
        .flat_map(|trace| {
            trace
                .calls
                .into_iter()
                .filter(|c| matches!(c.call_type(), CallType::Create))
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
