use yew::{html,Html};
use yew::virtual_dom::{Classes,VNode,VTag,VText};

pub fn index(src: &str) -> Html {
    let div = yew::utils::document().create_element("div").unwrap();
    div.set_inner_html(src);
    Html::VRef(div.into())
}