mod canvas;
mod shapes;
mod tools;
use web_sys::HtmlCanvasElement;
use yew::prelude::*;

use crate::app::{canvas::get_event_canvas_postion, shapes::Shape, tools::ToolAction};

pub struct EventHandler {
    canvas_ref: NodeRef,
    tools: Vec<tools::Tool>,
    shapes: Vec<Shape>,
}

impl EventHandler {
    fn new(canvas_ref: NodeRef) -> Self {
        Self {
            canvas_ref,
            tools: vec![
                tools::select_tool::SelectTool::default().into(),
                tools::shape_tool::ShapeTool::new(
                    "ti-square",
                    "Rectangle drawing tool.",
                    shapes::Rectangle::default().into(),
                )
                .into(),
                tools::shape_tool::ShapeTool::new(
                    "ti-circle",
                    "Ellipse drawing tool.",
                    shapes::Ellipse::default().into(),
                )
                .into(),
            ],
            shapes: vec![],
        }
    }
    fn get_canvas(&self) -> HtmlCanvasElement {
        self.canvas_ref.cast::<HtmlCanvasElement>().unwrap()
    }
    fn refresh_canvas(&self) {
        canvas::refresh_canvas(self.get_canvas(), &self.shapes, None);
    }
    fn handle_ptr_event(&mut self, event: PointerEvent, mut tool_idx: usize) {
        if tool_idx >= self.tools.len() {
            tool_idx = 0;
        }
        let canvas = self.get_canvas();
        let tool = &mut self.tools[tool_idx];
        let position = get_event_canvas_postion(&canvas, &event);
        match event.type_().as_str() {
            "pointerdown" => {
                canvas.set_pointer_capture(event.pointer_id()).unwrap();
                if tool.onmousedown(position, canvas.clone(), &mut self.shapes) {
                    canvas::refresh_canvas(canvas, &self.shapes, Some(tool));
                }
            }
            "pointerup" => {
                canvas.release_pointer_capture(event.pointer_id()).unwrap();
                if tool.onmouseup(position, canvas.clone(), &mut self.shapes) {
                    canvas::refresh_canvas(canvas, &self.shapes, Some(tool));
                }
            }
            "pointermove" => {
                if tool.onmousemove(position, canvas.clone(), &mut self.shapes) {
                    canvas::refresh_canvas(canvas, &self.shapes, Some(tool));
                }
            }
            "pointerleave" => {
                if tool.onmouseleave(position, canvas.clone(), &mut self.shapes) {
                    canvas::refresh_canvas(canvas, &self.shapes, Some(tool));
                }
            }
            _ => {}
        }
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let canvas_ref = use_node_ref();
    let cur_tool = use_state_eq(|| 0);
    let event_handler = use_mut_ref(|| EventHandler::new(canvas_ref.clone()));

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
        Callback::from(move |_| event_handler.borrow().refresh_canvas())
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
                event_handler.borrow().tools.iter().enumerate().map(|(i,tool)|{
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
