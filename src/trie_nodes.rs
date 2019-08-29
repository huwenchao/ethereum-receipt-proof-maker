use rlp::RlpStream;
use ethereum_types::H256;
use crate::errors::AppError;
use crate::get_keccak_hash::keccak_hash_bytes;
use crate::path_codec::{
    encode_leaf_path_from_nibbles,
    encode_extension_path_from_nibbles,
};
use crate::nibble_utils::{
    Nibbles,
    get_nibbles_from_bytes
};
use crate::types::{
    Bytes,
    Result,
};

static NO_NODE_IN_STRUCT_ERR: &'static str = "✘ No node present in struct to rlp-encode!";

#[derive(Debug, PartialEq, Eq)]
pub struct Node {
    pub leaf: Option<LeafNode>,
    pub extension: Option<ExtensionNode>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct LeafNode {
    pub raw: Bytes,
    pub value: Bytes,
    pub encoded_path: Bytes,
    pub path_nibbles: Nibbles,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ExtensionNode {
    pub raw: Bytes,
    pub value: Bytes,
    pub encoded_path: Bytes,
    pub path_nibbles: Nibbles,
}

impl Node {
    pub fn new_leaf(path_nibbles: Nibbles, value: Bytes) -> Result<Node> {
        let encoded_path = encode_leaf_path_from_nibbles(path_nibbles.clone())?;
        let mut raw = encoded_path.clone();
        raw.append(&mut value.clone());
        Ok(
            Node {
                extension: None,
                leaf: Some(
                    LeafNode {
                        raw,
                        value,
                        path_nibbles,
                        encoded_path,
                    }
                )
            }
        )
    }

    pub fn rlp_encode(&self) -> Result<Bytes> {
        if let Some(leaf) = &self.leaf {
            let mut rlp_stream = RlpStream::new();
            rlp_stream.begin_list(2);
            rlp_stream.append(&leaf.encoded_path);
            rlp_stream.append(&leaf.value);
            Ok(rlp_stream.out())
        } else if let Some(extension) = &self.extension {
            let mut rlp_stream = RlpStream::new();
            rlp_stream.begin_list(2);
            rlp_stream.append(&extension.encoded_path);
            rlp_stream.append(&extension.value);
            Ok(rlp_stream.out())
        } else {
            Err(AppError::Custom(NO_NODE_IN_STRUCT_ERR.to_string()))
        }
    }

    pub fn hash(&self) -> Result<H256> {
        self.rlp_encode()
            .and_then(|encoded| keccak_hash_bytes(&encoded))
    }
}

#[cfg(test)]
mod tests {
    use hex;
    use super::*;
    use crate::nibble_utils::{
        Nibbles,
        get_length_in_nibbles,
    };
    use crate::utils::convert_hex_to_h256;

    fn get_sample_leaf_node() -> Node {
        let path_bytes = vec![0x12, 0x34, 0x56];
        let path_nibbles = get_nibbles_from_bytes(path_bytes.clone());
        let value = hex::decode("c0ffee".to_string()).unwrap();
        Node::new_leaf(path_nibbles, value)
            .unwrap()
    }

    fn get_sample_leaf_node_expected_encoding() -> Bytes {
        hex::decode("c9842012345683c0ffee")
            .unwrap()
    }

    fn get_sample_leaf_node_expected_hash() -> H256 {
        let hex = "c9161ce49c6a3362f5d20db4b6e36c259c9624eac5f99e64a052f45035d14c5d"
            .to_string();
        convert_hex_to_h256(hex)
            .unwrap()
    }


    #[test]
    fn should_get_new_leaf_node_correctly() {
        let path_bytes = vec![0x12, 0x34, 0x56];
        let expected_nibble_length = path_bytes.clone().len() * 2;
        let path_nibbles = get_nibbles_from_bytes(path_bytes.clone());
        let value = hex::decode("c0ffee".to_string()).unwrap();
        let expected_encoded_path = encode_leaf_path_from_nibbles(path_nibbles.clone())
            .unwrap();
        let mut expected_raw = expected_encoded_path.clone();
        expected_raw.append(&mut value.clone());
        let result = Node::new_leaf(path_nibbles.clone(), value.clone())
            .unwrap();
        if let Some(_) = result.extension {
            panic!("Node should be a leaf node!")
        }
        match result.leaf {
            None => panic!("Leaf node should be present in node!"),
            Some(leaf) => {
                let nibble_length = get_length_in_nibbles(&leaf.path_nibbles.clone());
                assert!(leaf.value == value);
                assert!(leaf.raw == expected_raw);
                assert!(leaf.path_nibbles == path_nibbles);
                assert!(leaf.encoded_path == expected_encoded_path);
                assert!(nibble_length== expected_nibble_length)
            }
        }
    }

    #[test]
    fn should_rlp_encode_leaf_node_correctly() {
        let leaf_node = get_sample_leaf_node();
        let expected_result = get_sample_leaf_node_expected_encoding();
        let result = leaf_node
            .rlp_encode()
            .unwrap();
        assert!(result == expected_result);
    }

    #[test]
    fn should_get_leaf_node_hash_correctly() {
        let leaf_node = get_sample_leaf_node();
        let expected_result = get_sample_leaf_node_expected_hash();
        let result = leaf_node
            .hash()
            .unwrap();
        assert!(result == expected_result);
    }
}
