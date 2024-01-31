// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod cli;
mod db;
mod shamir;
mod wallet;

use cli::{handle_gui_cli, handle_onetime_cli};
use serde::{Serialize, Serializer};
use std::env::set_var;
use std::process::ExitCode;
use std::sync::Mutex;
use tauri::{self, Manager, State};
use wallet::{
    create_wallet, recover_wallet, recover_wallet_shamir, xprv_from_mnemonic, MyWallet,
    WalletInterface,
};

fn main() -> ExitCode {
    set_var("RUST_BACKTRACE", "1"); //NOTE: remove for production

    let context = tauri::generate_context!();
    if let Some(exit_code) = handle_onetime_cli(&context) {
        return ExitCode::from(exit_code);
    };

    tauri::Builder::default()
        .setup(|app| {
            handle_gui_cli(app);
            let window = app.get_window("main").unwrap();
            window.open_devtools();
            Ok(())
        })
        .manage::<StateContent>(Mutex::new(vec![]))
        .invoke_handler(tauri::generate_handler![
            load_wallet,
            cw,
            cw_shamir,
            rw,
            rw_shamir,
            get_wallet_names,
            get_wallet_data,
            get_transactions,
            send,
            fee_for_transaction,
            close_wallet,
            delete_wallet
        ])
        .run(context)
        .expect("error while running tauri application");
    ExitCode::from(0)
}

type StateContent = Mutex<Vec<(String, Mutex<MyWallet>)>>;

// Create wallet and return mnemonic
#[tauri::command]
fn cw(name: String, password: String, state: State<StateContent>) -> CommandResult<String> {
    let (wallet, mnemonic_words) = create_wallet(name.clone(), password)?;
    add_wallet_state(name, wallet, state);
    Ok(mnemonic_words)
}

#[tauri::command]
fn cw_shamir(
    name: String,
    password: String,
    treshold: u8,
    count: u8,
    state: State<StateContent>,
) -> CommandResult<Vec<String>> {
    let (wallet, mnemonic) = create_wallet(name.clone(), password)?;
    let xprv = xprv_from_mnemonic(&mnemonic)?;
    let shamir_mnemonic_shares = shamir::split(xprv, treshold, count)?;

    add_wallet_state(name, wallet, state);
    Ok(shamir_mnemonic_shares)
}

#[tauri::command]
fn load_wallet(name: String, password: String, state: State<StateContent>) -> CommandResult<()> {
    let wallet = db::retrieve_wallet(&name, &password)?;
    add_wallet_state(name, wallet, state);
    Ok(())
}

// Retrive wallet from mnemonic
#[tauri::command]
fn rw(
    name: String,
    password: String,
    mnemonic: String,
    state: State<StateContent>,
) -> CommandResult<()> {
    let wallet = recover_wallet(name.clone(), password, mnemonic)?;
    add_wallet_state(name, wallet, state);
    Ok(())
}

#[tauri::command]
fn rw_shamir(
    name: String,
    password: String,
    mnemonics: Vec<Vec<String>>,
    state: State<StateContent>,
) -> CommandResult<()> {
    let wallet = recover_wallet_shamir(name.clone(), password, mnemonics)?;
    add_wallet_state(name, wallet, state);
    Ok(())
}

#[tauri::command]
fn send(
    name: String,
    amount: u64,
    address: String,
    fee_rate: f32,
    state: State<StateContent>,
) -> CommandResult<()> {
    with_wallet(&name, state, |wallet| {
        wallet.send(address, amount, fee_rate)?;
        Ok(())
    })?
}

#[tauri::command]
fn fee_for_transaction(
    name: String,
    amount: u64,
    address: String,
    fee_rate: f32,
    state: State<StateContent>,
) -> CommandResult<u64> {
    with_wallet(&name, state, |wallet| {
        let (_pbst, details) = wallet.create_tx(address, amount, fee_rate)?;
        let fee = details.fee.unwrap(); // on Electrum never none
        Ok(fee)
    })?
}

#[derive(Serialize)]
struct TransactionData {
    tx_id: String,
    sender_address: String,
    sent: u64,
    received: u64,
    timestamp: u64,
}

// TODO: confirmed / confirmed time
#[derive(Serialize)]
struct WalletData {
    name: String,
    balance: u64,
    address: String,
}

#[tauri::command]
fn get_wallet_data(name: String, state: State<StateContent>) -> CommandResult<WalletData> {
    with_wallet(&name, state, |wallet| {
        wallet.synchronize()?;
        Ok(WalletData {
            name: name.clone(),
            balance: wallet.get_total_balance()?,
            address: wallet.last_used_address()?.to_string(),
        })
    })?
}

#[tauri::command]
fn get_transactions(
    name: String,
    state: State<StateContent>,
) -> CommandResult<Vec<TransactionData>> {
    with_wallet(&name, state, |wallet| {
        Ok(wallet
            .all_txs()?
            .iter()
            .map(|tx| TransactionData {
                tx_id: tx.txid.to_string(),
                sender_address: "TODO:".to_string(),
                sent: tx.sent,
                received: tx.received,
                timestamp: match &tx.confirmation_time {
                    Some(ct) => ct.timestamp,
                    None => 0,
                },
            })
            .collect())
    })?
}

#[tauri::command]
fn get_wallet_names() -> CommandResult<Vec<String>> {
    Ok(db::get_wallet_names()?)
}

#[tauri::command]
fn close_wallet(name: String, state: State<StateContent>) -> CommandResult<()> {
    let mut wallets = state.lock().unwrap();
    let index_to_remove = wallets
        .iter()
        .position(|(w_name, _)| *w_name == name)
        .ok_or("Can't close wallet")?;
    wallets.remove(index_to_remove);
    Ok(())
}

#[tauri::command]
fn delete_wallet(name: String, state: State<StateContent>) -> CommandResult<()> {
    db::delete_wallet(name.clone())?;
    close_wallet(name, state)?;
    Ok(())
}

fn add_wallet_state(name: String, wallet: MyWallet, state: State<StateContent>) {
    let mut wallets = state.lock().unwrap();
    wallets.push((name, Mutex::new(wallet)));
}

fn with_wallet<F, R>(name: &String, state: State<StateContent>, callback: F) -> CommandResult<R>
where
    F: FnOnce(&MyWallet) -> R,
{
    let wallets = state.lock().unwrap();
    let wallet = wallets
        .iter()
        .find(|(n, _)| n.eq(name))
        .ok_or("Could not find wallet in current state")?
        .1
        .lock()
        .unwrap();
    Ok(callback(&wallet))
}

// create the error type that represents all errors possible in our program
#[derive(Debug, thiserror::Error)]
pub enum CommandError {
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
    #[error("Error: {0}")]
    GeneralError(String),
}

impl From<&str> for CommandError {
    fn from(err: &str) -> Self {
        CommandError::GeneralError(err.to_string())
    }
}

impl From<String> for CommandError {
    fn from(err: String) -> Self {
        CommandError::GeneralError(err)
    }
}

// we must manually implement serde::Serialize
impl Serialize for CommandError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

pub type CommandResult<T, E = CommandError> = anyhow::Result<T, E>;
