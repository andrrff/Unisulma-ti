use yew::prelude::*;

use crate::components::{info_cpu, info_hd};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub cpu: String,
    pub hdd: String
}

pub struct Open
{
    props: Props
}

impl Component for Open {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self
        {
            props
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html
    {
        html!{
            <>
                // <info_cpu::LoadInfo cpu=self.props.cpu.clone()/>
            </>
        }        
    }
}
