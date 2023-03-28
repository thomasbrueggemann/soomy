use yew::prelude::*;
use yew_router::prelude::*;

use crate::{images::get_all_image_urls, route::Route};

#[function_component]
pub fn Overview() -> Html {
    let thumbnails = get_all_image_urls();

    html! {
        <section class="section">
            <div class="container is-fluid">
                <div class="tile is-ancestor"> {
                    thumbnails.into_iter().map(|thumbnail| {
                        html! {
                            <div key={thumbnail.category.clone()} class="tile">
                                <Link<Route> to={Route::Detail{ image: thumbnail.category.clone()}}>
                                    <img src={thumbnail.image_url} class="p-6" />
                                </Link<Route>>
                            </div>
                        }
                    }).collect::<Html>()
                }
                </div>
            </div>
        </section>
    }
}
