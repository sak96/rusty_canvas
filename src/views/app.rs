use crate::components::base_button::BaseButton;
use crate::store::tools::Tools;
use crate::types::tools::{Tool, ToolAction};
use crate::views::the_canvas::TheCanvas;
use strum::IntoEnumIterator;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let (tool_state, tool_dispatch) = use_store::<Tools>();
    html! {
        <div style="min-height: 100vh; display: flex;">
            <div style=r#"
                position: absolute;
                left: 50%;
                margin-left: -20px;
                top: 0;
            "#>
            {{
                let cur_tool = tool_state.tool.to_string();
                Tool::iter().map(|tool|{
                    html!{
                    <BaseButton
                        selected={tool.to_string().eq(&cur_tool)}
                        title={tool.button_title()}
                        onclick={
                            let tool_dispatch = tool_dispatch.clone();
                            let tool = tool.clone();
                            tool_dispatch.reduce_mut_callback_with(move |tools,  _| {
                                tools.tool = tool.clone();
                            })
                        }>
                        <i class={classes!("ti", tool.button_icon())} />
                    </BaseButton>
                }}).collect::<Html>()
            }}
            </div>
            <TheCanvas />
        </div>
    }
}
