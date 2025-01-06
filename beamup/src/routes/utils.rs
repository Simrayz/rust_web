use rust_html::Template;

pub struct RouteOptions {
    pub content: Template,
    pub head: Template,
    pub title: &'static str,
    pub path: &'static str,
    pub exact: bool,
}

impl Default for RouteOptions {
    fn default() -> Self {
        Self {
            content: "".into(),
            head: "".into(),
            title: "",
            path: "",
            exact: false,
        }
    }
}
