use components::{detail::Detail, overview::Overview};
use route::Route;
use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod route;

fn routes(routes: Route) -> Html {
    match routes {
        Route::Overview => html! { <Overview /> },
        Route::Detail => html! {
            <Detail />
        },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={routes} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
