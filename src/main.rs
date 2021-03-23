#![recursion_limit="2048"]
use serde::Deserialize;
use yew_router::{route::Route, switch::Permissive};
use yew::{
    format::{Json, Nothing},
    prelude::*,
    services::fetch::{FetchService, FetchTask, Request, Response},
};

mod switch;
use switch::{AppAnchor, AppRoute, AppRouter, PublicUrlSwitch};

mod components;

mod pages;
use pages::{page_not_found::PageNotFound, pc::Pc, home::Home};

pub struct Unisulma;
impl Component for Unisulma {
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
                <main>
                    <AppRouter
                        render=AppRouter::render(Self::switch)
                        redirect=AppRouter::redirect(|route: Route| {
                            AppRoute::PageNotFound(Permissive(Some(route.route))).into_public()
                        })
                    />
                </main>
            </>
        }
    }
}


fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    App::<Unisulma>::new().mount_to_body();
}

impl Unisulma{
    fn switch(switch: PublicUrlSwitch) -> Html {
        match switch.route() {
            AppRoute::Pc(id, number) => {
                html! { <Pc id=number /> }
            }
            AppRoute::PageNotFound(Permissive(route)) => {
                html! { <PageNotFound route=route /> }
            }
            AppRoute::Home => {
                html! { <Home /> }
            }
        }
    }
}