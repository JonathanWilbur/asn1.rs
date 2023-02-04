use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

// This trick is undocumented completely, but I found it here:
// https://github.com/tauri-apps/plugins-workspace/issues/35#issuecomment-1356443521
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    pub fn invoke(cmd: &str, args: JsValue) -> JsValue;
}
