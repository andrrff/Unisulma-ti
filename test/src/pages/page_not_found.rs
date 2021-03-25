use yew::prelude::*;
use yewtil::NeqAssign;

use crate::{
    switch::{AppAnchor, AppRoute},
};

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub route: Option<String>,
}

pub struct PageNotFound {
    props: Props,
}
impl Component for PageNotFound {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        html! {
            <>
                <section class="hero is-danger is-bold is-large " style="padding-top: 40px">
                    <div class="hero-body">
                        <div class="container">
                            <h1 class="title">
                                { "Página não econtrada" }
                            </h1>
                            <h2 class="subtitle">
                                { "Essa página não existe T_T" }<br/>{" "}<br/>{" "}
                                
                                <figure class="image is-128x128">
                                    <img class="is-rounded" src="https://media1.tenor.com/images/288dcf97572a602d2e12e4a7b7f0fc6b/tenor.gif"/>
                                </figure> <br/>{" "}<br/>{" "}
                                <div class="buttons">
                                <a classes="button is-dark is-rounded" href="/">
                                    { "Voltar" }
                                </a>
                                </div>
                            </h2>
                        </div>
                    </div>
                </section>
            </>
        }
    }
}
