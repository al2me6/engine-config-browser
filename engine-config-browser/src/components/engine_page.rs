use yew::prelude::*;
use yew_router::prelude::*;
use yewtil::NeqAssign;

use super::EngineConfigInfo;
use crate::{AppRoute, DATABASE, RO_REPO};

#[derive(Clone, PartialEq, Properties)]
pub struct EnginePageProps {
    pub name: String,
}

pub struct EnginePage {
    pub props: EnginePageProps,
}

impl Component for EnginePage {
    type Message = ();
    type Properties = EnginePageProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        EnginePage { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let eng = &DATABASE.engines[&self.props.name];

        let bullet = |name: &str, value: &str| {
            html! {
                <li>
                    <b>{ name }</b>
                    { ": " }
                    { value }
                </li>
            }
        };

        let file_name = format!("{}.cfg", eng.file_name);

        html! {
            <>
            <RouterButton<AppRoute> route=AppRoute::Index>
                { "Back" }
            </RouterButton<AppRoute>>
            <h2>
                { &eng.title }
                { " [" }
                <code><a
                    href=format!(
                        "{}/tree/{}/GameData/RealismOverhaul/Engine_Configs/{}",
                        RO_REPO,
                        DATABASE.commit,
                        file_name,
                    )
                    target="_blank"
                >
                    { file_name }
                </a></code>
                { "]" }
            </h2>
            <ul>
                { if let Some(ref description) = eng.description {
                    bullet("Description", description)
                } else {
                    html!{}
                }}
                { bullet("Type", &eng.engine_type.to_string()) }
                { bullet("Manufacturer", &eng.manufacturer) }
            </ul>
            { for eng.engine_configs.keys().map(|config| html!{
                <EngineConfigInfo engine=self.props.name.clone() config=config.clone() />
            }) }
            </>
        }
    }
}
