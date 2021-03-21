use yew::prelude::*;
use yewtil::NeqAssign;

use crate::{
    switch::{AppAnchor, AppRoute},
};

pub struct Pc;
impl Component for Pc {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <h1>{"Ola mundo!"}</h1>
            </>
        }
    }
}
