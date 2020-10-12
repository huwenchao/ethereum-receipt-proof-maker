#![feature(try_trait)]
// #![feature(const_vec_new)]
#![feature(exclusive_range_pattern)]

mod trie;
mod utils;
mod state;
mod types;
mod errors;
mod get_log;
mod rlp_codec;
mod constants;
mod get_block;
mod trie_nodes;
mod usage_info;
mod test_utils;
mod path_codec;
mod parse_cli_args;
mod get_receipts;
mod nibble_utils;
mod get_database;
mod get_tx_index;
mod get_endpoint;
mod make_rpc_call;
mod get_keccak_hash;
mod connect_to_node;
mod validate_tx_hash;
mod validate_cli_args;
mod get_receipts_trie;
mod get_rpc_call_jsons;
mod get_branch_from_trie;
mod get_hex_proof_from_branch;
mod initialize_state_from_cli_args;

#[macro_use] extern crate log;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;
#[cfg(test)] #[macro_use] extern crate serial_test_derive;

use crate::get_tx_index::get_tx_index_and_add_to_state;
use crate::get_receipts_trie::get_receipts_trie_and_set_in_state;
use crate::get_block::get_block_from_tx_hash_in_state_and_set_in_state;
use crate::get_branch_from_trie::get_branch_from_trie_and_put_in_state;
use crate::get_hex_proof_from_branch::get_hex_proof_from_branch_in_state;
use crate::get_receipts::get_all_receipts_from_block_in_state_and_set_in_state;
use crate::state::State;
use crate::utils::convert_hex_to_h256;

pub fn get_hex_proof(tx_hash: String) -> Result<types::HexProof, errors::AppError>{
    State::init(
        convert_hex_to_h256(tx_hash.clone())?,
        tx_hash,
    )
        .and_then(get_block_from_tx_hash_in_state_and_set_in_state)
        .and_then(get_all_receipts_from_block_in_state_and_set_in_state)
        .and_then(get_tx_index_and_add_to_state)
        .and_then(get_receipts_trie_and_set_in_state)
        .and_then(get_branch_from_trie_and_put_in_state)
        .and_then(get_hex_proof_from_branch_in_state)


}

#[test]
fn test_get_hex_proof() {
    let tx_hash = "0xb540248a9cca048c5861dec953d7a776bc1944319b9bd27a462469c8a437f4ff";
    let proof = get_hex_proof(String::from(tx_hash));
    match proof {
        Ok(proof)=>{println!("{:?}", proof.clone());},
        Err(err) =>{println!("{:?}", err);}
    }
    // assert_eq!(proof.unwrap(), "f901b2f901af822080b901a9f901a60182d0d9b9010000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000010000000000000000000000000000000000000000000000000000000408000000000000000000000000000000000000000000000000000000000000000000000001000000000000000000000010000000000000000000000000000000000000000000000000000000400000000000100000000000000000000000000080000000000000000000000000000000000000000000100002000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002000000000000000000000000f89df89b94dac17f958d2ee523a2206206994597c13d831ec7f863a0ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3efa00000000000000000000000006cc5f688a315f3dc28a7781717a9a798a59fda7ba00000000000000000000000007e7a32d9dc98c485c489be8e732f97b4ffe3a4cda000000000000000000000000000000000000000000000000000000001a13b8600");
}
