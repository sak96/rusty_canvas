use crate::components::base_button::BaseButton;
use crate::store::AppState;
use crate::types::colors::Color;
use crate::types::shapes::Shape;
use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ColorProps {
    pub color: Color,
}

#[function_component(ColorButton)]
pub fn color_button(ColorProps { color }: &ColorProps) -> Html {
    let dispatch = use_dispatch::<AppState>();
    let current_color = use_selector(|app: &AppState| app.get_color().clone());
    html! {
        <BaseButton
            selected={color.eq(&*current_color)}
            title={color.to_string()}
            onclick={
                let color = color.clone();
                let dispatch = dispatch.clone();
                dispatch.reduce_mut_callback_with(move |app,  _| {
                    app.set_color(color.clone());
                    app.modify_selected(|x: &mut Shape| x.set_color(color.clone()));
                })
            }
        >
            <i style={format!("color: {}", color)} class={classes!("ti", "ti-square-filled")} />
        </BaseButton>
    }
}
