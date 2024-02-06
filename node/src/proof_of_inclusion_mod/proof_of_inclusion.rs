use super::proof_of_inclusion_error::ProofOfInclusionError;
use crate::messages::message_constants::NOT_FOUND_COMMAND;
use crate::{
    block_mod::blockchain::BlockChain, messages::read_from_bytes::fill_command,
    wallet_utils::merkle_block::MerkleBlock,
};
use bitcoin_hashes::{sha256d, Hash};
use std::collections::HashSet;
use std::{
    io::Write,
    sync::{Arc, Mutex},
};

/// Computes the path of transaction IDs (txids) for a given leaf node index in a Merkle tree.
///
/// The `index` parameter represents the index of the leaf node.
/// The `levels` parameter is a reference to a vector of levels in the Merkle tree.
/// Each level contains a vector of transaction IDs (txids) for the nodes in that level.
///
/// Returns a `HashSet` containing the txids in the path from the leaf node to the root.
/// Computes the hash of two child nodes in a Merkle tree.
fn compute_hash(left_child: Vec<u8>, right_child: Vec<u8>) -> Vec<u8> {
    let vector = vec![left_child, right_child].concat();

    sha256d::Hash::hash(&vector).to_byte_array().to_vec()
}

fn txid_path(mut index: usize, levels: &Vec<Vec<Vec<u8>>>) -> HashSet<Vec<u8>> {
    let mut path: HashSet<Vec<u8>> = HashSet::new();

    for lvl in levels {
        path.insert(lvl[index].clone());
        index /= 2;
    }

    path
}

/// Calculates a Merkle proof given the levels of the Merkle tree, a path of transaction IDs (txids), and the Merkle root.
///
/// # Arguments
///
/// * `levels` - A reference to a vector of levels in the Merkle tree. Each level contains a vector of transaction IDs (txids) for the nodes in that level.
/// * `path` - A `HashSet` containing the txids in the path from the leaf node to the root.
/// * `merkle_root` - The Merkle root of the tree.
///
/// # Returns
///
/// A `MerkleBlock` containing the hashes, flags, and Merkle root for the proof.
fn calculate_merkle_proof(
    levels: &Vec<Vec<Vec<u8>>>,
    path: HashSet<Vec<u8>>,
    merkle_root: Vec<u8>,
) -> MerkleBlock {
    let mut index = 0;
    let mut flags: Vec<u8> = Vec::new();
    let mut hashes: Vec<Vec<u8>> = Vec::new();
    let mut aux: Vec<Vec<u8>> = Vec::new();

    for i in (1..levels.len()).rev() {
        flags.push(1);
        if i == 1 {
            flags.push(0);
            flags.push(0);
            hashes.push(levels[i - 1][index * 2].clone());
            hashes.push(levels[i - 1][index * 2 + 1].clone());
            continue;
        }
        if path.contains(&levels[i - 1][index * 2 + 1]) {
            flags.push(0);
            hashes.push(levels[i - 1][index * 2].clone());
            index = index * 2 + 1;
            continue;
        }
        //path contiene hijo izq
        aux.push(levels[i - 1][index * 2 + 1].clone());
        index *= 2;
    }

    aux.reverse();
    hashes.extend(aux);

    MerkleBlock::new(hashes, flags, merkle_root)
}

/// Calculates the Merkle root given a list of transactions and updates the levels of the Merkle tree.
///
/// # Arguments
///
/// * `txn_list` - A mutable reference to a vector of transaction IDs (txids).
/// * `levels` - A mutable reference to a vector of levels in the Merkle tree. Each level contains a vector of transaction IDs (txids) for the nodes in that level.
///
/// # Returns
///
/// The Merkle root as a vector of bytes.
fn calculate_merkle_root(txn_list: &mut Vec<Vec<u8>>, levels: &mut Vec<Vec<Vec<u8>>>) -> Vec<u8> {
    let mut new_level_txn_list: Vec<Vec<u8>> = Vec::new();
    let mut merkle_root_hash: Vec<u8> = Vec::new();

    // Return when the length of the list is one
    if txn_list.len() == 1 {
        merkle_root_hash.extend(txn_list[0].as_slice());
        return merkle_root_hash;
    }

    // If list is odd, duplicate and append the last item
    if txn_list.len() % 2 != 0 {
        if let Some(last_txn) = txn_list.last() {
            txn_list.push(last_txn.clone());
        }
    }

    // Concatenation and hashing
    for i in (0..(txn_list.len() - 1)).step_by(2) {
        let hash = compute_hash(txn_list[i].clone(), txn_list[i + 1].clone());

        new_level_txn_list.push(hash);
    }
    levels.push(new_level_txn_list.clone());

    calculate_merkle_root(&mut new_level_txn_list, levels)
}

