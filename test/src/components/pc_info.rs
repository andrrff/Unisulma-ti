use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub setor: String,
    pub id: String,
    pub hdd: String,
    pub cpu: String,
    pub os: String,
    pub user: String,
    pub marca: String,
    pub monitor: String,
    pub tamMonitor: String,
    pub ram: String,
    pub status: String
}

pub struct Pc {
    props: Props,
}
impl Component for Pc {
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
                <ol class="gradient-list" style="margin-left: 30px; margin-right: 30px; margin-top: 100px;">
                                <strong><h2 class="text_config_01">
                                    {"Computer ID: "}{self.props.id.clone()}
                                </h2></strong>
                                <strong><h2 class="text_config_01">
                                    {"HDD: "}{self.props.hdd.clone()}
                                </h2></strong>
                                <strong><h2 class="text_config_01">
                                    {"CPU: "}{self.props.cpu.clone()}
                                </h2></strong>
                                <strong><h2 class="text_config_01">
                                    {"OS: "}{self.props.os.clone()}
                                </h2></strong>
                                <strong><h2 class="text_config_01">
                                    {"Marca da Máquina: "}{self.props.marca.clone()}
                                </h2></strong>
                                <strong><h2 class="text_config_01">
                                    {"Monitor: "}{self.props.monitor.clone()}
                                </h2></strong>
                                <strong><h2 class="text_config_01">
                                    {"Tamanho do monitor: "}{self.props.monitor.clone()}
                                </h2></strong>
                                <strong><h2 class="text_config_01">
                                    {"Mem. Ram: "}{self.props.ram.clone()}
                                </h2></strong>
                                <strong><h2 class="text_config_01">
                                    {"Status do Sistema: "}{self.props.status.clone()}
                                </h2></strong>
                </ol>
            </>
        }
    }
}
