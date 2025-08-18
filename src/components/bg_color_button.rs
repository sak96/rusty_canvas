use crate::components::base_button::BaseButton;
use crate::store::AppState;
use crate::types::colors::BackgroundColor;
use crate::types::shapes::Shape;
use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Properties, PartialEq)]
pub struct BackgroundColorProps {
    pub bg_color: Option<BackgroundColor>,
    #[prop_or("ti-square-filled".into())]
    pub icons: String,
}

#[function_component(BackgroundColorButton)]
pub fn bg_color_button(BackgroundColorProps { bg_color, icons }: &BackgroundColorProps) -> Html {
    let dispatch = use_dispatch::<AppState>();
    let current_bg_color = use_selector(|app: &AppState| app.get_bg_color().clone());
    let color = bg_color.clone().map_or("black".into(), |c| c.to_string());
    html! {
        <BaseButton
            selected={bg_color.eq(&*current_bg_color)}
            onclick={
                let bg_color = bg_color.clone();
                let dispatch = dispatch.clone();
                dispatch.reduce_mut_callback_with(move |app,  _| {
                    app.set_bg_color(bg_color.clone());
                    app.modify_selected(|x: &mut Shape| x.set_bg_color(bg_color.clone()));
                })
            }
        >
            <i
                style={format!("color: {}", color)}
                class={classes!("ti", icons)}
            />
        </BaseButton>
    }
}
