use yew::prelude::*;
use yew_router::prelude::*;

use crate::{images::get_all_image_urls, route::Route};

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

#[function_component]
pub fn Overview() -> Html {
    let thumbnails = get_all_image_urls();

    html! {
        <>
            <p>{ "Overview" }</p>
            {
                thumbnails.into_iter().map(|thumbnail| {
                    html! {
                        <div key={thumbnail.category.clone()}>
                            <Link<Route> to={Route::Detail{ image: thumbnail.category.clone()}}>{
                                capitalize(&thumbnail.category)
                            }</Link<Route>>
                        </div>
                    }
                }).collect::<Html>()
            }

        </>
    }
}
