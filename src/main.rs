use yew::prelude::*;

enum Msg {
    AddOne,
}

struct AppComponent {
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
                <button onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</button>
                <p>{ self.count }</p>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<AppComponent>();
}
