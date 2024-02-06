use bitcoin_hashes::{sha256d, Hash};
use node::wallet_utils::merkle_block::MerkleBlock;

use crate::proof_of_inclusion::check_proof_error::CheckProofError;

#[derive(Debug)]
pub struct Level {
    left: Option<Vec<u8>>,
    right: Option<Vec<u8>>,
}

/// Computes the hash of the concatenated left and right child data using the SHA-256d algorithm.
///
/// # Arguments
///
/// * `left_child`: A `Vec<u8>` representing the data of the left child.
/// * `right_child`: A `Vec<u8>` representing the data of the right child.
///
/// # Returns
///
/// A `Vec<u8>` containing the resulting hash of the concatenated data.
///
/// # Description
///
/// This function takes two vectors, `left_child` and `right_child`, representing the data of the left
/// and right children, respectively. It concatenates the data into a single vector, computes the hash
/// of the concatenated data using the SHA-256d algorithm, and returns the resulting hash as a new vector.
/// The SHA-256d algorithm applies the SHA-256 hash function twice to the input data, resulting in a
/// 256-bit hash.
///
fn compute_hash(left_child: Vec<u8>, right_child: Vec<u8>) -> Vec<u8> {
    let vector = vec![left_child, right_child].concat();

    sha256d::Hash::hash(&vector).to_byte_array().to_vec()
}

/// Calculates and verifies the proof of inclusion for a given Merkle block.
///
/// # Arguments
///
/// * `merkle_block`: A `MerkleBlock` object representing the Merkle block for which the proof of inclusion is to be calculated and verified.
///
/// # Returns
///
/// A boolean value indicating whether the proof of inclusion is valid or not. `true` if the proof is valid, `false` otherwise.
///
/// # Description
///
/// This function calculates and verifies the proof of inclusion for a given Merkle block. It starts by initializing an empty vector `levels`
/// to store the levels of the Merkle tree. It then iterates through the flags of the Merkle block to construct the levels of the tree.
/// If a flag is 0 and the next flag is also 0, it means that the current level has both left and right children, so it creates a new `Level`
/// object with the corresponding left and right child hashes and breaks the loop. If a flag is 1, it means that the current level has no children,
/// so it creates a new `Level` object with no children and continues to the next flag. If a flag is not 0 or 1, it means that the current level
/// has a left child but no right child, so it pops a hash from the `hashes` vector and creates a new `Level` object with the left child hash.
/// After constructing the levels, it reverses the order of the levels vector.
///
/// Then, it iterates through the levels vector and computes the missing hashes for each level by calling the `compute_hash` function. It fills
/// the missing left or right child hash depending on whether the next level has a missing left or right child. The iteration continues until
/// it reaches the last level, which represents the root of the Merkle tree.
///
/// Finally, it compares the calculated Merkle root with the Merkle root of the given Merkle block. If they are equal, it returns `true`, indicating
/// that the proof of inclusion is valid. Otherwise, it returns `false`.
///
pub fn get_proof_of_inclusion(mut merkle_block: MerkleBlock) -> Result<bool, CheckProofError> {
    let mut levels: Vec<Level> = Vec::new();
    let mut i = 1;
    let mut hashes = merkle_block.hashes();

    hashes.reverse();
    levels.push(Level {
        left: None,
        right: None,
    }); //root

    while i < merkle_block.flags().len() {
        if merkle_block.flags()[i] == 0 && merkle_block.flags()[i + 1] == 0 {
            levels.push(Level {
                left: hashes.pop(),
                right: hashes.pop(),
            });
            break;
        }

        if merkle_block.flags()[i] == 1 {
            levels.push(Level {
                left: None,
                right: None,
            });
            i += 1;
            continue;
        }
        levels.push(Level {
            left: hashes.pop(),
            right: None,
        });
        i += 2;
    }

    levels.reverse();

    for i in 0..levels.len() {
        if levels[i + 1].right.is_none() && levels[i + 1].left.is_none() {
            levels[i + 1].left = Some(compute_hash(
                levels[i].left.clone().ok_or(CheckProofError::Left)?,
                levels[i].right.clone().ok_or(CheckProofError::Right)?,
            ));
            if (i + 1) == (levels.len() - 1) {
                break;
            }
            levels[i + 1].right = hashes.pop();
        } else if levels[i + 1].right.is_some() && levels[i + 1].left.is_none() {
            levels[i + 1].left = Some(compute_hash(
                levels[i].left.clone().ok_or(CheckProofError::Left)?,
                levels[i].right.clone().ok_or(CheckProofError::Right)?,
            ));
        } else {
            levels[i + 1].right = Some(compute_hash(
                levels[i].left.clone().ok_or(CheckProofError::Left)?,
                levels[i].right.clone().ok_or(CheckProofError::Right)?,
            ));
        }
    }

    Ok(&levels[levels.len() - 1]
        .left
        .clone()
        .ok_or(CheckProofError::Left)?
        == merkle_block.get_merkle_root())
}
