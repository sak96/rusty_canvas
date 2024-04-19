use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <h1 ~innerText="Hello World" />
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
