use crate::db::save_wallet;
use bdk::bitcoin::Network;
use bdk::database::SqliteDatabase;
use bdk::{
    bitcoin::util::bip32::ExtendedPrivKey,
    keys::{
        bip39::{Language, Mnemonic, WordCount},
        DerivableKey, ExtendedKey, GeneratableKey, GeneratedKey,
    },
    template::Bip84,
    wallet::{self, AddressIndex},
};
use bdk::{miniscript, KeychainKind, Wallet};
use core::f32;
use std::path::Path;
use std::str::FromStr;

static NETWORK: Network = Network::Testnet;

pub type Wall = Wallet<SqliteDatabase>;
pub type Descriptor = Bip84<ExtendedPrivKey>;

pub fn create_wallet(name: &String, password: &String) -> (Wall, String) {
    // Generate fresh mnemonic
    let mnemonic: GeneratedKey<_, miniscript::Segwitv0> =
        Mnemonic::generate((WordCount::Words12, Language::English)).unwrap();
    // Convert mnemonic to string
    let mnemonic_words = mnemonic.to_string();
    // Parse a mnemonic
    let mnemonic = Mnemonic::parse(&mnemonic_words).unwrap();
    // Generate the extended key
    let xkey: ExtendedKey = mnemonic.into_extended_key().unwrap();
    // Get xprv from the extended key
    let xprv = xkey.into_xprv(NETWORK).unwrap();
    // Create a BDK wallet structure using BIP 84 descriptor ("m/84h/1h/0h/0" and "m/84h/1h/0h/1")
    let descriptor = Bip84(xprv, KeychainKind::External);
    let change_descriptor = Bip84(xprv, KeychainKind::Internal);

    let wallet = initialize_wallet(name, descriptor, change_descriptor);

    save_wallet(name, &password, &wallet).unwrap();
    (wallet, mnemonic_words)
}

pub fn initialize_wallet(
    name: &String,
    descriptor: Descriptor,
    change_descriptor: Descriptor,
) -> Wall {
    Wallet::new(descriptor, Some(change_descriptor), NETWORK, get_db(name)).unwrap()
}

pub fn initialize_wallet_strings(
    name: &String,
    descriptor: &String,
    change_descriptor: &String,
) -> Wallet<SqliteDatabase> {
    Wallet::new(descriptor, Some(change_descriptor), NETWORK, get_db(name)).unwrap()
}

pub fn recover_wallet(recovery_phrase: String) {
    let mnemonic = Mnemonic::from_str(&recovery_phrase);

    todo!()
}

// TODO: finish functions
pub trait WalletInterface {
    fn get_balance(&self) -> anyhow::Result<u64>;
    fn sync(&self);
    fn last_unusued_address(&self) -> anyhow::Result<wallet::AddressInfo>;
    fn create_tx(&self, recipient: String, amount: u128, fee_rate: f32);
    fn sign(&self);
    fn broadcast(&self);
    fn all_txs(&self);
}

impl WalletInterface for Wall {
    fn get_balance(&self) -> anyhow::Result<u64> {
        Ok(self.get_balance()?.get_total())
    }

    fn sync(&self) {
        todo!()
    }

    fn last_unusued_address(&self) -> anyhow::Result<wallet::AddressInfo> {
        Ok(self.get_address(AddressIndex::LastUnused)?)
    }

    fn create_tx(&self, recipient: String, amount: u128, fee_rate: f32) {
        todo!()
    }

    fn sign(&self) {
        todo!()
    }

    fn broadcast(&self) {
        todo!()
    }

    fn all_txs(&self) {
        todo!()
    }
}

// TODO:
fn _shamir_into_shares(_mnemonic: String) {}

fn get_db(name: &String) -> SqliteDatabase {
    // TODO: proper path and name
    let db_path: &Path = Path::new(&name);
    let db: SqliteDatabase = SqliteDatabase::new(db_path);
    db
}
