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
    Submit(u8),
    AddProblema(usize),
    AddServico(usize),
    CurrentProblema(usize),
    CurrentServico(usize),
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
    current_problema: usize,
    current_servico: usize,
    add_problema: Vec<String>,
    add_servico: Vec<String>,
    add_problema_struct: Vec<Html>,
    add_servico_struct: Vec<Html>,
    allow_modify: bool,
    debug_password: String,
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
    debug_problemas: String,
    debug_servicos: String,
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
                let problemas = pc.problemas.clone();
                let servicos = pc.servicos.clone();
                let mut elements_problemas: Vec<Html> = vec![html!{}];
                let mut elements_servicos: Vec<Html> = vec![html!{}];
                if problemas.len().clone() > 1
                {
                    let mut words: Vec<String> = Vec::new();
                    // let problemas = pc.problemas.clone();
                    // let servicos = pc.servicos.clone();
                    // let mut elements_problemas: Vec<Html> = Vec::new();
                    for (e, i) in problemas.iter().enumerate()
                    {
                        let mut name_problemas_data: Vec<char> = Vec::new();
                        let letters = i.to_string().chars().collect::<Vec<char>>();
                        for (j, l) in letters.iter().enumerate()
                        {
                            if j <= 11
                            {
                                name_problemas_data.push(*l);
                            }
                            else
                            {
                                words.push(name_problemas_data.iter().collect::<String>().clone());
                                break;
                            }
                        }
                        elements_problemas.push(html!{
                                        <label>
                                            <input onclick=self.link.callback(move |_| Msg::CurrentProblema(e)) type="radio" name="radio" checked=false/>
                                            <span>{words[e].clone()}</span>
                                        </label>
                                    });
                    }
                }
                if servicos.len().clone() > 1
                {
                    let mut words: Vec<String> = Vec::new();
                    for (e, i) in servicos.iter().enumerate()
                    {
                        let mut name_servicos_data: Vec<char> = Vec::new();
                        let letters = i.to_string().chars().collect::<Vec<char>>();
                        for (j, l) in letters.iter().enumerate()
                        {
                            if j <= 11
                            {
                                name_servicos_data.push(*l);
                            }
                            else
                            {
                                words.push(name_servicos_data.iter().collect::<String>().clone());
                                break;
                            }
                        }
                        elements_servicos.push(html!{
                                        <label>
                                            <input onclick=self.link.callback(move |_| Msg::CurrentServico(e)) type="radio" name="radio" checked=false/>
                                            <span>{words[e].clone()}</span>
                                        </label>
                                    });
                    }
                }
                if self.allow_modify == true
                {
                    info = html!{
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
                                        <fieldset>
                                            <legend>{"Problemas"}</legend>
                                            <details open=false>
                                                <summary>{"Escolha um para editar..."}<span class="material-icons-round"><img src="https://img.icons8.com/material-rounded/24/000000/down-squared.png"/></span></summary>
                                                <div>
                                                    {for elements_problemas.clone()}
                                                    {for self.add_problema_struct.clone()}
                                                    <label>
                                                        <input type="radio" name="radio" checked=true/> //onclick=self.link.callback(move |_| Msg::AddProblema(problemas.clone().len()))
                                                        <span><img src="https://img.icons8.com/metro/26/000000/plus-2-math.png" style="width: 24px"/>{" Adicionar um problema - in build🚧"}</span>
                                                    </label>
                                                </div>
                                            </details>
                                        </fieldset>
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
                                                                    }" placeholder="(??/??/????): Escreva aqui o problema - Seu nome" oninput=self.link.callback(|input: InputData| Msg::Payload(11, input.value))>{pc.problemas[self.current_problema].clone()}</textarea>
                                                                    <button class="submit-confirm" onclick=self.link.callback(|_| Msg::Submit(1)) style="margin-top: 15px; margin-bottom: 100px;">{"Confimar"}</button>
                                    <span class="message">{"Ok. I'll go first. I'm a big fan of the New York Giants. You?"}</span>
                                    </div>
                                    <fieldset>
                                            <legend>{"Serviços"}</legend>
                                            <details open=false>
                                                <summary>{"Escolha um para editar..."}<span class="material-icons-round"><img src="https://img.icons8.com/material-rounded/24/000000/down-squared.png"/></span></summary>
                                                <div>
                                                    {for elements_servicos.clone()}
                                                    {for self.add_servico_struct.clone()}
                                                    <label>
                                                        <input type="radio" name="radio" checked=true/> //onclick=self.link.callback(move |_| Msg::AddServico(servicos.clone().len()))
                                                        <span><img src="https://img.icons8.com/metro/26/000000/plus-2-math.png" style="width: 24px"/>{" Adicionar um serviço - in build🚧"}</span>
                                                    </label>
                                                </div>
                                            </details>
                                        </fieldset>
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
                                                    }" placeholder="(??/??/????): Escreva aqui o serviços - Seu nome" oninput=self.link.callback(|input: InputData| Msg::Payload(12, input.value))>{pc.servicos[self.current_servico].clone()}</textarea>
                                                    <button class="submit-confirm" onclick=self.link.callback(|_| Msg::Submit(2)) style="margin-top: 15px; margin-bottom: 100px;">{"Confimar"}</button>
                                    </div>
                                </div>
                                <button class="submit">{"Reload"}</button>
                                <button class="submit-delete" onclick=self.link.callback(|_| Msg::Submit(0)) style="margin-top: 15px;">{"Deletar"}</button>
                            </form>
                    }
                };
                html! {
                    <>
                        <div class="input-holder" style="padding-top: 20px">
                                    <input type="password" placeholder=format!("Senha") class="validate" required=true style="border: 2px solid #ccc;
                                                                                                                                                border-radius: 4px;
                                                                                                                                                padding: 10px;
                                                                                                                                                width: 100%;
                                                                                                                                                resize: none;
                                                                                                                                                transition: .3s;
                                                                                                                                                &:focus {
                                                                                                                                                    border: 2px solid #999;
                                                                                                                                                    outline: none;
                                                                                                                                                }" oninput=self.link.callback(|input: InputData| Msg::Payload(13, input.value))/>
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
    fn export_problemas(&self) -> Vec<String>
    {
        match self.pc {
            Some(ref pc) => {
                pc.problemas.clone()
            }
            None => {
                vec!["Carregando".to_string()]
            }
        }
    }
    fn export_servicos(&self) -> Vec<String>
    {
        match self.pc {
            Some(ref pc) => {
                pc.servicos.clone()
            }
            None => {
                vec!["Carregando".to_string()]
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
        // let callback = link.callback(|_msg: Msg| Msg::GetInfo);
        // callback.emit(Msg::GetInfo);
        Self{
            props,
            view: html!{},
            allow_modify: false,
            current_problema: 0usize,
            current_servico: 0usize,
            add_problema: Vec::new(),
            add_servico: Vec::new(),
            add_problema_struct: Vec::new(),
            add_servico_struct: Vec::new(),
            debug_password: String::default(),
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
            debug_problemas: String::default(),
            debug_servicos: String::default(),
            fetch_task: None,
            pc: None,
            link,
            error: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use Msg::*;

        match msg {
        AddProblema(value) =>
        {
            // self.add_problema = self.export_problemas();
            let mut name_problemas_data: Vec<char> = Vec::new();
            self.add_problema.push(String::from("(??/??/????): Escreva aqui o problema - Seu nome"));
            self.current_problema = self.add_problema.len().clone();
            let mut elements_problemas: Vec<Html> = Vec::new();
            for (_, i) in self.add_problema.clone().iter().enumerate()
            {
                let letters = i.to_string().chars().collect::<Vec<char>>();
                for (j, l) in letters.iter().enumerate()
                {
                    if j <= 11
                    {
                        name_problemas_data.push(*l);
                    }
                    else
                    {
                        break;
                    }
                }
                let number = self.current_problema.clone();
                elements_problemas.push(html!{
                                <label>
                                    <input onclick=self.link.callback(move |_| Msg::CurrentProblema(number)) type="radio" name="radio" checked=true/>
                                    <span>{"(??/??/????)"}</span>
                                </label>
                            });
            }
            self.add_problema_struct = elements_problemas;
            true
        }
        AddServico(value) =>
        {
            let mut name_servicos_data: Vec<char> = Vec::new();
            let mut words: Vec<String> = Vec::new();
            self.add_servico.push(String::from("(??/??/????): Escreva aqui o problema - Seu nome"));
            self.current_servico = self.add_servico.len().clone();
            let mut elements_servicos: Vec<Html> = Vec::new();
            for (_, i) in self.add_servico.clone().iter().enumerate()
            {
                let letters = i.to_string().chars().collect::<Vec<char>>();
                for (j, l) in letters.iter().enumerate()
                {
                    if j <= 11
                    {
                        name_servicos_data.push(*l);
                    }
                    else
                    {
                        words.push(name_servicos_data.iter().collect::<String>().clone()).clone();
                        break;
                    }
                }
                elements_servicos.push(html!{
                                <label>
                                    <input onclick=self.link.callback(move |_| Msg::CurrentServico(value)) type="radio" name="radio" checked=true/>
                                    <span>{"(??/??/????)"}</span>
                                </label>
                            });
            }
            self.add_servico_struct = elements_servicos;
            true
        }
        CurrentProblema(number) =>
        {
            self.current_problema = number;
            true
        }
        CurrentServico(number) =>
        {
            self.current_servico = number;
            true
        }
        Payload(option, modify) => {
            match option
            {
                 0 => {self.debug_user = modify; if self.allow_modify == true {caller::write_user_data_user(JsValue::from_f64(self.props.id.clone().parse::<f64>().unwrap()), JsValue::from_str(Box::leak(self.debug_user.clone().into_boxed_str())));}},
                 1 => {self.debug_setor = modify; if self.allow_modify == true {caller::write_user_data_setor(JsValue::from_f64(self.props.id.clone().parse::<f64>().unwrap()), JsValue::from_str(Box::leak(self.debug_setor.clone().into_boxed_str())));}},
                 2 => {self.debug_id = modify; if self.allow_modify == true {caller::write_user_data_id(JsValue::from_f64(self.props.id.clone().parse::<f64>().unwrap()), JsValue::from_str(Box::leak(self.debug_id.clone().into_boxed_str())));}},
                 3 => {self.debug_hdd = modify; if self.allow_modify == true {caller::write_user_data_hdd(JsValue::from_f64(self.props.id.clone().parse::<f64>().unwrap()), JsValue::from_str(Box::leak(self.debug_hdd.clone().into_boxed_str())));}},
                 4 => {self.debug_cpu = modify; if self.allow_modify == true {caller::write_user_data_cpu(JsValue::from_f64(self.props.id.clone().parse::<f64>().unwrap()), JsValue::from_str(Box::leak(self.debug_cpu.clone().into_boxed_str())));}},
                 5 => {self.debug_os = modify; if self.allow_modify == true {caller::write_user_data_os(JsValue::from_f64(self.props.id.clone().parse::<f64>().unwrap()), JsValue::from_str(Box::leak(self.debug_os.clone().into_boxed_str())));}},
                 6 => {self.debug_marca = modify; if self.allow_modify == true {caller::write_user_data_marca(JsValue::from_f64(self.props.id.clone().parse::<f64>().unwrap()), JsValue::from_str(Box::leak(self.debug_marca.clone().into_boxed_str())));}},
                 7 => {self.debug_monitor = modify; if self.allow_modify == true {caller::write_user_data_monitor(JsValue::from_f64(self.props.id.clone().parse::<f64>().unwrap()), JsValue::from_str(Box::leak(self.debug_monitor.clone().into_boxed_str())));}},
                 8 => {self.debug_tamMonitor = modify; if self.allow_modify == true {caller::write_user_data_tam_monitor(JsValue::from_f64(self.props.id.clone().parse::<f64>().unwrap()), JsValue::from_str(Box::leak(self.debug_tamMonitor.clone().into_boxed_str())));}},
                 9 => {self.debug_ram = modify; if self.allow_modify == true {caller::write_user_data_ram(JsValue::from_f64(self.props.id.clone().parse::<f64>().unwrap()), JsValue::from_str(Box::leak(self.debug_ram.clone().into_boxed_str())));}},
                 10 => {self.debug_status = modify; if self.allow_modify == true {caller::write_user_data_status(JsValue::from_f64(self.props.id.clone().parse::<f64>().unwrap()), JsValue::from_str(Box::leak(self.debug_status.clone().into_boxed_str())));}},
                 11 => {self.debug_problemas = modify;},
                 12 => {self.debug_servicos = modify;},
                 13 => {self.debug_password = modify; if self.debug_password == "cav2017".to_string(){self.allow_modify = true;}},
                _ => unimplemented!()
            }
            true
        }
        Submit(value) =>
        {
            match value
            {
                0 => {if self.allow_modify == true {caller::write_user_data_visibilidade(JsValue::from_f64(self.props.id.clone().parse::<f64>().unwrap()), JsValue::from_bool(false), JsValue::from_str(Box::leak(format!("{} - DELETADO", self.export_name().clone()).into_boxed_str())));}},
                1 => {if self.allow_modify == true {self.add_problema = self.export_problemas();
            self.add_problema[self.current_problema] = self.debug_problemas.clone();
            // self.add_servico[self.current_servico] = self.debug_servicos.clone();
            let mut problemas: Vec<JsValue> = Vec::new();
            for i in self.add_problema.iter()
            {
                problemas.push(JsValue::from_str(Box::leak(i.clone().into_boxed_str())));
            }caller::write_user_data_problemas(JsValue::from_f64(self.props.id.clone().parse::<f64>().unwrap()), problemas.into_boxed_slice(), JsValue::from_f64(self.current_problema.clone() as f64));}},
                2 => {if self.allow_modify == true {self.add_servico = self.export_servicos();
            self.add_servico[self.current_servico] = self.debug_servicos.clone();
            let mut servicos: Vec<JsValue> = Vec::new();
            for i in self.add_servico.iter()
            {
                servicos.push(JsValue::from_str(Box::leak(i.clone().into_boxed_str())));
            }caller::write_user_data_servicos(JsValue::from_f64(self.props.id.clone().parse::<f64>().unwrap()), servicos.into_boxed_slice(), JsValue::from_f64(self.current_servico.clone() as f64));}},
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
                    <div class="components" style="width: 100%; margin-top: 60px; margin-bottom: 100px; grid-template-columns: auto;overflow: auto; height: auto;">
                    <h1>{"*Os botões estão com seus callback em produção ainda, estão funcionando porém ainda não está 100%, portanto paciência :)"}</h1>
                        {self.view_html()}
                    </div>
                </div>
                <div class="level-item" style="padding-top: 100px; padding-bottom: 40px">
                        <img src="https://img.icons8.com/material-sharp/24/000000/github.png" style="max-width: 24px;"/><h1 style="font-family: 'Oswald', sans-serif;
                                        color: #383741;
                                        font-size: 100%;
                                        font-weight: 700;">
                                <a href="https://github.com/andrrff">{" andrrff GitHub"}</a></h1>
                                </div>
            </>
        }
    }
}
