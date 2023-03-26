use yew::prelude::*;
use yew_router::prelude::*;

use crate::route::Route;

#[function_component]
pub fn Overview() -> Html {
    html! { <><p>{ "Overview" }</p><img src="https://images.pexels.com/photos/2668755/pexels-photo-2668755.jpeg" /><Link<Route> to={Route::Detail}>{ "Detail" }</Link<Route>></> }
}
