use yew::prelude::*;
use yew_router::prelude::*;
use crate::pages::family::Family;

#[derive(Clone, Debug, Switch)]
pub enum AppRoute {
    #[to = "/family/"]
    Family,
    #[to = "/!"]
    Home,
}

pub fn switch(route: AppRoute) -> Html {
    match route {
        AppRoute::Home =>
            html! {
                <h1>{ "Home" }</h1>
            },
        AppRoute::Family =>
            html! {
                <Family />
            }
    }
}