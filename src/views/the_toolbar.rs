use crate::components::base_button::BaseButton;
use crate::store::AppState;
use crate::types::tools::{Tool, ToolAction};
use strum::IntoEnumIterator;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(TheToolbar)]
pub fn the_toolbar() -> Html {
    let dispatch = use_dispatch::<AppState>();
    let current_tool = use_selector(|app: &AppState| app.get_tool().clone());
    html! {
        <div style=r#"
            position: absolute;
            left: 50%;
            top: 0;
            box-shadow: 0px 0px .9px 0px rgba(0, 0, 0, .17), 0px 0px 3.1px 0px rgba(0, 0, 0, .08), 0px 7px 14px 0px rgba(0, 0, 0, .05)
        "#>
        {{
            Tool::iter().map(|tool|{
                html!{
                <BaseButton
                    selected={tool.eq(&current_tool)}
                    title={tool.button_title()}
                    onclick={
                        let dispatch = dispatch.clone();
                        let tool = tool.clone();
                        dispatch.reduce_mut_callback_with(move |app,  _| {
                            app.set_tool(tool.clone());
                        })
                    }>
                    <i class={classes!("ti", tool.button_icon())} />
                </BaseButton>
            }}).collect::<Html>()
        }}
        </div>
    }
}
