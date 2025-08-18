use crate::components::bg_color_button::BackgroundColorButton;
use crate::components::color_button::ColorButton;
use crate::types::colors::{BackgroundColor, Color};
use strum::IntoEnumIterator;
use yew::prelude::*;

#[function_component(TheSidebar)]
pub fn the_sidebar() -> Html {
    html! {
        <div style=r#"
            position: absolute;
            left: 0;
            top: 10%;
            display: flex;
            flex-direction: column;
            justify-content: center;
            box-shadow: 0px 0px .9px 0px rgba(0, 0, 0, .17), 0px 0px 3.1px 0px rgba(0, 0, 0, .08), 0px 7px 14px 0px rgba(0, 0, 0, .05)
        "#>
            <i style="margin: 1px auto;">{"Stroke"}</i>
            <div>
            {{
                Color::iter().map(|color| html!{
                    <ColorButton {color} />
                 }).collect::<Html>()
            }}
            </div>
            <i style="margin: 1px auto;">{"Fill"}</i>
            <div>
            <BackgroundColorButton bg_color={None} icons={"ti-square"} />
            {{
                 BackgroundColor::iter().map(|bg_color| html!{
                     <BackgroundColorButton {bg_color} />
                 }).collect::<Html>()
             }}
            </div>
        </div>
    }
}
