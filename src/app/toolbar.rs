use yew::prelude::*;

#[derive(PartialEq)]
pub enum Tool {
    Rectangle,
    Select,
    Pan,
}

pub const TOOLS: [Tool; 3] = [Tool::Select, Tool::Rectangle, Tool::Pan];

impl Tool {
    pub fn text(&self) -> String {
        match self {
            Tool::Select => "select",
            Tool::Rectangle => "rectangle",
            Tool::Pan => "pan",
        }
        .into()
    }
}

#[derive(Properties, PartialEq)]
pub struct ToolbarProps {
    pub ontoolchange: Callback<&'static Tool>,
}

#[function_component(Toolbar)]
pub fn toolbar(ToolbarProps { ontoolchange }: &ToolbarProps) -> Html {
    html! {
        <div style=r#"
            margin: auto 0;
            position: relative;
            top: 0;
            text-align: center;
            border: black;
        "#>
        {
            TOOLS.iter().map(|tool| {
                let ontoolchange = ontoolchange.clone();
                html!{
                    <button ~innerText={tool.text()} onclick={move |_| {ontoolchange.emit(tool)}} />
                }
            }).collect::<Html>()
        }
        </div>
    }
}
