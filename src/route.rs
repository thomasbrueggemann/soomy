use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Overview,
    #[at("/detail/:image")]
    Detail { image: String },
}
