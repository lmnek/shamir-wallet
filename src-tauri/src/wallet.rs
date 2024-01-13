use bdk::{keys::{
    bip39::{Language, Mnemonic, WordCount},
    DerivableKey, ExtendedKey, GeneratableKey, GeneratedKey,
}, template::Bip84, bitcoin::util::bip32::ExtendedPrivKey};
use bdk::database::SqliteDatabase;
use bdk::bitcoin::Network;
use bdk::{miniscript, KeychainKind, Wallet};
use std::path::Path;

pub type Wall = Wallet<SqliteDatabase>;
pub type Descriptor = Bip84<ExtendedPrivKey>;

pub fn create_wallet(name: &String, password: &String) -> (Wall, String) {
    let network = Network::Testnet; 
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
    let xprv = xkey.into_xprv(network).unwrap();
    // Create a BDK wallet structure using BIP 84 descriptor ("m/84h/1h/0h/0" and "m/84h/1h/0h/1")
    let descriptor = Bip84(xprv, KeychainKind::External);
    let change_descriptor = Bip84(xprv, KeychainKind::Internal);

    let wallet = initialize_wallet(name, descriptor, change_descriptor);

    
    (wallet, mnemonic_words)
}


pub fn initialize_wallet(name: &String, descriptor: Descriptor, change_descriptor: Descriptor) -> Wall {
    Wallet::new(descriptor, Some(change_descriptor), Network::Testnet, get_db(name)).unwrap()
}

pub fn initialize_wallet_strings(name: &String, descriptor: &String, change_descriptor: &String) -> Wallet<SqliteDatabase> {
    Wallet::new(descriptor, Some(change_descriptor), Network::Testnet, get_db(name)).unwrap()
}

pub fn get_balance(wallet: Wallet<SqliteDatabase>) -> Result<bdk::Balance, bdk::Error> {
    wallet.get_balance()
}

// TODO:
fn _shamir_into_shares(_mnemonic: String) {}

fn get_db(name: &String) -> SqliteDatabase {
    // TODO: proper path and name
    let db_path: &Path = Path::new(&name);
    let db: SqliteDatabase = SqliteDatabase::new(db_path);
    db
}


