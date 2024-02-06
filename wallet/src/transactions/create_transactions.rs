use bitcoin_hashes::{hash160, sha256, sha256d, Hash};
use node::block_mod::{script::Script, transaction::Transaction, tx_in::TxIn, tx_out::TxOut};
use secp256k1::{Message, PublicKey, Secp256k1, SecretKey};

use super::create_transaction_error::TransactionCreateError;

/// Creates a pay-to-public-key-hash (P2PKH) script with the given hash160 value.
///
/// # Arguments
///
/// * `h160`: A vector of bytes representing the hash160 value.
///
/// # Returns
///
/// A `Script` object representing the P2PKH script.
///
/// # Description
///
/// This function creates a P2PKH script using the given hash160 value. The P2PKH script consists of a series of opcodes and data values.
/// The script starts with the opcode `OP_DUP` (0x76) to duplicate the top stack item, followed by `OP_HASH160` (0xa9) to hash the duplicated
/// value. Then, the hash160 value is added to the script. After that, `OP_EQUALVERIFY` (0x88) is used to verify that the top two stack items
/// are equal. Finally, `OP_CHECKSIG` (0xac) is used to verify the signature using the public key.
///
fn p2pkh(h160: Vec<u8>) -> Script {
    Script::new(Some(vec![
        vec![0x76],
        vec![0xa9],
        h160,
        vec![0x88],
        vec![0xac],
    ]))
}

/// Decodes a Base58 encoded address into its corresponding payload.
///
/// # Arguments
///
/// * `address`: A vector of bytes representing the Base58 encoded address.
///
/// # Returns
///
/// A vector of bytes representing the decoded payload.
///
/// # Description
///
/// This function decodes a Base58 encoded address into its corresponding payload. The Base58 encoded address typically represents a
/// cryptocurrency address. The decoding process involves reversing the Base58 encoded address, decoding it into a byte vector, and
/// verifying the checksum to ensure the integrity of the payload. If the checksum is valid, the payload is returned. Otherwise,
/// an empty vector is returned.
fn decode_base58(address: &Vec<u8>) -> Vec<u8> {
    if let Ok(/*mut*/ combined) = bs58::decode(address).into_vec() {
        //let checksum = &combined[combined.len() - 4..];
        //let payload = &combined[..combined.len() - 4];

        return combined[1..combined.len() - 4].to_vec();
    };
    Vec::new()
}

/// Generates a Base58-encoded Bitcoin address from a public key.
///
/// # Arguments
///
/// * `public_key`: A vector of bytes representing the public key.
///
/// # Returns
///
/// A vector of bytes representing the Base58-encoded Bitcoin address.
///
/// # Description
///
/// This function takes a public key as input and generates a Base58-encoded Bitcoin address. It first calculates the hash160 value of the public key using the hash160 algorithm. Then, it combines the version prefix (0x6f) with the hash160 value and calculates the double hash (sha256d) of the combined data. The first 4 bytes of the double hash are used as the checksum. Finally, the version prefix, hash160 value, and checksum are concatenated and encoded using Base58 to obtain the Bitcoin address.
pub fn address_from_public_key(public_key: &[u8]) -> Vec<u8> {
    let h160 = hash160::Hash::hash(public_key).to_byte_array();

    let version_prefix: [u8; 1] = [0x6f];
    let doble_hash = sha256d::Hash::hash(&[&version_prefix[..], &h160[..]].concat());
    let checksum = &doble_hash[..4];

    let input = [&version_prefix[..], &h160[..], checksum].concat();

    bs58::encode(input).into_vec()
}

/// Generates a script (public key script) from a Base58-encoded Bitcoin address.
///
/// # Arguments
///
/// * `address`: A vector of bytes representing the Base58-encoded Bitcoin address.
///
/// # Returns
///
/// A vector of bytes representing the script.
///
/// # Description
///
/// This function takes a Base58-encoded Bitcoin address as input and generates the corresponding script (public key script). It first decodes the Base58-encoded address using the `decode_base58` function to obtain the hash160 value. Then, it calls the `p2pkh` function to create the script using the hash160 value. The resulting script is returned as a vector of bytes.
pub fn pk_script_from_address(address: &Vec<u8>) -> Vec<u8> {
    let h160 = decode_base58(address);

    p2pkh(h160).as_bytes()
}

