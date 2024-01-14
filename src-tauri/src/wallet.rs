use crate::db::save_wallet;
use bdk::bitcoin::psbt::PartiallySignedTransaction;
use bdk::bitcoin::{Network, Transaction};
use bdk::blockchain::Blockchain;
use bdk::blockchain::ElectrumBlockchain;
use bdk::database::SqliteDatabase;
use bdk::electrum_client::Client;
use bdk::wallet::AddressInfo;
use bdk::{
    bitcoin::util::bip32::ExtendedPrivKey,
    bitcoin::Address,
    keys::{
        bip39::{Language, Mnemonic, WordCount},
        DerivableKey, ExtendedKey, GeneratableKey, GeneratedKey,
    },
    template::Bip84,
    wallet::AddressIndex,
};
use bdk::{miniscript, KeychainKind, SignOptions, Wallet, SyncOptions, TransactionDetails};
use core::f32;
use std::path::Path;
use std::str::FromStr;

static NETWORK: Network = Network::Testnet;

pub type Descriptor = Bip84<ExtendedPrivKey>;

pub fn create_wallet(name: &String, password: &String) -> (MyWallet, String) {
    let mnemonic: GeneratedKey<_, miniscript::Segwitv0> =
        Mnemonic::generate((WordCount::Words12, Language::English)).unwrap();
    let mnemonic_words = mnemonic.to_string();
    let mnemonic = Mnemonic::parse(&mnemonic_words).unwrap();
    let xkey: ExtendedKey = mnemonic.into_extended_key().unwrap();
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
) -> MyWallet {
    MyWallet {
        wallet: Wallet::new(descriptor, Some(change_descriptor), NETWORK, get_db(name)).unwrap(),
        blockchain: None
    }
}

pub fn initialize_wallet_strings(
    name: &String,
    descriptor: &String,
    change_descriptor: &String,
) -> MyWallet {
    MyWallet {
        wallet: Wallet::new(descriptor, Some(change_descriptor), NETWORK, get_db(name)).unwrap(),
        blockchain: None
    }
}

// TODO:
pub fn _recover_wallet(recovery_phrase: String) {
    let _mnemonic = Mnemonic::from_str(&recovery_phrase);
    todo!()
}

pub struct MyWallet {
    wallet: Wallet<SqliteDatabase>,
    blockchain: Option<ElectrumBlockchain> 
}

impl MyWallet {
    pub fn get_balance(&self) -> anyhow::Result<u64> {
        let balance = self.wallet.get_balance()?;
        Ok( balance.get_total() )
    }

    pub fn sync(&self) {
        // NOTE: syncoptions - progress
        let client = Client::new("ssl://electrum.blockstream.info:60002").unwrap();
        let blockchain = ElectrumBlockchain::from(client);
        self.wallet.sync(&blockchain, SyncOptions::default()).unwrap();
        // HACK: deal with blockchain
        //self.blockchain = Some(blockchain);
    }

    pub fn last_unusued_address(&self) -> anyhow::Result<Address> {
        let AddressInfo { address, .. } = self.wallet.get_address(AddressIndex::LastUnused)?;
        Ok( address ) 
    }

    pub fn create_tx(&self, send_to: String, amount: u64, _fee_rate: f32) -> (PartiallySignedTransaction, TransactionDetails)  {
        let dest_script = Address::from_str(send_to.as_str()).unwrap().script_pubkey();
        // psbt = partially signed transaction 
        let (psbt, details) = {
            let mut builder = self.wallet.build_tx();
            builder
                .add_recipient(dest_script, amount)
                .enable_rbf()
                //.include_output_redeem_witness_script()
                .do_not_spend_change();
                //.fee_rate(FeeRate::from_sat_per_vb(fee_rate));
            builder.finish().unwrap()
        };

        println!("Transaction details: {:#?}", details);
        println!("Unsigned PSBT: {}", &psbt);

        (psbt, details)
    }

    pub fn sign(&self, mut psbt: PartiallySignedTransaction) -> anyhow::Result<Transaction> {
        let finalized = self.wallet.sign(&mut psbt, SignOptions::default())?;
        if !finalized {
            let _psbt_is_finalized = self.wallet.finalize_psbt(&mut psbt, SignOptions::default())?;
        }
        let tx = psbt.extract_tx();
        Ok( tx )
    }

    pub fn broadcast_tx(&self, tx:Transaction) -> Result<(), bdk::Error> {
        let client = Client::new("ssl://electrum.blockstream.info:60002").unwrap();
        let blockchain = ElectrumBlockchain::from(client);
        blockchain.broadcast(&tx)
    }

    pub fn send(&self, send_to: String, amount: u64) {
        let (pbst, _details) = self.create_tx(send_to, amount, 10.0);
        let tx = self.sign(pbst).unwrap();
        self.broadcast_tx(tx).unwrap();
    }

    pub fn all_txs(&self) -> Result<Vec<TransactionDetails>, bdk::Error> {
        self.wallet.list_transactions(false)
    }

    pub fn get_descritors(&self) -> (String, String) {
        let desc = self.wallet 
            .get_descriptor_for_keychain(KeychainKind::External)
            .to_string();
        let change_desc = self.wallet
            .get_descriptor_for_keychain(KeychainKind::Internal)
            .to_string();
        (desc, change_desc)
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
