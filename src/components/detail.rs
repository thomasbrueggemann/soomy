use yew::prelude::*;

use crate::images::get_image_url;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub image: String,
}

#[function_component]
pub fn Detail(props: &Props) -> Html {
    html! {
    <>
        {"Detail "}
        {&props.image}
        <img src={get_image_url(props.image.to_owned())} />
    </> }
}
