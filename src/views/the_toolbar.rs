use crate::components::base_button::BaseButton;
use crate::store::tools::Tools;
use crate::types::tools::{Tool, ToolAction};
use strum::IntoEnumIterator;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(TheToolbar)]
pub fn the_toolbar() -> Html {
    let (tool_state, tool_dispatch) = use_store::<Tools>();
    html! {
        <div style=r#"
            position: absolute;
            left: 50%;
            top: 0;
            box-shadow: 0px 0px .9px 0px rgba(0, 0, 0, .17), 0px 0px 3.1px 0px rgba(0, 0, 0, .08), 0px 7px 14px 0px rgba(0, 0, 0, .05)
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
    }
}
