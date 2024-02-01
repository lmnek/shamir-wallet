use crate::db;
use crate::shamir;
use anyhow::Context;
use bdk::bitcoin::psbt::PartiallySignedTransaction;
use bdk::bitcoin::util::bip32::ExtendedPrivKey;
use bdk::bitcoin::{Network, Transaction};
use bdk::blockchain::Blockchain;
use bdk::blockchain::ElectrumBlockchain;
use bdk::database::SqliteDatabase;
use bdk::electrum_client::Client;
use bdk::wallet::AddressInfo;
use bdk::FeeRate;
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
use std::path::PathBuf;
use std::str::FromStr;
use anyhow::{Result, anyhow};

static NETWORK: Network = Network::Testnet;

pub type MyWallet = Wallet<SqliteDatabase>;

pub fn create_wallet(name: String, password: String) -> Result<(MyWallet, String)> {
    if db::get_wallet_names()?.contains(&name) {
        return Err(anyhow!("Wallet with this name already exists"));
    };

    let mnemonic: GeneratedKey<_, miniscript::Segwitv0> =
        Mnemonic::generate((WordCount::Words12, Language::English)).unwrap();
    let mnemonic = mnemonic.to_string();
    let xprv = xprv_from_mnemonic(&mnemonic)?;
    let wallet = init_wallet(&name, xprv)?;

    // NOTE: saving encrypted private key to DB, possibly dangerous
    db::save_private_key(&name, &password, xprv)?;
    Ok((wallet, mnemonic))
}

pub fn xprv_from_mnemonic(mnemonic: &String) -> Result<ExtendedPrivKey> {
    let mnemonic = Mnemonic::parse(mnemonic)?;
    let xkey: ExtendedKey = mnemonic.into_extended_key()?;
    let xprv = xkey.into_xprv(NETWORK).context("Can't convert private key into extended")?;
    Ok(xprv)
}

pub fn init_wallet(
    name: &String,
    xprv: ExtendedPrivKey
) -> Result<MyWallet>
{
    // Create a BDK wallet structure using BIP 84 descriptor ("m/84h/1h/0h/0" and "m/84h/1h/0h/1")
    let descriptor = Bip84(xprv, KeychainKind::External);
    let change_descriptor = Bip84(xprv, KeychainKind::Internal);
    let db = get_db(name)?;
    let wallet = Wallet::new(descriptor, Some(change_descriptor), NETWORK, db)?;
    Ok(wallet)
}


pub fn recover_wallet(name: String, password: String, mnemonic: String) -> Result<MyWallet> {
    let xprv = xprv_from_mnemonic(&mnemonic)?;
    let wallet = init_wallet(&name, xprv)?;
    db::save_private_key(&name, &password, xprv)?;
    Ok(wallet)
}

pub fn recover_wallet_shamir(name: String, password: String, mnemonics: Vec<Vec<String>>) -> Result<MyWallet> {
    let xprv = shamir::combine(mnemonics)?;
    let wallet = init_wallet(&name, xprv)?;
    db::save_private_key(&name, &password, xprv)?;
    Ok(wallet)
}

pub trait WalletInterface {
    fn get_total_balance(&self) -> Result<u64>;
    fn synchronize(&self) -> Result<()>;
    fn last_used_address(&self) -> Result<Address>;
    fn create_tx(
        &self,
        send_to: String,
        amount: u64,
        fee_rate: f32,
    ) -> Result<(PartiallySignedTransaction, TransactionDetails)>;
    fn sign_tx(&self, psbt: PartiallySignedTransaction) -> Result<Transaction>;
    fn broadcast_tx(&self, tx: Transaction) -> Result<(), bdk::Error>;
    fn send(&self, send_to: String, amount: u64, fee_rate: f32) -> Result<()>;
    fn all_txs(&self) -> Result<Vec<TransactionDetails>>;
    fn get_descritors(&self) -> (String, String);
}

impl WalletInterface for MyWallet {
    fn get_total_balance(&self) -> Result<u64> {
        let balance = self.get_balance()?;
        Ok(balance.get_total())
    }

    fn synchronize(&self) -> Result<()> {
        // NOTE: SyncOptions - progress
        let client = Client::new("ssl://electrum.blockstream.info:60002")?;
        let blockchain = ElectrumBlockchain::from(client);
        self.sync(&blockchain, SyncOptions::default())?;
        Ok(())
    }

    fn last_used_address(&self) -> Result<Address> {
        let AddressInfo { address, .. } = self.get_address(AddressIndex::LastUnused)?;
        Ok(address)
    }

    fn create_tx(
        &self,
        send_to: String,
        amount: u64,
        fee_rate: f32,
    ) -> Result<(PartiallySignedTransaction, TransactionDetails)> {
        let dest_script = Address::from_str(send_to.as_str())
            .map_err(|_| anyhow!("Invalid address"))?.script_pubkey();
        // psbt = partially signed transaction
        let (psbt, details) = {
            let mut builder = self.build_tx();
            builder
                .add_recipient(dest_script, amount)
                .fee_rate(FeeRate::from_sat_per_vb(fee_rate));
            builder.finish()?
        };
        Ok((psbt, details))
    }

    fn sign_tx(&self, mut psbt: PartiallySignedTransaction) -> Result<Transaction> {
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

    fn send(&self, send_to: String, amount: u64, fee_rate: f32) -> Result<()> {
        let (pbst, _details) = self.create_tx(send_to, amount, fee_rate)?;
        let tx = self.sign_tx(pbst)?;
        self.broadcast_tx(tx)?;
        Ok(())
    }

    fn all_txs(&self) -> Result<Vec<TransactionDetails>> {
        Ok(self.list_transactions(false)?)
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

pub fn delete_wallet_db(name: &String) -> Result<()>{
    let db_path = get_db_path(name)?;
    std::fs::remove_file(db_path)?;
    Ok(())
}

fn get_db(name: &String) -> Result<SqliteDatabase> {
    let db_path = get_db_path(name)?;
    let db: SqliteDatabase = SqliteDatabase::new(db_path);
    Ok(db)
}

fn get_db_path(name: &String) -> Result<PathBuf> {
    let data_path = path::data_dir().context("Can't access systems data directory")?;
    let db_path = data_path.join("shamir_wallet").join(format!("{}.sqlite", name));
    Ok(db_path)
}



