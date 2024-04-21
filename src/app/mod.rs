mod canvas;
mod toolbar;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let tool = use_state_eq(|| &toolbar::TOOLS[0]);
    let ontoolchange = {
        let tool = tool.clone();
        Callback::from(move |new_tool| tool.set(new_tool))
    };
    html! {
        <div>
            <toolbar::Toolbar {ontoolchange} />
            <canvas::DrawingCanvas />
        </div>
    }
}
