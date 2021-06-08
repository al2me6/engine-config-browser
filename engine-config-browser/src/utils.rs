use yew::Properties;

pub fn document() -> web_sys::Document {
    window().document().unwrap()
}

pub fn window() -> web_sys::Window {
    web_sys::window().unwrap()
}

#[derive(Clone, PartialEq, Eq, Debug, Properties)]
pub struct ConfigName {
    pub name: String,
}
