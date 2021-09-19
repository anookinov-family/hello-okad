use yew_router::prelude::*;

#[derive(Clone, Debug, Switch)]
pub enum AppRoute {
    #[to = "/family/"]
    Family,
    #[to = "/!"]
    Home,
}