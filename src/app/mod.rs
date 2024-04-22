mod canvas;
mod shapes;
mod tools;
use web_sys::HtmlCanvasElement;
use yew::prelude::*;

use crate::app::{canvas::get_event_canvas_postion, shapes::Draw};

macro_rules! move_to_current_scope {
    ($( $i:ident ),*) => {
        $(let $i = $i.clone();)*
    };
}

macro_rules! handle_canvas_by_tool {
    ($onevent:ident, $canvas_ref:expr, $shapes:expr, $tools:expr, $cur_tool:expr) => {
        let $onevent = {
            let canvas_ref = $canvas_ref.clone();
            let shapes = $shapes.clone();
            let tools = $tools.clone();
            let cur_tool = $cur_tool.clone();
            Callback::from(move |event: MouseEvent| {
                let mut tools = tools.borrow_mut();
                if *cur_tool > tools.len() {
                    cur_tool.set(0)
                }
                let tool = &mut tools[*cur_tool];
                let canvas: HtmlCanvasElement = canvas_ref.cast::<HtmlCanvasElement>().unwrap();
                let position = get_event_canvas_postion(canvas.clone(), event);
                if tool.$onevent(position, canvas.clone(), &mut shapes.borrow_mut()) {
                    let interface = canvas::refresh_canvas(canvas, &shapes.borrow());
                    tool.draw_extra_shapes(&interface);
                }
            })
        };
    };
}

#[function_component(App)]
pub fn app() -> Html {
    let tools: Vec<Box<dyn tools::Tool>> =
        vec![Box::<tools::rectangle_tool::RectangleTool>::default()];
    let tools = use_mut_ref(|| tools);
    let canvas_ref = use_node_ref();
    let shapes = use_mut_ref(Vec::<Box<dyn Draw>>::new);
    let cur_tool = use_state_eq(|| 0);

    handle_canvas_by_tool! {onmousedown, canvas_ref, shapes, tools, cur_tool};
    handle_canvas_by_tool! {onmouseup, canvas_ref, shapes, tools, cur_tool};
    handle_canvas_by_tool! {onmousemove, canvas_ref, shapes, tools, cur_tool};
    handle_canvas_by_tool! {onmouseleave, canvas_ref, shapes, tools, cur_tool};

    let onresize = {
        move_to_current_scope!(canvas_ref, shapes);
        Callback::from(move |_| {
            let canvas: HtmlCanvasElement = canvas_ref.cast::<HtmlCanvasElement>().unwrap();
            canvas::refresh_canvas(canvas, &shapes.borrow());
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
            <canvas
                style="flex: 1"
                ref={canvas_ref}
                {onmouseup}
                {onmousemove}
                {onmousedown}
                {onmouseleave}
                {onresize}
                />
        </div>
    }
}