/// Generates a script (public key script) from a public key.
///
/// # Arguments
///
/// * `public_key`: A vector of bytes representing the public key.
///
/// # Returns
///
/// A vector of bytes representing the script.
///
/// # Description
///
/// This function generates a script (public key script) from a given public key. It first calls the `address_from_public_key` function to obtain the Base58-encoded Bitcoin address corresponding to the public key. Then, it decodes the Base58-encoded address using the `decode_base58` function to obtain the hash160 value. Finally, it calls the `p2pkh` function to create the script using the hash160 value, and returns the resulting script as a vector of bytes.
///
pub fn pk_script_from_public_key(public_key: &[u8]) -> Vec<u8> {
    let address = address_from_public_key(public_key);
    let pub_key_dec = decode_base58(&address);

    p2pkh(pub_key_dec).as_bytes()
}

/// Creates a list of transaction outputs (TxOut) from a list of target addresses and amounts, along with a fee.
///
/// # Arguments
///
/// * `targets`: A vector of tuples representing the target addresses and amounts. Each tuple contains a vector of bytes representing the address and an `i64` amount.
/// * `fee`: The fee amount to be deducted from the total amount.
///
/// # Returns
///
/// A tuple containing the list of transaction outputs (TxOut) and the total amount including the fee.
///
/// # Description
///
/// This function creates a list of transaction outputs (TxOut) from a given list of target addresses and amounts, along with a fee. It iterates over each target, decodes the Base58-encoded address using the `decode_base58` function, and creates a script (public key script) using the decoded hash160 value. The script and amount are then used to create a new transaction output (TxOut), which is added to the `txout_list`. The total amount is updated by adding the current amount. Finally, the function returns a tuple containing the list of transaction outputs (TxOut) and the total amount including the fee.
///
fn create_txout_list(targets: Vec<(Vec<u8>, i64)>, fee: i64) -> (Vec<TxOut>, i64) {
    let mut total_amount = fee;
    let mut txout_list = vec![];

    for (address, amount) in targets {
        let h160 = decode_base58(&address);
        let script = Script::new(Some(vec![
            vec![0x76],
            vec![0xa9],
            h160,
            vec![0x88],
            vec![0xac],
        ]));
        let txout = TxOut::new(amount, script.as_bytes());
        total_amount += amount;
        txout_list.push(txout);
    }
    (txout_list, total_amount)
}

/// Creates a list of transaction inputs (TxIn) from a list of unspent transaction outputs (UTXO) and the total amount required.
///
/// # Arguments
///
/// * `utxo`: A mutable vector of tuples representing the unspent transaction outputs. Each tuple contains a vector of bytes representing the transaction ID, a u32 representing the output index, and a TxOut struct representing the output details.
/// * `total_amount`: The total amount required for the transaction.
///
/// # Returns
///
/// A Result containing a tuple with the list of transaction inputs (TxIn) and the change amount, or an error of type TransactionCreateError if there are insufficient funds.
///
/// # Description
///
/// This function creates a list of transaction inputs (TxIn) from a given list of unspent transaction outputs (UTXO) and the total amount required for the transaction. It iterates over the UTXO list in reverse order, popping the last element. For each UTXO, it creates a new transaction input (TxIn) using the transaction ID, output index, empty scriptSig, and a sequence value of 0xffffffff. The amount of the UTXO is added to the accumulated amount. The transaction input is added to the `txin_list`. If there are no more UTXO available before reaching the total required amount, an error of type TransactionCreateError::InsufficientFunds is returned. Finally, the function calculates the change amount by subtracting the total required amount from the accumulated amount and returns a tuple containing the list of transaction inputs (TxIn) and the change amount.
///
fn create_txin_list(
    mut utxo: Vec<(Vec<u8>, u32, TxOut)>,
    total_amount: i64,
) -> Result<(Vec<TxIn>, i64), TransactionCreateError> {
    let mut acum_amount = 0;
    let mut txin_list = vec![];

    while acum_amount < total_amount {
        if let Some(txout) = utxo.pop() {
            let txin = TxIn::new(txout.0, txout.1, vec![], 0xffffffff);
            acum_amount += txout.2.get_value();
            txin_list.push(txin);
        } else {
            return Err(TransactionCreateError::InsufficientFounds);
        }
    }

    let change_amount = acum_amount - total_amount;

    Ok((txin_list, change_amount))
}

