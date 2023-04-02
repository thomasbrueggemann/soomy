use web_sys::HtmlElement;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{images::get_image_url, pan_zoom::pan_zoom, route::Route};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub image: String,
}

#[function_component]
pub fn Detail(props: &Props) -> Html {
    let div_ref = use_node_ref();

    {
        let div_ref = div_ref.clone();

        use_effect_with_deps(
            |div_ref| {
                let div = div_ref
                    .cast::<HtmlElement>()
                    .expect("div_ref not attached to div element");

                pan_zoom(div);

                move || {}
            },
            div_ref,
        );
    }

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
            <div ref={div_ref}>
                <img src={get_image_url(props.image.to_owned())} />
            </div>
        </>
    }
}
