use std::fmt;

use rust_html::{rhtml, Render, Template};

#[allow(dead_code)]
pub enum ButtonColor {
    Primary,
    Secondary,
    Alternate,
    Info,
    Success,
    Warning,
    Danger,
}

impl fmt::Display for ButtonColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let base_class = "cursor-pointer whitespace-nowrap rounded-md px-4 py-2 text-sm font-medium tracking-wide transition hover:opacity-75 text-center focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-black active:opacity-100 active:outline-offset-0 disabled:opacity-75 disabled:cursor-not-allowed dark:focus-visible:outline-white";
        let class = match self {
            ButtonColor::Primary => "bg-black text-white dark:bg-white dark:text-black dark:focus-visible:outline-white",
            ButtonColor::Secondary => "bg-neutral-800 text-white focus-visible:outline-neutral-800 dark:bg-neutral-300 dark:text-black dark:focus-visible:outline-neutral-300",
            ButtonColor::Alternate => "bg-neutral-50 text-neutral-900 focus-visible:outline-neutral-50 dark:bg-neutral-900  dark:focus-visible:outline-neutral-900",
            ButtonColor::Info => "bg-sky-500 text-white focus-visible:outline-sky-500 dark:bg-sky-500 dark:text-white dark:focus-visible:outline-sky-500",
            ButtonColor::Success => "bg-green-500 text-white focus-visible:outline-green-500 active:opacity-100 dark:bg-green-500 dark:focus-visible:outline-green-500",
            ButtonColor::Warning => "bg-amber-500 text-white dark:bg-amber-500 dark:text-white dark:focus-visible:outline-amber-500",
            ButtonColor::Danger => "bg-red-500 text-white focus-visible:outline-red-500 dark:bg-red-500 dark:text-white dark:focus-visible:outline-red-500",
        };
        write!(f, "{}", [base_class, class].join(" "))
    }
}

pub struct Button {
    pub content: String,
    pub color: ButtonColor,
}

impl Render for Button {
    fn render(&self) -> Template {
        let button_classes = &self.color;

        rhtml! { r#"
            <button class="{button_classes}">{self.content}</button>
        "# }
    }
}
