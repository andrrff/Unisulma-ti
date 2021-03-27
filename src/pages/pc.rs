use yewtil::NeqAssign;
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
    components::pc_info,
    js::caller
};

#[derive(Deserialize, Debug, Clone)]
pub struct Data {
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
    problemas: Vec<String>,
    servicos: Vec<String>
}

#[derive(Debug)]
pub enum Msg {
    GetInfo,
    Info(u8),
    ReceiveResponse(Result<Data, anyhow::Error>),
}

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub id: String,
}

pub struct Pc{
    props: Props,
    option01: bool,
    option02: bool,
    option03: bool,
    view: Html,
    fetch_task: Option<FetchTask>,
    pc: Option<Data>,
    link: ComponentLink<Self>,
    error: Option<String>,
}

impl Pc
{
    fn view_html(&self) -> Html {
        let mut info: Html = html!{};
        match self.pc {
            Some(ref pc) => {
                if self.option01
                {
                    info = html!{
                        <pc_info::Pc setor=pc.setor.clone()
                                    id=pc.id.clone()
                                    hdd=pc.hdd.clone()
                                    cpu=pc.cpu.clone()
                                    os=pc.os.clone()
                                    user=pc.user.clone()
                                    marca=pc.marca.clone()
                                    monitor=pc.monitor.clone()
                                    tamMonitor=pc.tamMonitor.clone()
                                    ram=pc.ram.clone()
                                    status=pc.status.clone()/>
                    }
                }
                else if self.option02
                {
                    info = html!{
                        <ol class="gradient-list" style="margin-left: 30px; margin-right: 30px; margin-top: 100px;">
                                <strong><h2 class="text_config_01">
                                    {for pc.problemas.clone()}
                                </h2></strong>
                        </ol>
                    }
                }
                else if self.option03
                {
                    info = html!{
                        <ol class="gradient-list" style="margin-left: 30px; margin-right: 30px; margin-top: 100px;">
                                <strong><h2 class="text_config_01">
                                    {for pc.servicos.clone()}
                                </h2></strong>
                        </ol>
                    }
                }
                else
                {
                    info = html!{}
                }

                let mut number_problemas: String = String::default();
                let mut number_servicos: String = String::default();

                if pc.problemas.clone().len() == 1 && pc.problemas[0].clone() == "".to_string()
                {
                    number_problemas = 0.to_string();
                }
                else 
                {
                    number_problemas = pc.problemas.clone().len().to_string();
                }

                if pc.servicos.clone().len() == 1 && pc.servicos[0].clone() == "".to_string()
                {
                    number_servicos = 0.to_string();
                }
                else 
                {
                    number_servicos = pc.servicos.clone().len().to_string();
                }


                html! {
                    <>
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
                                                        position: absolute;
                                                        bottom: 115%;
                                                        right: 2%;
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
                    <div class="segmented-control" style="position: absolute; left: 10%; top: 15%;">
      
                    <input type="radio" name="radio2" value="3" id="tab-1" onclick=self.link.callback(move |_| Msg::Info(0)) checked=self.option01/>
                    <label for="tab-1" class= "segmented-control__1">
                        <p style="color:#4A4A4A;">{"Info"}</p></label>
                    
                    <input type="radio" name="radio2" value="4" id="tab-2" onclick=self.link.callback(move |_| Msg::Info(1)) checked=self.option02/>
                    <label for="tab-2" class= "segmented-control__2">
                        <p style="color:#4A4A4A;">{"Problemas"}<span class="tag is-danger" style="font-size: 0.5rem; background: #6D5DFC">
                                <strong style="color: white; font-weight: 790;">{number_problemas}</strong></span></p></label>
                    
                    <input type="radio" name="radio2" value="5" id="tab-3" onclick=self.link.callback(move |_| Msg::Info(2)) checked=self.option03/>
                    <label for="tab-3" class= "segmented-control__3">
                        <p style="color:#4A4A4A;">{"Serviços"}<span class="tag is-dark" style="font-size: 0.5rem;">
                                <strong style="color: white; font-weight: 790;">{number_servicos}</strong></span></p></label>
                    
                    <div class="segmented-control__color"></div>
                    </div>
                    {info}
                    </>
                }
            }
            None => {
                html! {}
            }
        }
    }