fn not_found(stream: &mut dyn Write) -> Result<(), ProofOfInclusionError> {
    let not_found = NOT_FOUND_COMMAND.to_string();
    let buffer = fill_command(&not_found).as_bytes().to_vec();
    stream
        .write(&buffer)
        .map_err(|_| ProofOfInclusionError::WriteError)?;
    Ok(())
}

/// Sends a proof of inclusion for a transaction in a block to the specified stream.
///
/// # Arguments
///
/// * `block_hash` - The hash of the block containing the transaction.
/// * `txn` - The transaction to prove inclusion for.
/// * `blockchain` - A reference to the blockchain as a shared mutable state.
/// * `stream` - A mutable reference to the stream to write the proof to.
///
/// # Returns
///
/// An `Ok(())` value on success, or an `Err(ProofOfInclusionError)` if an error occurred.
pub fn send_proof(
    mut block_hash: Vec<u8>,
    mut txn: Vec<u8>,
    blockchain: &Arc<Mutex<BlockChain>>,
    stream: &mut dyn Write,
) -> Result<(), ProofOfInclusionError> {
    let blockchain = blockchain
        .lock()
        .map_err(|_| ProofOfInclusionError::LockBlockChain)?;

    block_hash.reverse();
    txn.reverse();

    let block = match blockchain.get_blocks().get(&block_hash) {
        Some(block) => block,
        None => return not_found(stream),
    };

    let mut txids = block.get_txn_ids();
    let mut i: usize = 0;
    let mut tx_found: bool = false;

    while i < txids.len() {
        if txn == txids[i] {
            tx_found = true;
            break;
        }
        i += 1;
    }

    if !tx_found {
        return not_found(stream);
    }

    if txids.len() % 2 != 0 {
        if let Some(last_txn) = txids.last() {
            txids.push(last_txn.clone());
        }
    }

    let mut levels: Vec<Vec<Vec<u8>>> = Vec::new();
    levels.push(txids.clone());
    let merkle_root = calculate_merkle_root(&mut txids, &mut levels); // nos fijamos antes que cumple o ni hace falta?
    let proof = calculate_merkle_proof(&levels, txid_path(i, &levels), merkle_root);
    stream
        .write(&proof.as_bytes())
        .map_err(|_| ProofOfInclusionError::WriteError)?;
    drop(blockchain);
    Ok(())
}

#[cfg(test)]
mod poi_test {
    use std::fs::OpenOptions;

    use crate::{
        block_mod::block::Block, messages::read_from_bytes::decode_hex,
        proof_of_inclusion_mod::level::Level, wallet_utils::merkle_block::MerkleBlock,
    };

    use super::{calculate_merkle_proof, calculate_merkle_root, compute_hash, txid_path};

    pub fn client_proof_of_inclusion(mut merkle_block: MerkleBlock) -> bool {
        let mut levels: Vec<Level> = Vec::new();
        let mut i = 1;
        let mut hashes = merkle_block.hashes();

        hashes.reverse();
        levels.push(Level::new(None, None)); //root

        while i < merkle_block.flags().len() {
            if merkle_block.flags()[i] == 0 && merkle_block.flags()[i + 1] == 0 {
                levels.push(Level::new(hashes.pop(), hashes.pop()));
                break;
            }

            if merkle_block.flags()[i] == 1 {
                levels.push(Level::new(None, None));
                i += 1;
                continue;
            }
            levels.push(Level::new(hashes.pop(), None));
            i += 2;
        }

        levels.reverse();

        for i in 0..levels.len() {
            if levels[i + 1].get_right().is_none() && levels[i + 1].get_left().is_none() {
                let left = levels[i].get_left().clone().unwrap();
                let right = levels[i].get_right().clone().unwrap();
                levels[i + 1].set_left(Some(compute_hash(left, right)));

                if (i + 1) == (levels.len() - 1) {
                    break;
                }
                levels[i + 1].set_right(hashes.pop());
            } else if levels[i + 1].get_right().is_some() && levels[i + 1].get_left().is_none() {
                let left = levels[i].get_left().clone().unwrap();
                let right = levels[i].get_right().clone().unwrap();
                levels[i + 1].set_left(Some(compute_hash(left, right)));
            } else {
                let left = levels[i].get_left().clone().unwrap();
                let right = levels[i].get_right().clone().unwrap();
                levels[i + 1].set_right(Some(compute_hash(left, right)));
            }
        }

        &levels[levels.len() - 1].get_left().clone().unwrap() == merkle_block.get_merkle_root()
    }

