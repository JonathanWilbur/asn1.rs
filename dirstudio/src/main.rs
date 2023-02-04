mod app;
mod ipc;
mod navtree;
mod tree;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
