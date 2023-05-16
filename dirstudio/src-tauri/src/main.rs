#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
mod menu;
mod state;
use crate::menu::get_menu;
use crate::state::ServerSideState;
use tauri::Manager;
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use ::idm::IdmStream;
use rose_client::BindParameters;
use std::net::ToSocketAddrs;
use std::time::Duration;
use tokio::net::TcpSocket;
use tokio::time::sleep;
use x500::DirectoryAbstractService::{DirectoryBindArgument, _encode_DirectoryBindArgument};
use x500::DirectoryIDMProtocols::id_idm_dap;
use rose_transport::{RosePDU, ROSEReceiver, ROSETransmitter};
use x500_client::RoseStream;

#[derive(serde::Deserialize)]
enum Command {
    BindViaURL(String),
}

#[derive(Debug, serde::Serialize, Clone)]
enum Response {
    BindOutcome(String),
}

async fn bind_idm (address: &str) {
    // TODO: Based on URL prefix, dispatch into separate functions
    let mut addrs = "dsa01.root.mkdemo.wildboar.software:4632"
        .to_socket_addrs()
        .unwrap();
    let socket = TcpSocket::new_v4().unwrap();
    let stream = socket.connect(addrs.next().unwrap()).await.unwrap();

    let idm = IdmStream::new(stream);
    let mut rose = RoseStream::new(idm);
    let dba = DirectoryBindArgument::new(None, None, vec![]);
    let encoded_dba = _encode_DirectoryBindArgument(&dba).unwrap();
    let bytes_written = rose
        .write_bind(BindParameters {
            protocol_id: id_idm_dap(),
            timeout: 5,
            parameter: encoded_dba,
            calling_ae_title: None,
            calling_ap_invocation_identifier: None,
            calling_ae_invocation_identifier: None,
            called_ae_title: None,
            called_ap_invocation_identifier: None,
            called_ae_invocation_identifier: None,
            implementation_information: None,
        })
        .await
        .unwrap();
    sleep(Duration::new(5, 0)).await;
    assert!(bytes_written.gt(&0));
    tokio::time::timeout(Duration::from_millis(10000), async {
        while let Some(rose_pdu) = rose.read_pdu().await.unwrap() {
            match &rose_pdu {
                RosePDU::BindResult(_br) => {
                    return;
                }
                _ => panic!(),
            };
        }
    })
    .await
    .unwrap();
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
async fn bind_via_url (url: &str) {
    if url.to_lowercase().starts_with("idm://") {
        bind_idm(&url[6..]).await
    }
}

// TODO: https://rfdonnelly.github.io/posts/tauri-async-rust-process/
fn main() {

    let (async_proc_input_tx, async_proc_input_rx) = mpsc::channel(1);
    let (async_proc_output_tx, mut async_proc_output_rx) = mpsc::channel(1);

    tauri::Builder::default()
        .menu(get_menu())
        .manage(ServerSideState {
            inner: Mutex::new(async_proc_input_tx),
            sessions: vec![],
        })
        .invoke_handler(tauri::generate_handler![js2rs])
        .setup(|app| {
            tauri::async_runtime::spawn(async move {
                async_process_model(
                    async_proc_input_rx,
                    async_proc_output_tx,
                ).await
            });

            let app_handle = app.handle();
            tauri::async_runtime::spawn(async move {
                loop {
                    if let Some(output) = async_proc_output_rx.recv().await {
                        rs2js(output, &app_handle);
                    }
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![js2rs])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


fn rs2js<R: tauri::Runtime>(message: Response, manager: &impl Manager<R>) {
    manager
        .emit_all("rs2js", message)
        .unwrap();
}

#[tauri::command]
async fn js2rs(
    message: Command,
    state: tauri::State<'_, ServerSideState>,
) -> Result<(), String> {
    let async_proc_input_tx = state.inner.lock().await;
    async_proc_input_tx
        .send(message)
        .await
        .map_err(|e| e.to_string())
}

async fn async_process_model(
    mut input_rx: mpsc::Receiver<Command>,
    output_tx: mpsc::Sender<Response>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    while let Some(input) = input_rx.recv().await {
        match &input {
            Command::BindViaURL(url) => bind_via_url(url).await,
        };
        let output = Response::BindOutcome(String::from("asdf"));
        output_tx.send(output).await?;
    }
    Ok(())
}
