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
                <ol class="gradient-list" style="margin-left: 30px; margin-right: 30px;">
                <li style="background: white; min-width: auto;  
                                        border: none;
                                        border-radius: 1rem;
                                        font-size: 1.4rem;
                                        padding-left: 3.8rem;
                                        box-shadow: inset 0.2rem 0.2rem 0.5rem var(--greyLight-2), inset -0.2rem -0.2rem 0.5rem var(--white);
                                        background: none;
                                        font-family: inherit;
                                        color: #9baacf;">
                                <strong><h2 class="text_config" style="font-size: 150%;font-weight: 1000; padding-bottom: 20px; color: #6d5dfc">
                                    {self.props.user.clone()}{":"}
                                </h2></strong>
                                <strong><h2 class="text_config">
                                    {"Setor: "}{self.props.setor.clone()}
                                </h2></strong>
                                <strong><h2 class="text_config">
                                    {"Computer ID: "}{self.props.id.clone()}
                                </h2></strong>
                                <strong><h2 class="text_config">
                                    {"HDD: "}{self.props.hdd.clone()}
                                </h2></strong>
                                <strong><h2 class="text_config">
                                    {"CPU: "}{self.props.cpu.clone()}
                                </h2></strong>
                                <strong><h2 class="text_config">
                                    {"OS: "}{self.props.os.clone()}
                                </h2></strong>
                                <strong><h2 class="text_config">
                                    {"Marca da Máquina: "}{self.props.marca.clone()}
                                </h2></strong>
                                <strong><h2 class="text_config">
                                    {"Monitor: "}{self.props.monitor.clone()}
                                </h2></strong>
                                <strong><h2 class="text_config">
                                    {"Tamanho do monitor: "}{self.props.monitor.clone()}
                                </h2></strong>
                                <strong><h2 class="text_config">
                                    {"Mem. Ram: "}{self.props.ram.clone()}
                                </h2></strong>
                                <strong><h2 class="text_config">
                                    {"Status do Sistema: "}{self.props.status.clone()}
                                </h2></strong>
                            </li>
                </ol>
            </>
        }
    }
}
