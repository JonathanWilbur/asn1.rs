use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

// This trick is undocumented completely, but I found it here:
// https://github.com/tauri-apps/plugins-workspace/issues/35#issuecomment-1356443521
// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
//     pub fn invoke(cmd: &str, args: JsValue) -> JsValue;
// }

// See: https://dev.to/stevepryde/create-a-desktop-app-in-rust-using-tauri-and-yew-2bhe

// This implmentation just marks invoke as async: https://github.com/spyglass-search/spyglass/blob/37601bfcbf2406360b8f80d763e5d3e9a9e43218/crates/client/src/main.rs#L21

// This one does too: https://github.com/eliaperantoni/spaceman/blob/03539e51e7f18903239c3672c614687e2c5a9a68/spaceman_gui_front/src/glue.rs#L7

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"], catch)]
    pub async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CopyToClipboardCommand {
    pub text: String,
}
