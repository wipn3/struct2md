//! A library providing a derive macro to format Rust structs into Markdown.
//!
//! # Example
//!
//! ```
//! use struct2md::ToMarkdown;
//!
//! #[derive(ToMarkdown)]
//! struct Section {
//!     #[md(heading = 1, bold, plain)]
//!     title: String,
//!     #[md(prefix = "> ")]
//!     summary: String,
//! }
//!
//! let section = Section {
//!     title: "Overview".to_string(),
//!     summary: "Generated documentation.".to_string(),
//! };
//!
//! assert_eq!(section.to_markdown(), "# **Overview**\n> Generated documentation.\n");
//! ```

pub use struct2md_derive::ToMarkdown;

/// Converts a type into its Markdown string representation.
pub trait ToMarkdown {
    fn to_markdown(&self) -> String;
}

impl ToMarkdown for String {
    fn to_markdown(&self) -> String {
        self.clone()
    }
}

impl ToMarkdown for str {
    fn to_markdown(&self) -> String {
        self.to_string()
    }
}

impl ToMarkdown for bool {
    fn to_markdown(&self) -> String {
        self.to_string()
    }
}

macro_rules! impl_to_markdown_display {
    ($($t:ty),*) => {
        $(
            impl ToMarkdown for $t {
                fn to_markdown(&self) -> String {
                    self.to_string()
                }
            }
        )*
    };
}

impl_to_markdown_display!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64);

impl<T: ToMarkdown> ToMarkdown for Option<T> {
    fn to_markdown(&self) -> String {
        match self {
            Some(v) => v.to_markdown(),
            None => String::new(),
        }
    }
}

impl<T: ToMarkdown> ToMarkdown for Vec<T> {
    fn to_markdown(&self) -> String {
        self.iter().map(|item| item.to_markdown()).collect()
    }
}

impl<T: ToMarkdown> ToMarkdown for [T] {
    fn to_markdown(&self) -> String {
        self.iter().map(|item| item.to_markdown()).collect()
    }
}
