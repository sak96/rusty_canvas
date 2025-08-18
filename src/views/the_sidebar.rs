use crate::components::base_button::BaseButton;
use crate::store::AppState;
use crate::types::shapes::Shape;
use yew::prelude::*;
use yewdux::prelude::*;

const COLORS: [&str; 5] = ["black", "red", "green", "darkblue", "orange"];

#[function_component(TheSidebar)]
pub fn the_sidebar() -> Html {
    let dispatch = use_dispatch::<AppState>();
    let current_color = use_selector(|app: &AppState| app.get_color().clone());

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
            <div>
            {{
                COLORS.iter().map(|color| {
                    html!{
                        <BaseButton
                            selected={color.eq(&*current_color)}
                            title={*color}
                            onclick={
                                let dispatch = dispatch.clone();
                                dispatch.reduce_mut_callback_with(move |app,  _| {
                                    app.set_color(color);
                                    app.modify_selected(|x: &mut Shape| x.set_color(color));
                                })
                            }
                        >
                            <i style={format!("color: {}", color)} class={classes!("ti", "ti-square-filled")} />
                        </BaseButton>
                     }
                }).collect::<Html>()
            }}
            </div>
        </div>
    }
}
