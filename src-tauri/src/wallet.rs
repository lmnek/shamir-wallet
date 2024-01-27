use crate::db;
use bdk::bitcoin::psbt::PartiallySignedTransaction;
use bdk::bitcoin::util::bip32::ExtendedPrivKey;
use bdk::bitcoin::{Network, Transaction};
use bdk::blockchain::Blockchain;
use bdk::blockchain::ElectrumBlockchain;
use bdk::database::SqliteDatabase;
use bdk::electrum_client::Client;
use bdk::wallet::AddressInfo;
use bdk::{
    bitcoin::Address,
    keys::{
        bip39::{Language, Mnemonic, WordCount},
        DerivableKey, ExtendedKey, GeneratableKey, GeneratedKey,
    },
    template::Bip84,
    wallet::AddressIndex,
};
use bdk::{miniscript, KeychainKind, SignOptions, SyncOptions, TransactionDetails, Wallet};
use tauri::api::path;
use core::f32;
use std::path::Path;
use std::str::FromStr;
use anyhow::{Result, anyhow};

static NETWORK: Network = Network::Testnet;

pub type MyWallet = Wallet<SqliteDatabase>;

pub fn create_wallet(name: String, password: String) -> Result<(MyWallet, String)> {
    if db::get_wallet_names()?.contains(&name) {
        return Err(anyhow!("Wallet name already exists"));
    };

    let mnemonic: GeneratedKey<_, miniscript::Segwitv0> =
        Mnemonic::generate((WordCount::Words12, Language::English)).unwrap();
    let mnemonic_words = mnemonic.to_string();
    let mnemonic = Mnemonic::parse(&mnemonic_words).unwrap();
    let xkey: ExtendedKey = mnemonic.into_extended_key().unwrap();
    let xprv: ExtendedPrivKey = xkey.into_xprv(NETWORK).unwrap();
    let wallet = init_wallet(&name, xprv);

    // NOTE: saving encrypted private key to DB, possibly dangerous
    db::save_private_key(&name, &password, xprv).unwrap();
    Ok((wallet, mnemonic_words))
}

pub fn init_wallet(
    name: &String,
    xprv: ExtendedPrivKey
) -> MyWallet
{
    // Create a BDK wallet structure using BIP 84 descriptor ("m/84h/1h/0h/0" and "m/84h/1h/0h/1")
    let descriptor = Bip84(xprv, KeychainKind::External);
    let change_descriptor = Bip84(xprv, KeychainKind::Internal);
    let db = get_db(name.to_string());
    let wallet = Wallet::new(descriptor, Some(change_descriptor), NETWORK, db).unwrap();
    wallet
}


// TODO:
pub fn _recover_wallet(recovery_phrase: String) {
    let _mnemonic = Mnemonic::from_str(&recovery_phrase);
    todo!()
}

// TODO:
fn _shamir_into_shares(_mnemonic: String) {}

pub trait WalletInterface {
    fn get_balance(&self) -> anyhow::Result<u64>;
    fn synchronize(&self);
    fn last_used_address(&self) -> anyhow::Result<Address>;
    fn create_tx(
        &self,
        send_to: String,
        amount: u64,
        fee_rate: f32,
    ) -> (PartiallySignedTransaction, TransactionDetails);
    fn sign_tx(&self, psbt: PartiallySignedTransaction) -> anyhow::Result<Transaction>;
    fn broadcast_tx(&self, tx: Transaction) -> Result<(), bdk::Error>;
    fn send(&self, send_to: String, amount: u64);
    fn all_txs(&self) -> Result<Vec<TransactionDetails>, bdk::Error>;
    fn get_descritors(&self) -> (String, String);
}

impl WalletInterface for MyWallet {
    fn get_balance(&self) -> anyhow::Result<u64> {
        let balance = self.get_balance()?;
        Ok(balance.get_total())
    }

    fn synchronize(&self) {
        // NOTE: syncoptions - progress
        let client = Client::new("ssl://electrum.blockstream.info:60002").unwrap();
        let blockchain = ElectrumBlockchain::from(client);
        self.sync(&blockchain, SyncOptions::default()).unwrap();
        // HACK: deal with blockchain
        //self.blockchain = Some(blockchain);
    }

    fn last_used_address(&self) -> anyhow::Result<Address> {
        let AddressInfo { address, .. } = self.get_address(AddressIndex::LastUnused)?;
        Ok(address)
    }

    fn create_tx(
        &self,
        send_to: String,
        amount: u64,
        _fee_rate: f32,
    ) -> (PartiallySignedTransaction, TransactionDetails) {
        let dest_script = Address::from_str(send_to.as_str()).unwrap().script_pubkey();
        // psbt = partially signed transaction
        let (psbt, details) = {
            let mut builder = self.build_tx();
            builder
                .add_recipient(dest_script, amount)
                .enable_rbf();
                //.include_output_redeem_witness_script()
                //.do_not_spend_change();
            //.fee_rate(FeeRate::from_sat_per_vb(fee_rate));
            builder.finish().unwrap()
        };

        println!("Transaction details: {:#?}", details);
        println!("Unsigned PSBT: {}", &psbt);

        (psbt, details)
    }

    fn sign_tx(&self, mut psbt: PartiallySignedTransaction) -> anyhow::Result<Transaction> {
        let finalized = self.sign(&mut psbt, SignOptions::default())?;
        if !finalized {
            println!("Tx has not been finalized");
        }
        let tx = psbt.extract_tx();
        Ok(tx)
    }

    fn broadcast_tx(&self, tx: Transaction) -> Result<(), bdk::Error> {
        let client = Client::new("ssl://electrum.blockstream.info:60002")?;
        let blockchain = ElectrumBlockchain::from(client);
        println!("Broadcasting: {}", tx.txid());
        blockchain.broadcast(&tx)
    }

    fn send(&self, send_to: String, amount: u64) {
        let (pbst, _details) = self.create_tx(send_to, amount, 10.0);
        let tx = self.sign_tx(pbst).unwrap();
        match self.broadcast_tx(tx) {
            Ok(_) => println!("Transaction broadcasted successfully"),
            Err(e) => eprintln!("Error broadcasting transaction: {:?}", e),
        };
    }

    fn all_txs(&self) -> Result<Vec<TransactionDetails>, bdk::Error> {
        self.list_transactions(false)
    }

    fn get_descritors(&self) -> (String, String) {
        let desc = self
            .get_descriptor_for_keychain(KeychainKind::External)
            .to_string();
        let change_desc = self
            .get_descriptor_for_keychain(KeychainKind::Internal)
            .to_string();
        (desc, change_desc)
    }
}


fn get_db(name: String) -> SqliteDatabase {
    let data_path = path::data_dir().unwrap();
    let db_path = data_path.join("shamir_wallet").join(format!("{}.sqlite", name));
    let db: SqliteDatabase = SqliteDatabase::new(db_path);
    db
}

pub fn delete_wallet_db(name: &String) -> anyhow::Result<()>{
    let db_path: &Path = Path::new(&name);
    std::fs::remove_file(db_path)?;
    Ok(())
}
