use yew::prelude::*;
use yew_router::{router::Router};
use crate::switch::AppRoute;
use crate::pages::family::Family;

pub enum Msg {
}

pub struct AppComponent {
}

impl Component for AppComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn view(&self) -> Html {
        html! {
            <Router<AppRoute>
                render={Router::render(Self::switch)}
            />
        }
    }
}

impl AppComponent {
    fn switch(route: AppRoute) -> Html {
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
}
