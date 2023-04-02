use yew::prelude::*;
use yew_router::prelude::*;

use crate::{images::get_image_url, route::Route};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub image: String,
}

#[function_component]
pub fn Detail(props: &Props) -> Html {
    html! {
        <>
            <div class="home-nav">
                <Link<Route> to={Route::Overview}>
                    <button class="button is-primary is-large">
                        <span class="icon">
                            <i class="fas fa-home"></i>
                        </span>
                    </button>
                </Link<Route>>
            </div>
            <div class="is-flex is-align-items-center is-justify-content-center" style="height:100%">
                <div class="is-flex is-align-items-center">
                    <img src={get_image_url(props.image.to_owned())} />
                </div>
            </div>
        </>
    }
}
