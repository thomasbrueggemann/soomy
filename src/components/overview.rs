use yew::prelude::*;
use yew_router::prelude::*;

use crate::route::Route;

#[function_component]
pub fn Overview() -> Html {
    html! { <><p>{ "Overview" }</p><Link<Route> to={Route::Detail}>{ "Detail" }</Link<Route>></> }
}