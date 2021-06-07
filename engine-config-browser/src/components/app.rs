use itertools::Itertools;
use yew::prelude::*;
use yew_router::prelude::*;

use super::Header;
use crate::{AppRoute, DATABASE};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Msg {
    Update,
}

pub struct App {
    link: ComponentLink<Self>,
    router: Box<dyn Bridge<RouteAgent>>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let router = RouteAgent::bridge(link.callback(|_| Msg::Update));
        App { link, router }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Update => true,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let renderer = Router::<AppRoute>::render(|switch| match switch {
            AppRoute::Index => html! {
                <table>
                <tr class="table-header">
                    <td>{ "Name" }</td>
                    <td>{ "Engine Type"}</td>
                    <td>{ "Manufacturer" }</td>
                    <td>{ "Number of Configs" }</td>
                    <td>{ "Default Config" }</td>
                </tr>
                { for DATABASE.engines.iter().sorted_by_key(|&(k, _)| k).map(|(name, eng)| html! {
                    <tr>
                        <td class ="table-header">{&name}</td>
                        <td>{ &eng.engine_type }</td>
                        <td>{ &eng.manufacturer }</td>
                        <td>{ eng.engine_configs.len() }</td>
                        <td>{ &eng.default_config }</td>
                    </tr>
                }) }
                </table>
            },
            AppRoute::Engine(engine) => html! {},
        });

        html! {
            <>
            <Header />
            <main>
                <Router<AppRoute> render=renderer />
            </main>
            </>
        }
    }
}
