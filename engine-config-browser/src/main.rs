#![warn(clippy::pedantic)]
#![allow(
    clippy::default_trait_access,
    clippy::missing_panics_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
)]

use engine_config::EngineDatabase;
use once_cell::sync::Lazy;
use yew_router::Switch;

pub mod utils;
pub mod components {
    pub mod app;
    pub mod engine_list;
    pub mod engine_page;
    pub mod header;

    pub use app::App;
    pub use engine_list::EngineList;
    pub use header::Header;
    pub use engine_page::EnginePage;
}

use crate::components::App;

pub const DATA_BIN: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/data.bin"));
pub static DATABASE: Lazy<EngineDatabase> = Lazy::new(|| bincode::deserialize(DATA_BIN).unwrap());

pub const RO_REPO: &str = "https://github.com/KSP-RO/RealismOverhaul";

#[derive(Debug, Clone, Switch)]
pub enum AppRoute {
    // #[to = "/#{engine}/{config}"]
    // EngineConfig { engine: String, config: String },
    #[to = "/#{}"]
    Engine(String),
    #[to = "/"]
    Index,
}

pub fn main() {
    yew::start_app::<App>();
}
