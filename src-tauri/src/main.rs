// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod cli;
mod db;
mod wallet;

use cli::{handle_gui_cli, handle_onetime_cli};
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
        //.invoke_handler(tauri::generate_handler![cw])
        .run(context)
        .expect("error while running tauri application");
    ExitCode::from(0)
}

//#[tauri::command]
//fn cw(name: String, password: String) -> String {
//    let password_hash = "todo";
//    //let (_wallet, mnemonic_words) = create_wallet(&name, &password_hash);
//    return "dfa".to_string(); //mnemonic_words
//}