/// Signs a transaction by adding the signature scripts to each transaction input.
///
/// # Arguments
///
/// * `transaction`: A mutable reference to a Transaction struct representing the transaction to be signed.
/// * `private_key`: A SecretKey representing the private key used for signing.
/// * `pk_script`: A reference to a vector of bytes representing the public key script associated with the transaction inputs.
///
/// # Description
///
/// This function iterates over each transaction input in the transaction and adds the signature scripts. It uses the provided private key to sign each input individually. For each input, it calculates the signature hash by calling `transaction.sig_hash` with the input index and the public key script. It then creates a message from the hashed data using the sha256::Hash algorithm. The private key is used to sign the message with the secp256k1 algorithm, producing a DER-encoded signature. The DER-encoded signature and the serialized public key are concatenated into a signature script. Finally, the signature script is set for the corresponding input in the transaction using `transaction.set_signature`.
///
fn sign_transaction(transaction: &mut Transaction, private_key: SecretKey, pk_script: &[u8]) {
    let secp = Secp256k1::new();

    for i in 0..transaction.get_tx_in_list().len() {
        let signature_hash = transaction.sig_hash(i, pk_script);
        let message = Message::from_hashed_data::<sha256::Hash>(&signature_hash);
        let der = secp
            .sign_ecdsa(&message, &private_key)
            .serialize_der()
            .to_vec();
        let sig = vec![der, vec![1_u8]].concat();
        let sec = PublicKey::from_secret_key(&secp, &private_key)
            .serialize()
            .to_vec();
        let signature_script = Script::new(Some(vec![sig, sec]));

        transaction.set_signature(i, signature_script.as_bytes());
    }
}

/// Creates a new transaction by assembling inputs, outputs, and signing it with a private key.
///
/// # Arguments
///
/// * `targets`: A vector of tuples containing the recipient addresses and corresponding amounts to be sent.
/// * `utxo`: A vector of tuples representing the unspent transaction outputs (UTXOs) available for spending.
/// * `private_key`: A reference to a vector of bytes representing the private key used for signing the transaction.
/// * `fee`: The fee amount to be deducted from the total transaction amount.
///
/// # Returns
///
/// * `Result<Transaction, TransactionCreateError>`: A Result enum with either a Transaction struct representing the created transaction or an error of type TransactionCreateError.
///
/// # Description
///
/// This function creates a new transaction by following the steps below:
///
/// 1. Initialize a new instance of the secp256k1::Secp256k1 struct.
/// 2. Deserialize the provided private_key vector of bytes into a SecretKey. If deserialization fails, return an error of type TransactionCreateError::PrivateKey.
/// 3. Generate the public key from the private key using the secp256k1 algorithm and serialize it into a vector of bytes.
/// 4. Obtain the public key script (pk_script) from the public key by calling the pk_script_from_public_key function.
/// 5. Create the list of transaction outputs (txout_list) and calculate the total transaction amount by calling the create_txout_list function with the targets and fee parameters.
/// 6. Create the list of transaction inputs (txin_list) and calculate the change amount by calling the create_txin_list function with the utxo and total_amount parameters. If there are insufficient funds to cover the total amount, return an error of type TransactionCreateError::InsufficientFounds.
/// 7. Create a new transaction (Transaction) object with the version 1, txin_list, txout_list, and lock_time set to 0.
/// 8. Sign the transaction by calling the sign_transaction function, passing in a mutable reference to the transaction, the private key, and the pk_script.
/// 9. Return the signed transaction as Ok(transaction).
///
pub fn create_transaction(
    targets: Vec<(Vec<u8>, i64)>,
    utxo: Vec<(Vec<u8>, u32, TxOut)>,
    private_key: &[u8],
    fee: i64,
) -> Result<Transaction, TransactionCreateError> {
    let secp = Secp256k1::new();

    let private_key =
        SecretKey::from_slice(private_key).map_err(|_| TransactionCreateError::PrivateKey)?;
    let public_key = PublicKey::from_secret_key(&secp, &private_key)
        .serialize()
        .to_vec();
    let pk_script = pk_script_from_public_key(&public_key);

    let (mut txout_list, total_amount) = create_txout_list(targets, fee);
    let (txin_list, change_amount) = create_txin_list(utxo, total_amount)?;

    let change = TxOut::new(change_amount, pk_script.clone());
    txout_list.push(change);

    let mut transaction = Transaction::new(1, txin_list, txout_list, 0);

    sign_transaction(&mut transaction, private_key, &pk_script);

    Ok(transaction)
}

