use yew::prelude::*;
mod canvas;

#[function_component(App)]
fn app() -> Html {
    html! {
        <canvas::DrawingCanvas />
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