    fn export_name(&self) -> String
    {
        match self.pc {
            Some(ref pc) => {
                pc.user.clone()
            }
            None => {
                "Carregando".to_string()
            }
        }
    }
    fn export_setor(&self) -> String
    {
        match self.pc {
            Some(ref pc) => {
                pc.setor.clone()
            }
            None => {
                "...".to_string()
            }
        }
    }
    // fn view_html(&self) -> String {
    //     match self.pc {
    //         Some(ref pc) => {
    //             self.pc.
    //         }
    //         None => {
    //             String::default()
    //         }
    //     }
    // }
    fn view_fetching(&self) -> Html {
        if self.fetch_task.is_some() {
            html! { <p>{ "Carregando dados..." }</p> }
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

impl Component for Pc {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|_msg: Msg| Msg::GetInfo);
        callback.emit(Msg::GetInfo);
        Self{
            props,
            option01: true,
            option02: false,
            option03: false,
            view: html!{},
            fetch_task: None,
            pc: None,
            link,
            error: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use Msg::*;

        match msg {
        Info(option) => {
            match option
            {
                0 => {self.option01 = true; self.option02 = false; self.option03 = false;},
                1 => {self.option01 = false; self.option02 = true; self.option03 = false},
                2 => {self.option01 = false; self.option02 = false; self.option03 = true},
                _ => unimplemented!()
            }
            true
        }
        GetInfo => {
                let request = Request::get(format!("https://unisulma-ti-default-rtdb.firebaseio.com/data/{}/.json", self.props.id.clone().trim().parse::<usize>().unwrap()))
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

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
            <div class="" style="padding-left: 10px; margin: 30px;">
                                // <div class="icon__home">
                                //     <ion-icon name="home"></ion-icon></div>
                                // <div class="icon__account">
                                //     <ion-icon name="person"></ion-icon></div>
                        <div class="icon__settings">
                            <AppAnchor route=AppRoute::Home>
                                <img src="https://img.icons8.com/material/24/000000/home--v5.png" style="margin-bottom: 5px;"/>
                            </AppAnchor>
                        </div>
                    </div>
                    <div>
                        <h1 style=" font-size: 400%;
                                    font-weight: 600;
                                    z-index: 1;
                                    text-align: center;
                                    /* left: 40%; */
                                    top: 10%;">{self.export_name()}</h1>
                        <h4 style="font-size: 150%;
                                    font-weight: 400;
                                    z-index: 1;
                                    text-align: center;
                                    /* left: 40%; */
                                    top: 20%;">{self.export_setor()}</h4>
                    </div>
                <div class="container">
                    <div class="components" style="width: 100%; margin-top: 60px; margin-bottom: 100px; grid-template-columns: auto;overflow: auto;">
                        {self.view_html()}
                    </div>
                </div>
                <div class="level-item" style="padding-top: 100px; padding-bottom: 40px">
                            <h1 style="font-family: 'Oswald', sans-serif;
                                        color: #383741;
                                        font-size: 100%;
                                        font-weight: 700;">
                                <a
                        href="javascript:void(
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
                        <img src="https://img.icons8.com/metro/26/000000/error.png" style="max-width: 24px;"/>{"Reportar"}
                        </a>  {" | "}</h1>
                        <img src="https://img.icons8.com/material-sharp/24/000000/github.png" style="max-width: 24px;"/><h1 style="font-family: 'Oswald', sans-serif;
                                        color: #383741;
                                        font-size: 100%;
                                        font-weight: 700;">
                                <a href="https://github.com/andrrff">{" andrrff GitHub"}</a></h1>{" | "}<img src="https://img.icons8.com/small/16/000000/microsoft-admin.png" style="max-width: 24px;"/><h1 style="font-family: 'Oswald', sans-serif;
                                        color: #383741;
                                        font-size: 100%;
                                        font-weight: 700;"><AppAnchor route=AppRoute::Admin(self.props.id.clone().parse::<String>().unwrap())><a>{"Admin"}</a></AppAnchor></h1>
                                
                        </div>   
            </>
        }
    }
}
