mod events;
mod shapes;
mod tools;

use tools::ToolAction;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let canvas_ref = use_node_ref();
    let cur_tool = use_state_eq(|| 0);
    let event_handler = use_mut_ref(|| events::EventHandler::new(canvas_ref.clone()));

    let on_pointer_event = {
        let event_handler = event_handler.clone();
        let cur_tool = cur_tool.clone();
        Callback::from(move |event| {
            event_handler
                .borrow_mut()
                .handle_ptr_event(event, *cur_tool)
        })
    };

    let onresize = {
        let event_handler = event_handler.clone();
        Callback::from(move |_| event_handler.borrow().reset_canvas())
    };

    html! {
        <div style="min-height: 100vh; display: flex;">
            <div style=r#"
                position: absolute;
                left: 50%;
                margin-left: -20px;
                top: 0;
            "#>
            {
                event_handler.borrow().all_tools().iter().enumerate().map(|(i,tool)|{
                    let color = if i == *cur_tool {"border: 2px solid blue ;"} else {""};
                    html!{
                    <button
                        class={classes!("ti", tool.button_icon())}
                        style={color}
                        ~title={tool.button_title()}
                        ~onclick={
                            let cur_tool = cur_tool.clone();
                            Callback::from(move |_| cur_tool.set(i))
                        }
                    />
                }}).collect::<Html>()
            }
            </div>
            <canvas
                style="flex: 1"
                ref={canvas_ref}
                onpointerup={on_pointer_event.clone()}
                onpointerdown={on_pointer_event.clone()}
                onpointermove={on_pointer_event.clone()}
                {onresize}
            />
        </div>
    }
}
