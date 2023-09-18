mod api;
mod app;
mod components;
mod pages;
mod router;
mod store;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<app::App>::new().render();
}
