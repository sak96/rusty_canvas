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
            box-shadow: 7px 0px 14px 0px rgb(142, 142, 142);
        "#>
            // box-shadow: 0 0 0 2em #f4aab9, 0 0 0 4em #66ccff;
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
