#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
pub mod api;
pub mod menu;
pub mod state;
use std::collections::HashMap;

use crate::api::{bind::bind, list::list, read::read};
use crate::menu::get_menu;
use crate::state::ServerSideState;

fn main() {
    tauri::Builder::default()
        .menu(get_menu())
        .manage(ServerSideState {
            sessions: HashMap::new(),
        })
        // It seems like you add the handlers in one call. If you call this
        // multiple times, newer calls overwrite older ones.
        .invoke_handler(tauri::generate_handler![
            bind,
            list,
            read,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
