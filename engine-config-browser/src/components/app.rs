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
            AppRoute::Index => Self::view_index(),
            AppRoute::Engine(engine) => html! {},
        });

        html! {
            <>
            <Header />
            <main>
                <div id="content-container">
                    <p>
                        { "Viewing data from " }
                        <a
                            href=format!("https://github.com/KSP-RO/RealismOverhaul/commit/{}", DATABASE.commit)
                            target="_blank"
                        >
                            { "commit " }
                            { &DATABASE.commit[0..7] }
                        </a>
                        { format!(", dated {} UTC.", DATABASE.timestamp) }
                    </p>
                    <Router<AppRoute> render=renderer />
                </div>
            </main>
            </>
        }
    }
}

impl App {
    fn view_index() -> Html {
        html! {
            <table>
            <tr>
                <th>{ "Name" }</th>
                <th>{ "Type"}</th>
                <th>{ "Manufacturer" }</th>
                <th>{ "Number of Configs" }</th>
            </tr>
            { for DATABASE.engines.iter().sorted_by_key(|&(k, _)| k).map(|(name, eng)| html! {
                <tr>
                    <td class ="table-header">{&name}</td>
                    <td>{ &eng.engine_type }</td>
                    <td>{ &eng.manufacturer }</td>
                    <td>{ eng.engine_configs.len() }</td>
                </tr>
            }) }
            </table>
        }
    }
}
