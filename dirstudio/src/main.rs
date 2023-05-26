mod app;
mod components;
mod info3;
mod ipc;
mod tree;

use app::App;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
