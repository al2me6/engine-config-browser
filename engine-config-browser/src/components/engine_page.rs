use yew::prelude::*;
use yew_router::prelude::*;
use yewtil::NeqAssign;

use crate::utils::ConfigName;
use crate::{AppRoute, DATABASE, RO_REPO};

pub struct EnginePage {
    pub config: ConfigName,
}

impl Component for EnginePage {
    type Message = ();
    type Properties = ConfigName;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        EnginePage { config: props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.config.neq_assign(props)
    }

    fn view(&self) -> Html {
        let eng = &DATABASE.engines[&self.config.name];

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
            </>
        }
    }
}
