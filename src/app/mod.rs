mod canvas;
mod shapes;
mod tools;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let tools: Vec<Box<dyn tools::Tool>> = vec![Box::<tools::RectangleTool>::default()];
    let tools = use_mut_ref(|| tools);
    let cur_tool = use_state_eq(|| 0);
    html! {
        <div style="min-height: 100vh; display: flex;">
            <div style=r#"
                position: absolute;
                left: 50%;
                margin-left: -20px;
                top: 0;
            "#>
            {
                tools.borrow().iter().enumerate().map(|(i,tool)|{html!{
                    <button
                        ~innerText={tool.button_icon()}
                        ~title={tool.button_title()}
                        ~onclick={
                            let cur_tool = cur_tool.clone();
                            Callback::from(move |_| cur_tool.set(i))
                        }
                    />
                }}).collect::<Html>()
            }
            </div>
            <canvas::DrawingCanvas />
        </div>
    }
}
