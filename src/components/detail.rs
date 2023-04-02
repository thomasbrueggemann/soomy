use yew::prelude::*;
use yew_router::prelude::*;

use crate::{images::get_image_url, route::Route, zoomist::Zoomist};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub image: String,
}

#[function_component]
pub fn Detail(props: &Props) -> Html {
    use_effect(move || {
        Zoomist::new("#photo");
    });

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
            <div id="photo" data-zoomist-src={get_image_url(props.image.to_owned())}></div>
        </>
    }
}
