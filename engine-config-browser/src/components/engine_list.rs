use itertools::Itertools;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{AppRoute, DATABASE};

pub struct EngineList {}

impl Component for EngineList {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        EngineList {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <table>
            <tr>
                <th>{ "Name" }</th>
                <th>{ "Type"}</th>
                <th>{ "Manufacturer" }</th>
                <th>{ "Number of Configs" }</th>
            </tr>
            { for DATABASE.engines.iter().sorted_by_key(|&(k, _)| k).map(|(file_name, eng)| html! {
                <tr>
                    <td class ="table-header">
                        <RouterAnchor<AppRoute>
                            route = AppRoute::Engine(file_name.clone())
                        >
                            { &eng.title }
                        </RouterAnchor<AppRoute>>
                    </td>
                    <td>{ &eng.engine_type }</td>
                    <td>{ &eng.manufacturer }</td>
                    <td>{ eng.engine_configs.len() }</td>
                </tr>
            }) }
            </table>
        }
    }
}
