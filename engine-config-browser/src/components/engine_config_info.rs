use std::fmt::Display;

use yew::prelude::*;
use yewtil::NeqAssign;

use crate::DATABASE;

#[derive(Clone, PartialEq, Properties)]
pub struct EnginePageProps {
    pub engine: String,
    pub config: String,
}

pub struct EngineConfigInfo {
    pub props: EnginePageProps,
}

impl Component for EngineConfigInfo {
    type Message = ();
    type Properties = EnginePageProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        EngineConfigInfo { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        fn bullet(name: &str, value: impl Display, unit: &str) -> Html {
            html! {
                <li>
                    <b>{ name }</b>
                    { format!(": {}{}", value, unit) }
                </li>
            }
        }

        let engine = &DATABASE.engines[&self.props.engine];
        let config = &engine.engine_configs[&self.props.config];

        html! {
            <>
            <h3>
                { format!("Configuration {}", self.props.config) }
            </h3>
            <ul>
                { if engine.default_config == self.props.config {
                    html! { <li><b>{ "Is default configuration" }</b></li> }
                } else {
                    html! {}
                }}
                { if config.config_description.is_empty() {
                    html!{}
                } else {
                    bullet("Description", &config.config_description, "")
                }}
                { bullet(
                    "Mass",
                    format!(
                        "{:.0}",
                        engine.original_mass
                            * if config.mass_mult == 0.0 { 1.0 } else { config.mass_mult }
                            * 1000_f32
                    ),
                    " kg"
                )}
                { bullet("SL Isp", format!("{:.1}", config.isp_sea_level), "s") }
                { bullet("Vac Isp", format!("{:.1}", config.isp_vacuum), "s") }
                { bullet("Min thrust", format!("{:.1}", config.min_thrust), " kN") }
                { bullet("Max thrust", format!("{:.1}", config.max_thrust), " kN") }
                { bullet("Min throttle", format!("{:.0}", config.min_throttle * 100_f32), "%") }
                <li>
                    <b>{ "Fuels:" }</b>
                    <ul>
                        { bullet("Requires ullage", config.ullage, "") }
                        { bullet("Is pressure fed", config.pressure_fed, "") }
                        { for config.propellants.iter().map(|(fuel, amount)| html! {
                            bullet(fuel, format!("{:.1}", amount * 100_f32), "%")
                        })}
                    </ul>
                </li>
                { bullet("Ignitions", "TODO", "") }
            </ul>
            </>
        }
    }
}
