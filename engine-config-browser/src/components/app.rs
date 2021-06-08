use yew::prelude::*;
use yew_router::prelude::*;

use super::Header;
use crate::components::{EngineList, EnginePage};
use crate::{AppRoute, DATABASE, RO_REPO};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Msg {
    Update,
}

pub struct App {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        App {}
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
            AppRoute::Index => html! { <EngineList /> },
            AppRoute::Engine(eng) => html! { <EnginePage name=eng /> },
        });

        html! {
            <>
            <Header />
            <div id="content-container">
                <main>
                    <p>
                        { "Viewing data from " }
                        <a
                            href=format!("{}/commit/{}", RO_REPO, DATABASE.commit)
                            target="_blank"
                        >
                            { "commit " }
                            <code>
                                { &DATABASE.commit[0..7] }  // This is fine, commit hashes are ASCII.
                            </code>
                        </a>
                        { format!(", dated {} UTC.", DATABASE.timestamp) }
                    </p>
                    <Router<AppRoute> render=renderer />
                    <footer>
                        { "Engine Config Browser by Al2Me6 | " }
                        <a href="https://github.com/al2me6/engine-config-browser" target="_blank">
                            { "Source" }
                        </a>
                    </footer>
                </main>
            </div>
            </>
        }
    }
}

// Parent Engine
// Config
// Tech Unlock
// Cost
// Mass
// Ignitions
// Ground Lit Only?
// Fuel
// Oxidizer
// SL Thrust
// Vac Thrust
// SL ISP
// Vac ISP
// Country (might have to add this)
