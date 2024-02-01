// Sled DB functions

use std::str::FromStr;

use crate::wallet::{MyWallet, self, init_wallet};
use anyhow::{anyhow, Context, Result};
use bdk::bitcoin::util::bip32::ExtendedPrivKey;
use orion::{aead, kdf, pwhash, util, kex::SecretKey};
use sled::{Db, Tree};
use tauri::api::path;

fn sled_db() -> Result<Db> {
    let data_path = path::data_dir().context("Can't access systems data directory")?;
    let db_path = data_path.join("shamir_wallet").join("wallets_info.db");
    let db = sled::open(&db_path)?;
    Ok(db)
}

// PERF: look at bottlenecks -> something takes very long
pub fn save_private_key(name: &String, password: &String, xprv: ExtendedPrivKey) -> Result<()> {
    let db = sled_db()?;
    let tree = db.open_tree(name)?;

    let passw = pwhash::Password::from_slice(password.as_bytes())?;
    let passw_hash = pwhash::hash_password(&passw, 3, 1 << 16)?;
    tree.insert(b"password_hash", passw_hash.unprotected_as_encoded())?;

    let mut salt_bytes = [0u8; 64];
    util::secure_rand_bytes(&mut salt_bytes)?;
    let salt = kdf::Salt::from_slice(&salt_bytes)?;
    let derived_key = kdf::derive_key(&passw, &salt, 3, 1 << 16, 32)?;

    tree.insert(b"enc_salt", salt_bytes.to_vec())?;
    tree.insert(b"private_key_wif", aead::seal(&derived_key,
        &xprv.to_string().as_bytes())?)?;
    Ok(())
}

pub fn retrieve_wallet(name: &String, password: &String) -> Result<MyWallet> {
    let db = sled_db()?;
    let tree = db.open_tree(&name)?;
    let derived_key = derive_encryption_key(password, &tree)?;

    // retrive and decrypt private key
    let enc_private_key = tree.get(b"private_key_wif")?.context("Private key now saved in DB")?;
    let private_key_str = &String::from_utf8(aead::open(&derived_key, &enc_private_key)?)?;
    let xprv = ExtendedPrivKey::from_str(private_key_str)?;

    let wallet = init_wallet(&name, xprv)?;
    Ok(wallet)
}

pub fn get_wallet_names() -> Result<Vec<String>>{
    let db = sled_db()?;
    let mut keys = Vec::new();
    for key in db.tree_names() {
        let key_string = std::str::from_utf8(&key)?.to_string();
        if key_string == "__sled__default" { continue; }
        keys.push(key_string);
    }
    Ok(keys)
}

pub fn delete_wallet(name: String) -> Result<bool> {
    wallet::delete_wallet_db(&name)?;
    let db = sled_db()?;
    Ok(!db.drop_tree(name.as_bytes())?) 
}


fn derive_encryption_key(password: &String, wallet_tree: &Tree) -> Result<SecretKey> {
    // verify password with hash in DB
    let passw = pwhash::Password::from_slice(password.as_bytes())?;
    let stored_hash = wallet_tree.get(b"password_hash")?.context("Password hash not saved in the DB")?;
    let hash = pwhash::PasswordHash::from_encoded(std::str::from_utf8(&stored_hash)?)?;
    if !pwhash::hash_password_verify(&hash, &passw).is_ok() {
        return Err(anyhow!("Wrong password"));
    }

    // derive decryption key from password
    let salt_bytes = wallet_tree.get(b"enc_salt")?.context("Password hash not saved in the DB")?.to_vec();
    let salt = kdf::Salt::from_slice(&salt_bytes)?;
    let derived_key = kdf::derive_key(&passw, &salt, 3, 1 << 16, 32)?;
    Ok(derived_key)
}
