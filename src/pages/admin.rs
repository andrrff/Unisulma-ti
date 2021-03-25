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
    Submit,
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
                                <button class="submit">{"Reload"}</button>
                                <button class="submit-delete" onclick=self.link.callback(|_| Msg::Submit) style="padding-top: 5px">{"🚧Deletar🚧"}</button>
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
            allow_modify: false,
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
                 13 => {self.debug_password 
                    = modify; if self.debug_password == "NBxT@9#JWGcj=-zbuMDz5j3*N3$P++dJBGH&5qFGhNkBV3Kkn!sej79W_CDapPs-YdwkWVyP8m$CjhPgR%e$?7%_ED7wZk4c=j=!KF-$@2^3pWucWrVmthLpsP^6PFgj=+@awvq_K+?hd3BxbE?&KGcp+q6H!@hgd57uDFd_KM%mHWS_@gXS^2LS4*9wdf-zSLsVK#J3qWGuA8E-a?ZC_3smGVe4&%#nvSSP&A6KQtV2_Us*ZgmE!&MhBMm$?32ZD9xDzjVq$XQFR2K8QpYetFbNPrxd9WTtWPS9Hk2U5@W*_W3mfny=3ax7+TSdkAas4x9RQ6MCw2!ad&7Ns%zBAfL8d7u*QkP3D3z?r&JKaF56pJ6dKp_PnR8*ySEAYcF%mpL?W8@^g3Ekj2te6z+v5&M#$qP7GVx54@zL*LPAnsCqz2jbMams_ZEseeS^Qp9wvUf3%jst4WYkXJWQ^u-EZGs^p2e#G=pPM?e=#Y6MvU5q7x+4pCdNF4asvkbYQeMCBHRChvAU^9#_fZcbUTRnEH%HcUGU-7x33B%YEz3QgJWzu9uDEbdkr3^vsS*KL7BRAEcSP55Udk%SEMzWf=*cASxVS%%t-@BuPwsF5wUXjP^@4aTAr&AkyH*hUSKsk&+4J%3VeV!WD%mxGFnrLuMztUygqB7-H#uSRthJKhA7&ykq*HZL#GLTVA3-^*U!b9G*a@xRa^drjNt9z-aCHMu!mLpws&JC^TNa^V6-3Vh+LDe+=ZJjH-4##Pj3gmfPRffRV3neyh$-3Tw=akQ2YN7gutXB9Cph_bv+2CwS8z52@TW*uXn4n7=waS3BQG%X2UpvZrVyE5h_bmbsgAB_zn3%w5pVBP#G69+c*YRyA7pJJh3m_N5s+BSy*7pUNp=P#NUf5fY=%sCUg!bmHKqfc8JcCx7e?gJhvHecVdkr@n9j^Uq3Mgctd?CAn_%mz#?Ma6Kyp%pbD93W?HLNf@z-YgLQ4sHT+puh7Cqj$c%RaAFCA&NLYDYn+7@!4AXgC&cwVwr=xKn?79qjYPz5U6QRx-Xesx%XzU82q4tDx4s6^DEkH!Rk5kPbQy#!t7T=d_kY-!z3!^#aZKu%6Hq3tUJeG3TbRMzV=2BWRsrpsBCVXTAhQT#tcGeQYUHL8k%Zwh-pLDF3p!*wD?=p*Sr=w5#mqXsU3nUx@vV6$2&aVNu88Avj3!-f3&N%AA*_57aQUfmFnr!ekANDrzTFeqKrhwk_wv6Y5JpHAEZw$NH8gsgu-BA7yu_7Wuzu62HT-vLkBrPPz?3byS5uxRJ#Y8xFBPA3wh$Ld-BQntmuYsnpv@CwTAqpJZVwqp2n=H-a4Syn%6-JUXJpVUP+-24M?cZYxaJDHkBzhsnmTF^m+GqTf8_^7Ney_&WjDSx$DNDkc?cmZmJN4SZ*Zz2$R_LQAW5zWJ9=3X7pjeCF_%_BY-fyFzS_bG93^!@?#aMVxFPvzG7jAwrzqVvtKegGqvCxJr#6HVg+#ZR3VUX^JCT59##HMg8CFQe_PxV+neKaq+KzBpk83&U5E8NQSd?-4p=@m$n?v2fxUAYub9!Hd&&vSg!G%y#wG6_+he+!DRQNX%zK*GwmJfuW@gmuJua%pVBcWa$ZW#?7MT4mcTv&a#Gw-bgHMpqQnv%W8wGnCFbM4xyt6cMs7S&7SB%hJmH%_X_54MzT3mtv3vf3TYnz=LSnrvp9@%b-xu2E9paJtWGPRTUcCCxG-M2caPhvaS&26WMS_Q&@uxu+PYEzJ?xW-uve2n4*gXBQ3hW@+mUqNQahdnZTWaXc-ZA5VGL7xnPm-nB!x!3zBP66Vv^3JeEb%FT!?^zgvrYn=uGWb-r=fpuRX%AMcQy!awupkBpv3a!_5+-f=Wce2c4FkyA=u-D7+h7Yz?TRf7=^RFsAw&5YusQ8Gh_!Qu62+_tqQV8KdkNjkmnPT+pR$eTGX6!PNsSe=@2J2t%f7Nr_-EZg_-hqKVjTR9t4QhuQh%w--CXU%C+md^FK-Xu4Ejmy%4U&r6KpkD6zqQg^mj4cs+@9s$kGS2Yz!P+aCZ_NrqP2*=&k".to_string(){self.allow_modify = true;}},
                //  11 => {self.debug_status = modify; caller::write_user_data_setor(JsValue::from_f64(self.props.id.clone().parse::<f64>().unwrap()), JsValue::from_str(Box::leak(self.debug_setor.clone().into_boxed_str())));},
                _ => unimplemented!()
            }
            true
        }
        Submit =>
        {
            // caller::remove_new_pc(JsValue::from_str(""), JsValue::from_str(""), JsValue::from_str(""), JsValue::from_str(""), JsValue::from_str(""), JsValue::from_str(""), JsValue::from_str(""), JsValue::from_str(""), JsValue::from_str(""), JsValue::from_str(""), JsValue::from_str(""), vec![JsValue::from_str("")].into_boxed_slice(), vec![JsValue::from_str("")].into_boxed_slice(), JsValue::from_f64(self.props.id.clone().parse::<f64>().unwrap()));
            // caller::write_new_pc(JsValue::from_str("test"), JsValue::from_str("test"), JsValue::from_str("test"), JsValue::from_str("test"), JsValue::from_str("test"), JsValue::from_str("test"), JsValue::from_str("test"), JsValue::from_str("test"), JsValue::from_str("test"), JsValue::from_str("test"), JsValue::from_str("test"), vec![JsValue::from_str("test")].into_boxed_slice(), vec![JsValue::from_str("test")].into_boxed_slice(), JsValue::from_f64(self.props.id.clone().parse::<f64>().unwrap()));
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
