#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
pub mod api;
pub mod menu;
pub mod state;
use std::collections::HashMap;
use std::sync::{Arc};
use tauri::async_runtime::Mutex;

use crate::api::{
    bind::bind,
    copy_to_clipboard::copy_to_clipboard,
    list::list,
    read::read,
};
use crate::menu::get_menu;
use crate::state::ServerSideState;

fn main() {
    tauri::Builder::default()
        .menu(get_menu())
        .manage(ServerSideState {
            sessions: Arc::new(Mutex::new(HashMap::new())),
        })
        // It seems like you add the handlers in one call. If you call this
        // multiple times, newer calls overwrite older ones.
        .invoke_handler(tauri::generate_handler![
            bind,
            copy_to_clipboard,
            list,
            read,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