#[cfg(test)]
mod create_transactions_test {
    use std::str::FromStr;

    use bitcoin_hashes::*;
    use node::{
        block_mod::{script::Script, transaction::Transaction, tx_in::TxIn, tx_out::TxOut},
        messages::read_from_bytes::{decode_hex, encode_hex},
    };
    use secp256k1::{Message, PublicKey, Secp256k1, SecretKey};

    use crate::transactions::{
        create_transaction_error::TransactionCreateError, create_transactions::decode_base58,
    };

    use super::{address_from_public_key, pk_script_from_address};

    #[test]
    pub fn create_transaction() -> Result<(), TransactionCreateError> {
        // total de la txout que voy a usar 0.01875221
        // quiero depositar desde la cuenta address a la cuenta target 0.012345

        let address = b"n1mDu5Zd5qS75vqK1yqnKmEZQzDyncQqj4".to_vec();
        let target = b"mp3PDnKDtxPYrPKcYLGX1pXMe6KwAsfquD".to_vec();

        //let pub_key = "02E641B11A0FB5A761814D0F166ADC4E654037C844B44226219AE3D6947EBC4DA6";
        let private_key = "740A9C5D2BD171E99DDDC268A26179FCAD9BFE9A7A8188725EDA0D1D9F6D2264";

        let mut prev_tx =
            decode_hex("7a56640d6c89ce4744ab77c5332c87fec02c58720a7fc1ba19d6b6546f5b29e8")?;
        prev_tx.reverse();
        let prev_index = 0; // 0.01875221 -0.012345 - 0.003

        let txin = TxIn::new(prev_tx, prev_index, vec![], 0xffffffff);

        // calculo el cambio
        let change_amount = 0.0009 * 100000000.0;
        let change_h160 = decode_base58(&address);
        let change_script = Script::new(Some(vec![
            vec![0x76],
            vec![0xa9],
            change_h160,
            vec![0x88],
            vec![0xac],
        ]));
        let change_txout = TxOut::new(change_amount as i64, change_script.as_bytes());

        let target_amount = 0.0021 * 100000000.0;
        let target_h160 = decode_base58(&target);
        let target_script = Script::new(Some(vec![
            vec![0x76],
            vec![0xa9],
            target_h160,
            vec![0x88],
            vec![0xac],
        ]));
        let target_txout = TxOut::new(target_amount as i64, target_script.as_bytes());

        let mut tx = Transaction::new(1, vec![txin], vec![change_txout, target_txout], 0);

        let secp = Secp256k1::new();

        let signature_hash = tx.sig_hash(0, &change_script.as_bytes());

        let private_key = SecretKey::from_str(private_key)?;

        let message = Message::from_hashed_data::<sha256::Hash>(&signature_hash);

        let der = secp
            .sign_ecdsa(&message, &private_key)
            .serialize_der()
            .to_vec();

        let sig = vec![der, vec![1_u8]].concat();

        let sec = PublicKey::from_secret_key(&secp, &private_key)
            .serialize()
            .to_vec();

        let signature_script = Script::new(Some(vec![sig, sec]));

        tx.set_signature(0, signature_script.as_bytes());

        println!("{:?}", encode_hex(&tx.as_bytes())?);

        Ok(())
    }
    #[test]
    pub fn test_address_from_public_key() -> Result<(), TransactionCreateError> {
        let public_key =
            decode_hex("02E641B11A0FB5A761814D0F166ADC4E654037C844B44226219AE3D6947EBC4DA6")?;
        let address = b"n1mDu5Zd5qS75vqK1yqnKmEZQzDyncQqj4".to_vec();

        let address_calculated = address_from_public_key(&public_key);

        assert_eq!(address_calculated, address);
        Ok(())
    }

