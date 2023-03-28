use yew::prelude::*;

use crate::images::get_image_url;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub image: String,
}

#[function_component]
pub fn Detail(props: &Props) -> Html {
    html! {
        <div class="is-flex is-align-items-center is-justify-content-center" style="height:100%">
            <div class="is-flex is-align-items-center">
                <img src={get_image_url(props.image.to_owned())} />
            </div>
        </div>
    }
}
