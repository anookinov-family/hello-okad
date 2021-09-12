use crate::header::{ListHeader, Props as HeaderProps};
use crate::item::{ListItem, Props as ItemProps};
use yew::html::{ChildrenRenderer, NodeRef};
use yew::prelude::*;
use yew::virtual_dom::{VChild, VComp};

#[derive(Clone, PartialEq)]
pub enum Variants {
    Item(<ListItem as Component>::Properties),
    Header(<ListHeader as Component>::Properties),
}

impl From<ItemProps> for Variants {
    fn from(props: ItemProps) -> Self {
        Variants::Item(props)
    }
}

impl From<HeaderProps> for Variants {
    fn from(props: HeaderProps) -> Self {
        Variants::Header(props)
    }
}

#[derive(Clone, PartialEq)]
pub struct ListVariant {
    props: Variants,
}

impl<CHILD> From<VChild<CHILD>> for ListVariant
where
    CHILD: Component,
    CHILD::Properties: Into<Variants>,
{
    fn from(vchild: VChild<CHILD>) -> Self {
        Self {
            props: vchild.props.into(),
        }
    }
}

impl From<ListVariant> for Html {
    fn from(variant: ListVariant) -> Html {
        match variant.props {
            Variants::Header(props) => {
                VComp::new::<ListHeader>(props, NodeRef::default(), None).into()
            }
            Variants::Item(props) => VComp::new::<ListItem>(props, NodeRef::default(), None).into(),
        }
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub children: ChildrenRenderer<ListVariant>,
}

pub struct List {
    props: Props,
}

impl Component for List {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { 
            props,
        }
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
                { self.view_header() }
                <div>
                    { self.view_items() }
                </div>
            </div>
        }
    }
}

impl List {
    fn view_header(&self) -> Html {
        html! { for self.props.children.iter().filter(|c| matches!(c.props, Variants::Header(_))) }
    }

    fn view_items(&self) -> Html {
        self.props
            .children
            .iter()
            .filter(|c| matches!(&c.props, Variants::Item(props) if !props.hide))
            .enumerate()
            .map(|(i, mut c)| {
                if let Variants::Item(ref mut props) = c.props {
                    props.name = format!("#{} - {}", i + 1, props.name);
                }
                c
            })
            .collect::<Html>()
    }
}