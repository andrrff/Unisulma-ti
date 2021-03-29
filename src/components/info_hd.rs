use serde::Deserialize;
use yew::{
    format::{Json, Nothing},
    prelude::*,
    services::fetch::{FetchService, FetchTask, Request, Response},
};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub hdd: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Struture {
    cache: String,
    capacidade: String,
    form_factor: String,
    interface: String,
    latencia: String,
    links: String,
    rpm: String,
}

#[derive(Debug)]
pub enum Msg {
    GetInfo,
    ReceiveResponse(Result<Struture, anyhow::Error>),
}

#[derive(Debug)]
pub struct LoadInfo {
    props: Props,
    toggle_view: bool,
    fetch_task: Option<FetchTask>,
    json: Option<Struture>,
    link: ComponentLink<Self>,
    error: Option<String>,
}

impl LoadInfo {
    fn view_json(&self) -> Html {
        match self.json {
            Some(ref content) => {
                html!{
                    <>
                        <div class="box_03">
                            <div style="padding-left: 30px">
                                <h1 style="font-size: 250%; font-weight: 600; padding-left: 20px">{"HDD"}</h1>
                                <div style="padding-left: 10px">
                                    <h1><strong>{"Modelo: "}</strong>{self.props.hdd.clone().replace("_", " ")}</h1>
                                    <h1><strong>{"Cache: "}</strong>{content.cache.clone()}</h1>
                                    <h1><strong>{"Capacidade: "}</strong>{content.capacidade.clone()}</h1>
                                    <h1><strong>{"Tamanho: "}</strong>{content.form_factor.clone()}</h1>
                                    <h1><strong>{"Interface: "}</strong>{content.interface.clone()}</h1>
                                    <h1><strong>{"Latência: "}</strong>{content.latencia.clone()}</h1>
                                    <h1><strong>{"RPM: "}</strong>{content.rpm.clone()}</h1>
                                </div>
                                <a class="open_link_02" href=content.links.clone() target="_blank"><img src="https://img.icons8.com/material-two-tone/24/000000/external-link.png" style="width: 20px"/></a>
                            </div>
                        </div>
                    </>
                }
            }
            None => {
                html! {}
            }
        }
    }

    fn view_fetching(&self) -> Html {
        if self.fetch_task.is_some() {
            html! {}
        } else {
            html! {}
        }
    }
    fn view_error(&self) -> Html {
        if let Some(ref error) = self.error {
            html! {<p>{ error.clone() }</p>}
        } else {
            html! {}
        }
    }
}
impl Component for LoadInfo {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|_msg: Msg| Msg::GetInfo);
        callback.emit(Msg::GetInfo);
        Self {
            props,
            toggle_view: false,
            fetch_task:       None,
            json:             None,
            link,
            error:            None,
        }
    }
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use Msg::*;

        match msg {
            GetInfo => {
                self.toggle_view = !self.toggle_view;
                let request = Request::get(format!("https://unisulma-ti-default-rtdb.firebaseio.com/info/hdd/{}/0/.json", self.props.hdd.clone()))
                    .body(Nothing)
                    .expect("Não foi possível efetuar o request.");
                let callback =
                    self.link
                        .callback(|response: Response<Json<Result<Struture, anyhow::Error>>>| {
                            let Json(data) = response.into_body();
                            Msg::ReceiveResponse(data)
                        });
                let task = FetchService::fetch(request, callback).expect("Falha ao iniciar o request");
                self.fetch_task = Some(task);
                true
            }
            ReceiveResponse(response) => {
                match response {
                    Ok(dados) => {
                        self.json = Some(dados);
                    }
                    Err(error) => {
                        self.error = Some(error.to_string())
                    }
                }
                self.fetch_task = None;
                true
            }
        }
    }
    fn view(&self) -> Html {
        html! {
            <>
                { self.view_fetching() }
                { self.view_json() }
                { self.view_error() }
            </>
        }
    }
}