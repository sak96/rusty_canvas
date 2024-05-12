use crate::{
    utils::tools::ToolAction,
    views::the_canvas::{EventHandler, TheCanvas},
};
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    // TODO: make event handler only for canvas
    let canvas_ref = use_node_ref();
    let event_handler = use_mut_ref(|| EventHandler::new(canvas_ref.clone()));
    let force_trigger = use_force_update();
    let set_tool_idx = {
        let event_handler = event_handler.clone();
        Callback::from(move |idx| {
            event_handler.borrow_mut().toolbar_mut().set_tool_idx(idx);
            force_trigger.force_update();
        })
    };
    html! {
        <div style="min-height: 100vh; display: flex;">
            <div style=r#"
                position: absolute;
                left: 50%;
                margin-left: -20px;
                top: 0;
            "#>
            {{
                let event_handler = event_handler.clone();
                let event_handler = event_handler.borrow();
                let tool_bar = event_handler.toolbar();
                tool_bar.all_tools().iter().enumerate().map(|(i,tool)|{
                    let color = if i == tool_bar.get_tool_idx() {"border: 2px solid blue ;"} else {""};
                    html!{
                    <button
                        class={classes!("ti", tool.button_icon())}
                        style={color}
                        ~title={tool.button_title()}
                        ~onclick={
                            let set_tool_idx = set_tool_idx.clone();
                            Callback::from(move |_| set_tool_idx.emit(i))
                        }
                    />
                }}).collect::<Html>()
            }}
            </div>
            <TheCanvas {event_handler}/>
        </div>
    }
}