    #[test]
    fn test_make_merkle_tree_single_tx() {
        let mut txids: Vec<Vec<u8>> = vec![[1].to_vec(), [1].to_vec()];
        let i = 0;

        let mut levels: Vec<Vec<Vec<u8>>> = Vec::new();
        levels.push(txids.clone()); // hay que hacerlo par antes

        let merkle_root = calculate_merkle_root(&mut txids, &mut levels);

        let path_to_prove = txid_path(i, &levels);
        let merkle_tree = calculate_merkle_proof(&levels, path_to_prove, merkle_root);

        assert!(client_proof_of_inclusion(merkle_tree));
    }

    #[test]
    fn test_make_merkle_tree_two_txs() {
        let mut txids: Vec<Vec<u8>> = vec![[1].to_vec(), [2].to_vec()];
        let i = 1;

        let mut levels: Vec<Vec<Vec<u8>>> = Vec::new();
        levels.push(txids.clone()); // hay que hacerlo par antes

        let merkle_root = calculate_merkle_root(&mut txids, &mut levels);

        let path_to_prove = txid_path(i, &levels);
        let merkle_tree = calculate_merkle_proof(&levels, path_to_prove, merkle_root);

        assert!(client_proof_of_inclusion(merkle_tree));
    }

    #[test]
    fn test_make_merkle_tree_three_txs() {
        let mut txids: Vec<Vec<u8>> = vec![[1].to_vec(), [2].to_vec(), [3].to_vec(), [3].to_vec()];
        let i = 2;

        let mut levels: Vec<Vec<Vec<u8>>> = Vec::new();
        levels.push(txids.clone()); // hay que hacerlo par antes

        let merkle_root = calculate_merkle_root(&mut txids, &mut levels);

        let path_to_prove = txid_path(i, &levels);
        let merkle_tree = calculate_merkle_proof(&levels, path_to_prove, merkle_root);

        assert!(client_proof_of_inclusion(merkle_tree));
    }

    #[test]
    fn test_node_proof_of_inclusion() {
        let file_path = "data/test_block.bin";

        // Open the file in read-write mode
        let mut file = OpenOptions::new()
            .read(true)
            .open(file_path)
            .expect("Failed to open the file");

        let block = Block::from_bytes(&mut file).unwrap();

        let mut txids = block.get_txn_ids().clone();
        let txid = txids[1].clone();

        let mut i: usize = 0;
        //let mut tx_found: bool = false;
        let tx_count = txid.len();

        while i < tx_count {
            if &txid == &txids[i] {
                //tx_found = true;
                break;
            }
            i += 1;
        }

        if txids.len() % 2 != 0 {
            if let Some(last_txn) = txids.last() {
                txids.push(last_txn.clone());
            }
        }

        let mut levels: Vec<Vec<Vec<u8>>> = Vec::new();
        levels.push(txids.clone());
        let merkle_root = calculate_merkle_root(&mut txids, &mut levels);
        let path_to_prove = txid_path(i, &levels);
        let merkle_tree = calculate_merkle_proof(&levels, path_to_prove, merkle_root);

        assert!(client_proof_of_inclusion(merkle_tree));
    }

    #[test]
    fn test_node_proof_of_inclusion2() {
        let file_path = "data/test_block2.bin";

        // Open the file in read-write mode
        let mut file = OpenOptions::new()
            .read(true)
            .open(file_path)
            .expect("Failed to open the file");

        let block = Block::from_bytes(&mut file).unwrap();

        let mut txids = block.get_txn_ids().clone();
        let mut txid =
            decode_hex("21aef0ffde1d441738d7b66896cb7d2f4005b384b53fe8951383cb9c752b2bdc").unwrap();
        txid.reverse();

        let i: usize = 0;
        //let tx_count = txid.len();

        if txids.len() % 2 != 0 {
            if let Some(last_txn) = txids.last() {
                txids.push(last_txn.clone());
            }
        }

        let mut levels: Vec<Vec<Vec<u8>>> = Vec::new();
        levels.push(txids.clone());

        let merkle_root = calculate_merkle_root(&mut txids, &mut levels);
        let path_to_prove = txid_path(i, &levels);
        let merkle_block = calculate_merkle_proof(&levels, path_to_prove, merkle_root);

        assert!(client_proof_of_inclusion(merkle_block));
    }
}
