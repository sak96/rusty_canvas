use crate::views::the_canvas::TheCanvas;
use crate::views::the_sidebar::TheSidebar;
use crate::views::the_toolbar::TheToolbar;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div style="min-height: 100vh; display: flex;">
            <TheToolbar />
            <TheCanvas />
            <TheSidebar />
        </div>
    }
}
