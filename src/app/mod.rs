mod canvas;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <canvas::DrawingCanvas />
    }
}
