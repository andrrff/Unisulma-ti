use serde::Deserialize;
use yew_router::{route::Route, switch::Permissive};
use yew::{
    format::{Json, Nothing},
    prelude::*,
    services::fetch::{FetchService, FetchTask, Request, Response},
};

use wasm_bindgen::{JsValue, prelude::*};

use crate::{
    switch::{AppAnchor, AppRoute},
    js::caller
};

#[derive(Deserialize, Debug, Clone)]
pub struct Info {
    setor: String,
    id: String,
    hdd: String,
    cpu: String,
    os: String,
    user: String,
    marca: String,
    monitor: String,
    tamMonitor: String,
    ram: String,
    status: String,
    visibilidade: bool
}

#[derive(Deserialize, Debug, Clone)]
pub struct Data {
    data: Vec<Info>
}

#[derive(Debug)]
pub enum Msg {
    GetInfo,
    ChangeSearchOption(bool),
    Button(f64),
    ReceiveResponse(Result<Data, anyhow::Error>),
    Payload(String),
}

#[derive(Debug)]
pub struct Home {
    payload: String,
    option: bool,
    option01: bool,
    option02: bool,
    debugged_payload: String,
    fetch_task: Option<FetchTask>,
    pc: Option<Data>,
    link: ComponentLink<Self>,
    error: Option<String>,
}
impl Home {
    fn view_iss_location(&self) -> Html {
        let mut setor: Vec<String> = Vec::new();
        let mut id: Vec<String> = Vec::new();
        let mut hdd: Vec<String> = Vec::new();
        let mut cpu: Vec<String> = Vec::new();
        let mut os: Vec<String> = Vec::new();
        let mut user: Vec<String> = Vec::new();
        let mut marca: Vec<String> = Vec::new();
        let mut monitor: Vec<String> = Vec::new();
        let mut tamMonitor: Vec<String> = Vec::new();
        let mut ram: Vec<String> = Vec::new();
        let mut status: Vec<String> = Vec::new();
        let mut cards: Vec<Html> = Vec::new();

        match self.pc {
            Some(ref pc) => {
                fn search(word: String, writing: String) -> bool
                {
                        let chars_name: Vec<char> = word.chars().collect();
                        let writing_chars: Vec<char> = writing.chars().collect();
                        {
                            for (j, c) in writing_chars.iter().enumerate()
                            {
                                if c != &chars_name[j]
                                {
                                    return false;
                                }
                            }
                        }
                        true
                }
                let mut count = 0;
                for i in 0..pc.data.len()
                {
                        if search(pc.data[i].user.clone().to_lowercase(), self.debugged_payload.clone().to_lowercase()) && self.option == true && pc.data[i].visibilidade.clone() == true || search(pc.data[i].setor.clone().to_lowercase(), self.debugged_payload.clone().to_lowercase()) && self.option == false && pc.data[i].visibilidade.clone() == true
                        {
                            count += 1;
                            user.push(pc.data[i].user.clone());
                            cards.push(html!{
                                <AppAnchor route=AppRoute::Pc(pc.data[i].id.clone(), i.to_string())>
                                    <li style="background: white; min-width: 500px;min-height: 350px;  
                                            border: none;
                                            margin: 30px;
                                            border-radius: 1rem;
                                            font-size: 1.4rem;
                                            padding-left: 3.8rem;
                                            box-shadow: 0.2rem 0.2rem 0.5rem var(--greyLight-2), -0.2rem -0.2rem 0.5rem var(--white);
                                            background: none;
                                            font-family: inherit;
                                            color: #9baacf;">
                                    <strong><h2 class="text_config" style="font-size: 150%;font-weight: 1000; padding-bottom: 20px; color: #6d5dfc">
                                        {pc.data[i].user.clone().to_uppercase()}{":"}
                                    </h2></strong>
                                    <strong><h2 class="text_config">
                                        <img src="https://img.icons8.com/material-sharp/24/000000/marker.png"/><strong>{"Setor: "}</strong>{pc.data[i].setor.clone().to_uppercase()}
                                    </h2></strong>
                                    <strong><h2 class="text_config">
                                        <img src="https://img.icons8.com/ios-glyphs/30/000000/medical-id.png"/><strong>{"Computer ID: "}</strong>{pc.data[i].id.clone().to_uppercase()}
                                    </h2></strong>
                                    <strong><h2 class="text_config">
                                        <img src="https://img.icons8.com/fluent-systems-filled/24/000000/hdd.png"/><strong>{"HDD: "}</strong>{pc.data[i].hdd.clone().to_uppercase()}
                                    </h2></strong>
                                    <strong><h2 class="text_config">
                                        <img src="https://img.icons8.com/material/24/000000/smartphone-cpu--v1.png"/><strong>{"CPU: "}</strong>{pc.data[i].cpu.clone().to_uppercase()}
                                    </h2></strong>
                                    <strong><h2 class="text_config">
                                        <img src="https://img.icons8.com/metro/26/000000/windows-logo.png"/><strong>{"OS: "}</strong>{pc.data[i].os.clone().to_uppercase()}
                                    </h2></strong>
                                    <strong><h2 class="text_config">
                                        <img src="https://img.icons8.com/windows/32/000000/flipboard-logo.png"/><strong>{"Marca da Máquina: "}</strong>{pc.data[i].marca.clone().to_uppercase()}
                                    </h2></strong>
                                    <strong><h2 class="text_config">
                                        <img src="https://img.icons8.com/material-sharp/24/000000/monitor.png"/><strong>{"Monitor: "}</strong>{pc.data[i].monitor.clone().to_uppercase()}
                                    </h2></strong>
                                    <strong><h2 class="text_config">
                                        <img src="https://img.icons8.com/windows/32/000000/page-size.png"/><strong>{"Tamanho do monitor: "}</strong>{pc.data[i].tamMonitor.clone().to_uppercase()}{"\""}
                                    </h2></strong>
                                    <strong><h2 class="text_config">
                                        <img src="https://img.icons8.com/material-rounded/24/000000/smartphone-ram.png"/><strong>{"Mem. Ram: "}</strong>{pc.data[i].ram.clone().to_uppercase()}
                                    </h2></strong>
                                    <strong><h2 class="text_config">
                                        <img src="https://img.icons8.com/metro/26/000000/unknown-status.png"/><strong>{"Status do Sistema: "}</strong>{pc.data[i].status.clone().to_uppercase()}
                                    </h2></strong>
                                </li>
                                </AppAnchor>
                            })
                        }
                   }
                   let number = pc.data.clone().len();
                //    number = number as f64;
                html! {
                    <>
                        <div class="level-item" style="padding-top: 80px;">
                        <img src="https://doity.com.br/media/doity/eventos/evento-58419-logo_organizador.png" alt="Home Logo"/>
                        </div>
                        <div class="level-item">
                            <div class="search">
                            <input type="text" class="search__input" oninput=self.link.callback(|input: InputData| Msg::Payload(input.value)) value=&self.payload placeholder="Pesquisar..."/>
                                <div class="search__icon">
                                    <span class="icon"><i aria-hidden="true" class="fa fa-search"></i></span>
                                </div>
                                <a onclick=self.link.callback(move |_| Msg::Button(number as f64))>
                                    // <AppAnchor route=AppRoute::AdminCreate(pc.data.len().to_string())>
                                    <span class="btn tag is-danger" style="margin: 5px;
                                                                            align-items: center;
                                                                            background-color: #f5f5f5;
                                                                            border-radius: 5px 18px 18px 5px;
                                                                            box-shadow: 0.2rem 0.2rem 0.5rem var(--greyLight-2), -0.2rem -0.2rem 0.5rem var(--white);
                                                                            color: #4a4a4a;
                                                                            display: inline-flex;
                                                                            /* left: -10px; */
                                                                            font-size: .75rem;
                                                                            height: 5.1em;
                                                                            width: 10px;
                                                                            justify-content: center;
                                                                            line-height: 1.5;
                                                                            padding-left: .75em;
                                                                            padding-right: .75em;
                                                                            white-space: nowrap;
                                                                            background-color: #E4EBF5;">
                                    <strong style="color: #6D5DFC; font-weight: 790; font-size: 400%;"><img src="https://img.icons8.com/material-sharp/24/000000/add-user-male.png"/></strong></span>
                                    // </AppAnchor>
                                </a>
                                // <div class="icon" style="padding-left: 10px">
                                // // <div class="icon__home">
                                // //     <ion-icon name="home"></ion-icon></div>
                                // // <div class="icon__account">
                                // //     <ion-icon name="person"></ion-icon></div>
                                // <div class="icon__settings">
                                //     <img src="https://img.icons8.com/material-sharp/24/000000/settings.png" style="margin: 35px;"/></div>
                                // </div>
                            </div>
                        </div>
                        <div class="radio" style="padding-top: 20px">
                            <div class="radio__1">
                                <h1 style="font-family: 'Oswald', sans-serif;
                                        color: #243D67;
                                        font-size: 100%;
                                        font-weight: 700;">{"Nome"}</h1>
                                <input id="radio-1" type="radio"  name="radio" value="1" onclick=self.link.callback(move |_| Msg::ChangeSearchOption(true)) checked=self.option01.clone()/>
                                <label for="radio-1"></label>
                            </div>
                            
                            <div class="radio__2">
                                <h1 style="font-family: 'Oswald', sans-serif;
                                        color: #243D67;
                                        font-size: 100%;
                                        font-weight: 700;">{"Setor"}</h1>
                                <input id="radio-2" type="radio"  name="radio" value="2" onclick=self.link.callback(move |_| Msg::ChangeSearchOption(false)) checked=self.option02.clone()/>
                                <label for="radio-2"></label>
                            </div>
                        </div>
                        <ol class="gradient-list con-cards" style="margin-right: 30px;">
                            {for cards.clone()}
                        </ol>
                        <div class="level-item" style="padding: 80px;">
                        <a href="javascript:void(
                            window.open(
                            'https://form.jotform.com/210793492450053',
                            'blank',
                            'scrollbars=yes,
                            toolbar=no,
                            width=700,
                            height=500'
                            )
                        )
                        ">
                        <button class="btn __primary" style="min-width: 60px;
                                                font-size: 120%;
                                                font-weight: 600;
                                                height: 60px;
                                                color: #4a4a4a;
                                                top: 85%;
                                                right: 30%;
                                                border-radius: 20px;
                                                background: #e4ebf5;
                                                border: 0px;
                                                outline: none;
                                                cursor: pointer;
                                                z-index: 200;
                                                margin: 10px;
                                                box-shadow: 0.3rem 0.3rem 0.6rem var(--greyLight-2), -0.2rem -0.2rem 0.5rem var(--white);
                                                transition: all .25s ease;">
                                    <img src="https://img.icons8.com/metro/26/000000/error.png" style="max-width: 24px;"/>{" Relatar um problema"}
                                </button>
                        </a>
                        <a href="https://github.com/andrrff">
                        <button class="btn __primary" style="min-width: 60px;
                                                font-size: 120%;
                                                font-weight: 600;
                                                height: 60px;
                                                width: 60px;
                                                color: #4a4a4a;
                                                top: 85%;
                                                right: 5%;
                                                border-radius: 20px;
                                                background: #e4ebf5;
                                                border: 0px;
                                                outline: none;
                                                cursor: pointer;
                                                z-index: 200;
                                                margin: 10px;
                                                box-shadow: 0.3rem 0.3rem 0.6rem var(--greyLight-2), -0.2rem -0.2rem 0.5rem var(--white);
                                                transition: all .25s ease;">
                                    <img src="https://img.icons8.com/material-sharp/24/000000/github.png"/>
                                </button>
                        </a>
                            </div>
                    </>
                }
            }
            None => {
                html! {
                    <div class="position-absolute top-50 start-50 translate-middle">
                        <figure class="image">
                            <img class="is-rounded" src="https://doity.com.br/media/doity/eventos/evento-58419-logo_organizador.png" alt="Home Suport" />
                        </figure>
                        <div class="d-flex justify-content-center">
                            <div class="spinner-border is-white" role="status">
                                <span class="visually-hidden">{"Carregando..."}</span>
                            </div>
                        </div>
                    </div>
                }
            }
        }
    }
    fn export_array_data(&self) -> f64
    {
        match self.pc {
            Some(ref pc) => {
                self.pc.as_ref().unwrap().data.clone().len() as f64
            }
            None => 
            {
                0.0
            }
        }
    }
    fn view_fetching(&self) -> Html {
        if self.fetch_task.is_some() {
            html! {}
        } else {
            html! { <p></p> }
        }
    }
    fn view_error(&self) -> Html {
        if let Some(ref error) = self.error {
            html! { <p>{ error.clone() }</p> }
        } else {
            html! {}
        }
    }
}
impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|_msg: Msg| Msg::GetInfo);
        callback.emit(Msg::GetInfo);
        // let callback = link.callback(move |_msg: Msg| Msg::Payload("".to_string()));
        // callback.emit(Msg::Payload("".to_string()));
        Self {
            payload:          String::default(),
            option: true,
            option01: true,
            option02: false,
            debugged_payload: format!("{}", "Como se chama?"),
            fetch_task: None,
            pc: None,
            link,
            error: None,
        }
    }
    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }
    fn update(&mut self, msg: Self::Message) -> bool {
        use Msg::*;

        match msg {
            Button(number) => {
                caller::write_new_pc(JsValue::from_str("?"), JsValue::from_str("?"), JsValue::from_str("?"), JsValue::from_str("?"), JsValue::from_str("?"), JsValue::from_str(Box::leak(self.debugged_payload.clone().into_boxed_str())), JsValue::from_str("?"), JsValue::from_str("?"), JsValue::from_str("?"), JsValue::from_str("?"), JsValue::from_str("?"), vec![JsValue::from_str("?")].into_boxed_slice(), vec![JsValue::from_str("?")].into_boxed_slice(), JsValue::from_f64(number), JsValue::from_bool(true));
                // caller::write_new_pc(JsValue::from_str("test"), JsValue::from_str("test"), JsValue::from_str("test"), JsValue::from_str("test"), JsValue::from_str("test"), JsValue::from_str(Box::leak(self.debugged_payload.clone().into_boxed_str())), JsValue::from_str("test"), JsValue::from_str("test"), JsValue::from_str("test"), JsValue::from_str("test"), JsValue::from_str("test"), vec![JsValue::from_str("test")].into_boxed_slice(), vec![JsValue::from_str("test")].into_boxed_slice(), JsValue::from_f64(number));
                let word = self.debugged_payload.clone();
                let callback = self.link.callback(move |_msg: Msg| Msg::Payload(word.clone()));
                callback.emit(Msg::Payload(self.debugged_payload.clone()));
                let callback = self.link.callback(|_msg: Msg| Msg::GetInfo);
                callback.emit(Msg::GetInfo);
                true
            }
            ChangeSearchOption(boolean) => {
                self.option = boolean;
                if boolean
                {
                    self.option01 = boolean;
                    self.option02 = !boolean;
                }
                else
                {
                    self.option01 = boolean;
                    self.option02 = !boolean;
                }
                true
            }
            Payload(payload) => {
                // let callback = self.link.callback(|_msg: Msg| Msg::GetInfo);
                // callback.emit(Msg::GetInfo);
                if payload != self.payload {
                    self.debugged_payload = format!("{}", payload);
                    if self.debugged_payload == ""
                    {
                        self.debugged_payload = format!("{}", "Como se chama?");
                    }
                    self.payload = payload;
                    true
                } else {
                    self.payload = payload;
                    false
                }
            }
            GetInfo => {
                let request = Request::get("https://unisulma-ti-default-rtdb.firebaseio.com/.json")
                    .body(Nothing)
                    .expect("Could not build request.");
                let callback =
                    self.link
                        .callback(|response: Response<Json<Result<Data, anyhow::Error>>>| {
                            let Json(data) = response.into_body();
                            Msg::ReceiveResponse(data)
                        });
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                self.fetch_task = Some(task);
                true
            }
            ReceiveResponse(response) => {
                match response {
                    Ok(info) => {
                        self.pc = Some(info);
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
            // <button onclick=self.link.callback(|_| Msg::GetInfo)>{"abrir"}</button>
            { self.view_fetching() }
            { self.view_iss_location() }
            { self.view_error() }
            </>
        }
    }
}