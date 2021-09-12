use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub text: String,
}

pub struct ListHeader {
    props: Props,
}

impl Component for ListHeader {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn view(&self) -> Html {
        html! {
            <h2 class="text-lg leading-6 font-medium text-gray-900">
                { &self.props.text }
            </h2>
        }
    }
}