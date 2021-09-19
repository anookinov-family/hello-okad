use crate::components::header::ListHeader;
use crate::components::item::ListItem;
use crate::pages::list::List;
use yew::prelude::*;

pub enum Msg {
}

pub struct Family {
}

impl Component for Family {
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
        let letters = ('A'..='C')
            .map(|letter| html_nested! { 
                <ListItem
                    name=letter.to_string()
                    title=letter.to_string()
                    status=letter.to_string()
                    role=letter.to_string()
                />
            });
        html! {
            <div class="lg:flex lg:items-center lg:justify-between">
                <div class="flex-1 min-w-0">
                    <h1 class="text-2xl font-bold leading-7 text-gray-900 sm:text-3xl sm:truncate">{ "Okad Family" }</h1>
                    <List>
                        <ListHeader text="Members" />
                        { for letters }
                    </List>
                </div>
            </div>
        }
    }
}