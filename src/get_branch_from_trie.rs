use hex;
use crate::trie::Trie;
use crate::errors::AppError;
use crate::nibble_utils::Nibbles;
use crate::utils::convert_hex_to_u256;
use crate::rlp_codec::rlp_encode_transaction_index;
use crate::types::{
    Result,
    NodeStack,
};
use crate::nibble_utils::{
    get_nibbles_from_bytes,
};

fn convert_usize_index_to_trie_key(index: usize) -> Result<Nibbles> {
    convert_hex_to_u256(hex::encode(index.to_be_bytes()))
        .and_then(|u256| rlp_encode_transaction_index(&u256))
        .map(get_nibbles_from_bytes)
}

fn get_branch_from_trie(
    receipts_trie: Trie,
    index: usize,
) -> Result<NodeStack> {
    receipts_trie
        .find(convert_usize_index_to_trie_key(index)?)
        .and_then(|(_, _, found_stack, remaining_key)| {
            match remaining_key.len() {
                0 => Ok(found_stack),
                _ => Err(AppError::Custom(
                    format!("✘ Error! No receipt in trie at given index: {}", index)
                ))
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{
        SAMPLE_RECECIPT_TX_HASHES,
        get_sample_trie_with_sample_receipts,
    };

    #[test]
    fn should_convert_usize_to_trie_key() {
        let index = 10;
        let expected_result = Nibbles { data: vec![0x0a], offset: 0 };
        let result = convert_usize_index_to_trie_key(index)
            .unwrap();
        assert!(result == expected_result)
    }

    #[test]
    fn should_get_branch_from_trie() {
        let index = 14;
        let trie = get_sample_trie_with_sample_receipts();
        let result = get_branch_from_trie(trie, index)
            .unwrap();
    }

    #[test]
    fn should_fail_to_get_non_existent_branch_from_trie_correctly() {
        let non_existent_index = SAMPLE_RECECIPT_TX_HASHES.len() + 1;
        let expected_error = format!(
            "✘ Error! No receipt in trie at given index: {}",
            non_existent_index
        );
        let trie = get_sample_trie_with_sample_receipts();
        match get_branch_from_trie(trie, non_existent_index) {
            Err(AppError::Custom(e)) => assert!(e == expected_error),
            _ => panic!("Getting branch should not have succeeded!")
        }
    }
}