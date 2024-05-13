use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct BaseButtonProps {
    pub children: Html,
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub title: String,
    #[prop_or_default]
    pub selected: bool,
}

#[function_component(BaseButton)]
pub fn base_button(
    BaseButtonProps {
        children,
        onclick,
        selected,
        title,
    }: &BaseButtonProps,
) -> Html {
    let mut style = r#"
      background-color: white;
      border: none;
      padding: 5px 5px;
      text-align: center;
      text-decoration: none;
      display: inline-block;
      cursor: pointer;
      font-size: 20px;
    "#
    .to_string();
    let title = title.clone();
    if *selected {
        style.push_str("background-color: lightblue;");
    }
    html! {
        <button {style} {title} {onclick}>{children.clone()}</button>
    }
}
