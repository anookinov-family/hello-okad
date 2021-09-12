use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub hide: bool,
    pub name: String,
    #[prop_or_default]
    pub children: Children,
}

pub struct ListItem {
    props: Props,
}

impl Component for ListItem {
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
            <div>
                { &self.props.name }
                { self.view_details() }
            </div>
        }
    }
}

impl ListItem {
    fn view_details(&self) -> Html {
        if self.props.children.is_empty() {
            html! {}
        } else {
            html! {
                <div>
                    { self.props.children.clone() }
                </div>
            }
        }
    }
}