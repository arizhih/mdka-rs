//! HTML to Markdown converter - Lightweight library written in Rust.

const INDENT_DEFAULT_SIZE: usize = 0;
const INDENT_UNIT_SIZE: usize = 4;

pub static ENCLOSURE_ATTRS: OnceLock<Vec<&str>> = OnceLock::new();

pub static CONFIG: OnceLock<Config> = OnceLock::new();

mod components;
mod utils;

use std::sync::{OnceLock};
use crate::utils::node::parse_html;

/// Convert HTML to Markdown
///
/// ```
/// use mdka::from_html;
///
/// let input = r#"
/// <h1>heading 1</h1>
/// <p>Hello, world.</p>"#;
/// let expect = "# heading 1\n\nHello, world.\n\n";
/// let ret = from_html(input);
/// assert_eq!(ret, expect);
/// ```
///
pub fn from_html(html: &str) -> String {
    CONFIG.get_or_init(|| Config::default());
    let dom = parse_html(html);
    components::node::node_md(&dom.document, None::<usize>)
}

#[derive(Debug)]
pub struct Config {
    pub enclosed_attrs : Vec<&'static str>,
    pub bold: &'static str,
    pub italic : &'static str,
    pub ignore_elems : Vec<&'static str>,
    pub br: &'static str,
    pub hr: &'static str,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            enclosed_attrs: vec!["id", "style"],
            bold: "**",
            italic: "*",
            ignore_elems: vec![],
            br: "    \n",
            hr: "\n---\n",
        }
    }
}