use yew::prelude::*;

pub enum Msg {
    AddOne,
}

pub struct AppComponent {
    link: ComponentLink<Self>,
    count: i64,
}

impl Component for AppComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            count: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => {
                self.count += 1;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <button class="py-2 px-4 bg-green-500 text-white font-semibold rounded-lg shadow-md hover:bg-green-700 focus:outline-none focus:ring-2 focus:ring-green-400 focus:ring-opacity-75" onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</button>
                <p class=classes!("bg-red-100")>{ self.count }</p>
            </div>
        }
    }
}