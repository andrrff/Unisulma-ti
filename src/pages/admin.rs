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
    Payload(u8, String),
    ReceiveResponse(Result<Data, anyhow::Error>),
}

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub id: String,
}

pub struct Admin{
    props: Props,
    view: Html,
    debug_setor: String,
    debug_id: String,
    debug_hdd: String,
    debug_cpu: String,
    debug_os: String,
    debug_user: String,
    debug_marca: String,
    debug_monitor: String,
    debug_tamMonitor: String,
    debug_ram: String,
    debug_status: String,
    debug_problemas: Vec<String>,
    debug_servicos: Vec<String>,
    fetch_task: Option<FetchTask>,
    pc: Option<Data>,
    link: ComponentLink<Self>,
    error: Option<String>,
}

impl Admin
{
    fn view_html(&self) -> Html {
        let mut info: Html = html!{};
        match self.pc {
            Some(ref pc) => {
                html! {
                    <>
                    <form class="validate-form" style="padding-bottom: 30px">
                        // <h1>{"Editar informações"}</h1>
                        <p style="color: black">{"Tome muito cuidado com o que você está modificando!!!"}</p>
                        <div>
                            <div class="input-holder">
                            <input type="text" placeholder=format!("User: {}", pc.user.clone()) class="validate" required=false style="border: 2px solid #ccc;
                                                                                                                                        border-radius: 4px;
                                                                                                                                        padding: 10px;
                                                                                                                                        width: 100%;
                                                                                                                                        resize: none;
                                                                                                                                        transition: .3s;
                                                                                                                                        &:focus {
                                                                                                                                            border: 2px solid #999;
                                                                                                                                            outline: none;
                                                                                                                                        }" oninput=self.link.callback(|input: InputData| Msg::Payload(0, input.value))/>
                            <span class="message">{"Who are you stranger?"}</span>
                            </div>
                            <div class="input-holder">
                            <input type="text" placeholder=format!("Setor: {}", pc.setor.clone()) class="validate" required=false style="border: 2px solid #ccc;
                                                                                                                                        border-radius: 4px;
                                                                                                                                        padding: 10px;
                                                                                                                                        width: 100%;
                                                                                                                                        resize: none;
                                                                                                                                        transition: .3s;
                                                                                                                                        &:focus {
                                                                                                                                            border: 2px solid #999;
                                                                                                                                            outline: none;
                                                                                                                                        }" oninput=self.link.callback(|input: InputData| Msg::Payload(1, input.value))/>
                            <span class="message">{"Who are you stranger?"}</span>
                            </div>
                            <div class="input-holder">
                            <input type="text" placeholder=format!("ComputerID: {}", pc.id.clone()) class="validate" required=false style="border: 2px solid #ccc;
                                                                                                                                        border-radius: 4px;
                                                                                                                                        padding: 10px;
                                                                                                                                        width: 100%;
                                                                                                                                        resize: none;
                                                                                                                                        transition: .3s;
                                                                                                                                        &:focus {
                                                                                                                                            border: 2px solid #999;
                                                                                                                                            outline: none;
                                                                                                                                        }" oninput=self.link.callback(|input: InputData| Msg::Payload(2, input.value))/>
                            <span class="message">{"Who are you stranger?"}</span>
                            </div>
                            <div class="input-holder">
                            <input type="text" placeholder=format!("HDD: {}", pc.hdd.clone()) class="validate" required=false style="border: 2px solid #ccc;
                                                                                                                                        border-radius: 4px;
                                                                                                                                        padding: 10px;
                                                                                                                                        width: 100%;
                                                                                                                                        resize: none;
                                                                                                                                        transition: .3s;
                                                                                                                                        &:focus {
                                                                                                                                            border: 2px solid #999;
                                                                                                                                            outline: none;
                                                                                                                                        }" oninput=self.link.callback(|input: InputData| Msg::Payload(3, input.value))/>
                            </div>
                            <div class="input-holder">
                            <input type="text" placeholder=format!("CPU: {}", pc.cpu.clone()) class="validate" required=false style="border: 2px solid #ccc;
                                                                                                                                        border-radius: 4px;
                                                                                                                                        padding: 10px;
                                                                                                                                        width: 100%;
                                                                                                                                        resize: none;
                                                                                                                                        transition: .3s;
                                                                                                                                        &:focus {
                                                                                                                                            border: 2px solid #999;
                                                                                                                                            outline: none;
                                                                                                                                        }" oninput=self.link.callback(|input: InputData| Msg::Payload(4, input.value))/>
                            </div>
                            <div class="input-holder">
                            <input type="text" placeholder=format!("Sistema Operacional: {}", pc.os.clone()) class="validate" required=false style="border: 2px solid #ccc;
                                                                                                                                        border-radius: 4px;
                                                                                                                                        padding: 10px;
                                                                                                                                        width: 100%;
                                                                                                                                        resize: none;
                                                                                                                                        transition: .3s;
                                                                                                                                        &:focus {
                                                                                                                                            border: 2px solid #999;
                                                                                                                                            outline: none;
                                                                                                                                        }" oninput=self.link.callback(|input: InputData| Msg::Payload(5, input.value))/>
                            </div>
                            <div class="input-holder">
                            <input type="text" placeholder=format!("Marca da Máquina: {}", pc.marca.clone()) class="validate" required=false style="border: 2px solid #ccc;
                                                                                                                                        border-radius: 4px;
                                                                                                                                        padding: 10px;
                                                                                                                                        width: 100%;
                                                                                                                                        resize: none;
                                                                                                                                        transition: .3s;
                                                                                                                                        &:focus {
                                                                                                                                            border: 2px solid #999;
                                                                                                                                            outline: none;
                                                                                                                                        }" oninput=self.link.callback(|input: InputData| Msg::Payload(6, input.value))/>
                            </div>
                            <div class="input-holder">
                            <input type="text" placeholder=format!("Monitor: {}", pc.monitor.clone()) class="validate" required=false style="border: 2px solid #ccc;
                                                                                                                                        border-radius: 4px;
                                                                                                                                        padding: 10px;
                                                                                                                                        width: 100%;
                                                                                                                                        resize: none;
                                                                                                                                        transition: .3s;
                                                                                                                                        &:focus {
                                                                                                                                            border: 2px solid #999;
                                                                                                                                            outline: none;
                                                                                                                                        }" oninput=self.link.callback(|input: InputData| Msg::Payload(7, input.value))/>
                            </div>
                            <div class="input-holder">
                            <input type="text" placeholder=format!("Tamanho do Monitor: {}", pc.tamMonitor.clone()) class="validate" required=false style="border: 2px solid #ccc;
                                                                                                                                        border-radius: 4px;
                                                                                                                                        padding: 10px;
                                                                                                                                        width: 100%;
                                                                                                                                        resize: none;
                                                                                                                                        transition: .3s;
                                                                                                                                        &:focus {
                                                                                                                                            border: 2px solid #999;
                                                                                                                                            outline: none;
                                                                                                                                        }" oninput=self.link.callback(|input: InputData| Msg::Payload(8, input.value))/>
                            </div>
                            <div class="input-holder">
                            <input type="text" placeholder=format!("Memória Ram: {}", pc.ram.clone()) class="validate" required=false style="border: 2px solid #ccc;
                                                                                                                                        border-radius: 4px;
                                                                                                                                        padding: 10px;
                                                                                                                                        width: 100%;
                                                                                                                                        resize: none;
                                                                                                                                        transition: .3s;
                                                                                                                                        &:focus {
                                                                                                                                            border: 2px solid #999;
                                                                                                                                            outline: none;
                                                                                                                                        }" oninput=self.link.callback(|input: InputData| Msg::Payload(9, input.value))/>
                            </div>
                            <div class="input-holder">
                            <input type="text" placeholder=format!("Status do Sistema: {}", pc.status.clone()) class="validate" required=false style="border: 2px solid #ccc;
                                                                                                                                        border-radius: 4px;
                                                                                                                                        padding: 10px;
                                                                                                                                        width: 100%;
                                                                                                                                        resize: none;
                                                                                                                                        transition: .3s;
                                                                                                                                        &:focus {
                                                                                                                                            border: 2px solid #999;
                                                                                                                                            outline: none;
                                                                                                                                        }" oninput=self.link.callback(|input: InputData| Msg::Payload(10, input.value))/>
                            </div>
                            <div class="input-holder">
                            <textarea class="validate" style="border: 2px solid #ccc;
                                                            border-radius: 4px;
                                                            padding: 10px;
                                                            width: 100%;
                                                            resize: none;
                                                            transition: .3s;
                                                            &:focus {
                                                                border: 2px solid #999;
                                                                outline: none;
                                                            }">{format!("Problemas: {:?}", pc.problemas.clone())}</textarea>
                            <span class="message">{"Ok. I'll go first. I'm a big fan of the New York Giants. You?"}</span>
                            </div>
                            <div class="input-holder">
                            <textarea style="border: 2px solid #ccc;
                                            border-radius: 4px;
                                            padding: 10px;
                                            width: 100%;
                                            resize: none;
                                            transition: .3s;
                                            &:focus {
                                                border: 2px solid #999;
                                                outline: none;
                                            }">{format!("Serviços: {:?}", pc.servicos.clone())}</textarea>
                            </div>
                        </div>
                        // <button class="submit" onclick=self.link.callback(|_| Msg::Submit)>{"Submit"}</button>
                    </form>
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

impl Component for Admin {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|_msg: Msg| Msg::GetInfo);
        callback.emit(Msg::GetInfo);
        Self{
            props,
            view: html!{},
            debug_setor: String::default(),
            debug_id: String::default(),
            debug_hdd: String::default(),
            debug_cpu: String::default(),
            debug_os: String::default(),
            debug_user: String::default(),
            debug_marca: String::default(),
            debug_monitor: String::default(),
            debug_tamMonitor: String::default(),
            debug_ram: String::default(),
            debug_status: String::default(),
            debug_problemas: Vec::new(),
            debug_servicos: Vec::new(),
            fetch_task: None,
            pc: None,
            link,
            error: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use Msg::*;

        match msg {
        Payload(option, modify) => {
            match option
            {
                 0 => {self.debug_user = modify; caller::write_user_data_user(JsValue::from_f64(self.props.id.clone().parse::<f64>().unwrap()), JsValue::from_str(Box::leak(self.debug_user.clone().into_boxed_str())));},
                 1 => {self.debug_setor = modify; caller::write_user_data_setor(JsValue::from_f64(self.props.id.clone().parse::<f64>().unwrap()), JsValue::from_str(Box::leak(self.debug_setor.clone().into_boxed_str())));},
                 2 => {self.debug_id = modify; caller::write_user_data_id(JsValue::from_f64(self.props.id.clone().parse::<f64>().unwrap()), JsValue::from_str(Box::leak(self.debug_id.clone().into_boxed_str())));},
                 3 => {self.debug_hdd = modify; caller::write_user_data_hdd(JsValue::from_f64(self.props.id.clone().parse::<f64>().unwrap()), JsValue::from_str(Box::leak(self.debug_hdd.clone().into_boxed_str())));},
                 4 => {self.debug_cpu = modify; caller::write_user_data_cpu(JsValue::from_f64(self.props.id.clone().parse::<f64>().unwrap()), JsValue::from_str(Box::leak(self.debug_cpu.clone().into_boxed_str())));},
                 5 => {self.debug_os = modify; caller::write_user_data_os(JsValue::from_f64(self.props.id.clone().parse::<f64>().unwrap()), JsValue::from_str(Box::leak(self.debug_os.clone().into_boxed_str())));},
                 6 => {self.debug_marca = modify; caller::write_user_data_marca(JsValue::from_f64(self.props.id.clone().parse::<f64>().unwrap()), JsValue::from_str(Box::leak(self.debug_marca.clone().into_boxed_str())));},
                 7 => {self.debug_monitor = modify; caller::write_user_data_monitor(JsValue::from_f64(self.props.id.clone().parse::<f64>().unwrap()), JsValue::from_str(Box::leak(self.debug_monitor.clone().into_boxed_str())));},
                 8 => {self.debug_tamMonitor = modify; caller::write_user_data_tam_monitor(JsValue::from_f64(self.props.id.clone().parse::<f64>().unwrap()), JsValue::from_str(Box::leak(self.debug_tamMonitor.clone().into_boxed_str())));},
                 9 => {self.debug_ram = modify; caller::write_user_data_ram(JsValue::from_f64(self.props.id.clone().parse::<f64>().unwrap()), JsValue::from_str(Box::leak(self.debug_ram.clone().into_boxed_str())));},
                 10 => {self.debug_status = modify; caller::write_user_data_status(JsValue::from_f64(self.props.id.clone().parse::<f64>().unwrap()), JsValue::from_str(Box::leak(self.debug_status.clone().into_boxed_str())));},
                //  10 => {self.debug_problemas = modify; caller::write_user_data_setor(JsValue::from_f64(self.props.id.clone().parse::<f64>().unwrap()), JsValue::from_str(Box::leak(self.debug_setor.clone().into_boxed_str())));},
                //  11 => {self.debug_status = modify; caller::write_user_data_setor(JsValue::from_f64(self.props.id.clone().parse::<f64>().unwrap()), JsValue::from_str(Box::leak(self.debug_setor.clone().into_boxed_str())));},
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
        // caller::write_user_data(JsValue::from_f64(self.props.id.clone().parse::<f64>().unwrap()), JsValue::from_str("test"), JsValue::from_str("test"), JsValue::from_str("test"), JsValue::from_str("test"), JsValue::from_str("test"), JsValue::from_str("test"), JsValue::from_str("test"), JsValue::from_str("test"), JsValue::from_str("test"), JsValue::from_str("test"), JsValue::from_str("test"));
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
                                    color: #ff6d6d;
                                    z-index: 1;
                                    text-align: center;
                                    /* left: 40%; */
                                    top: 10%;">{"ADMIN PAGE"}</h1>
                        <h4 style="font-size: 150%;
                                    font-weight: 400;
                                    z-index: 1;
                                    text-align: center;
                                    /* left: 40%; */
                                    top: 20%;">{format!("Você está editando o computerID: {}", self.props.id.clone())}</h4>
                    </div>
                <div class="container">
                    <div class="components" style="width: 100%; margin-top: 60px; margin-bottom: 100px; grid-template-columns: auto;overflow: auto;">
                    // <h1>{self.debug_setor.clone()}</h1>
                        {self.view_html()}
                    </div>
                </div>
                <div class="level-item" style="padding-top: 100px; padding-bottom: 40px">
                        <img src="https://img.icons8.com/material-sharp/24/000000/github.png"/><h1 style="font-family: 'Oswald', sans-serif;
                                        color: #383741;
                                        font-size: 100%;
                                        font-weight: 700;">
                                <a href="https://github.com/andrrff">{" andrrff GitHub"}</a></h1>
                                </div>
            </>
        }
    }
}