    #[test]
    pub fn test_pk_script_from_address() -> Result<(), TransactionCreateError> {
        let address = b"mzx5YhAH9kNHtcN481u6WkjeHjYtVeKVh2".to_vec(); //ejemplo sacado del libro

        let h160 = decode_hex("d52ad7ca9b3d096a38e752c2018e6fbc40cdf26f")?;
        let pk_script = vec![
            vec![0x76],
            vec![0xa9],
            vec![20],
            h160,
            vec![0x88],
            vec![0xac],
        ]
        .concat();

        let pk_script_calculated = pk_script_from_address(&address);

        assert_eq!(pk_script, pk_script_calculated);

        Ok(())
    }

    #[test]
    pub fn test_pk_script_from_public_key() -> Result<(), TransactionCreateError> {
        let public_key =
            decode_hex("0362599B444272856B51E7EE10A4B70A683A9965AD3859E4D75E9B9EC136F84144")?;

        println!("{}", public_key.len());
        let address = address_from_public_key(&public_key);

        let pk_script = pk_script_from_address(&address);

        println!("{:?}", pk_script);

        Ok(())
    }

    #[test]
    pub fn test_create_transaction_2() -> Result<(), TransactionCreateError> {
        let address = b"mq5boK8wasubp4QHZ349damhWQLCthdrKP".to_vec();
        let target = b"n3yL92bzbMkicfYwUS3K7huHj81ew877ob".to_vec();

        let private_key = "11063638E1C47A9EEEDCDB476654644B00F7BFF9798031CFBB1EB9DA4D8B51F4";

        let mut prev_tx =
            decode_hex("3464a5386b818c901b910d96ee71bce0ea9a4465719ea458deea9df81e8504f5")?;
        prev_tx.reverse();
        let prev_index = 0;

        let txin = TxIn::new(prev_tx, prev_index, vec![], 0xffffffff);

        // calculo el cambio
        let change_amount = 0.0009 * 100000000.0;
        let change_h160 = decode_base58(&address);
        let change_script = Script::new(Some(vec![
            vec![0x76],
            vec![0xa9],
            change_h160,
            vec![0x88],
            vec![0xac],
        ]));
        let change_txout = TxOut::new(change_amount as i64, change_script.as_bytes());

        let target_amount = 0.0021 * 100000000.0;
        let target_h160 = decode_base58(&target);
        let target_script = Script::new(Some(vec![
            vec![0x76],
            vec![0xa9],
            target_h160,
            vec![0x88],
            vec![0xac],
        ]));
        let target_txout = TxOut::new(target_amount as i64, target_script.as_bytes());

        let mut tx = Transaction::new(1, vec![txin], vec![change_txout, target_txout], 0);

        let secp = Secp256k1::new();

        let signature_hash = tx.sig_hash(0, &change_script.as_bytes());

        let private_key = SecretKey::from_str(private_key)?;

        let message = Message::from_hashed_data::<sha256::Hash>(&signature_hash);

        let der = secp
            .sign_ecdsa(&message, &private_key)
            .serialize_der()
            .to_vec();

        let sig = vec![der, vec![1_u8]].concat();
        println!("len sig {}", sig.len());

        let sec = PublicKey::from_secret_key(&secp, &private_key)
            .serialize()
            .to_vec();

        println!("len sec {}", sec.len());

        let signature_script = Script::new(Some(vec![sig, sec]));

        tx.set_signature(0, signature_script.as_bytes());

        println!("len script {}", signature_script.as_bytes().len());

        println!("{:?}", encode_hex(&tx.as_bytes())?);

        Ok(())
    }
}
