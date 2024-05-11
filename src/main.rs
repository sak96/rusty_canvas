mod components;
mod store;
mod types;
mod utils;
mod views;

fn main() {
    yew::Renderer::<views::app::App>::new().render();
}
