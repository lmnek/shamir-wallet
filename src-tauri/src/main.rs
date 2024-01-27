// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod cli;
mod db;
mod wallet;

use cli::{handle_gui_cli, handle_onetime_cli};
use wallet::{create_wallet, init_wallet};
use std::env::set_var;
use std::process::ExitCode;
use tauri;

fn main() -> ExitCode {
    set_var("RUST_BACKTRACE", "1"); //NOTE: remove for production 

    let context = tauri::generate_context!();
    if let Some(exit_code) = handle_onetime_cli(&context) {
        return ExitCode::from(exit_code);
    };

    tauri::Builder::default()
        .setup(|app| {
            handle_gui_cli(app);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![load_wallet, cw, get_wallet_names])
        .run(context)
        .expect("error while running tauri application");
    ExitCode::from(0)
}

#[tauri::command]
fn cw(name: String, password: String) -> Result<String, String> {
    match create_wallet(name, password) {
        Ok((wallet, mnemonic_words)) => {
            // TODO: save wallet state
            Ok(mnemonic_words)
        },
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
fn load_wallet(name: String, password: String) -> Result<(), String> {
    match db::retrieve_wallet(&name, &password) {
        Ok(wallet) => { 
            // save state
            Ok(())
        },
        Err(err) => Err(err.to_string()),
    }
}


#[tauri::command]
fn get_wallet_names() -> Result<Vec<String>, String>{
    db::get_wallet_names().map_err(|err| err.to_string())
}
