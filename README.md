# struct2md

[<img alt="crates.io" src="https://img.shields.io/crates/v/struct2md.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/struct2md)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-struct2md-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/struct2md)
[<img alt="license" src="https://img.shields.io/badge/license-MIT%2FApache--2.0-blue?style=for-the-badge" height="20">](https://github.com/wipn3/struct2md)

This library provides a derive macro to format Rust structs into Markdown via field attributes.

```bash
cargo add struct2md
```

## Example

```rust
use struct2md::ToMarkdown;

#[derive(ToMarkdown)]
struct Issue {
    #[md(heading = 1, bold, plain)]
    title: String,

    #[md(heading = 3, quote)]
    description: String,

    #[md(heading = 3, list = "ordered")]
    steps: Vec<String>,

    #[md(heading = 3, list = "unordered")]
    tags: Vec<String>,

    // Fields without #[md] attributes are ignored
    author_id: u64,
}

fn main() {
    let issue = Issue {
        title: "Crash on startup".to_string(),
        description: "Application panics when config is missing.".to_string(),
        steps: vec![
            "Clone repository".to_string(),
            "Run cargo run --bin app".to_string(),
        ],
        tags: vec!["bug".to_string(), "backend".to_string()],
        author_id: 42,
    };

    println!("{}", issue.to_markdown());
}
```

Output:

```markdown
# **Crash on startup**
### description:
> Application panics when config is missing.
### steps:
1. Clone repository
2. Run cargo run --bin app
### tags:
- bug
- backend
```

## Field Attributes

| Attribute | Parameter | Description |
|---|---|---|
| `#[md]` | - | Includes field in output, delegating formatting to `ToMarkdown` |
| `#[md(plain)]` | Flag | Omits the default `field_name:` prefix |
| `#[md(heading = N)]` | `u8` (1..=6) | Prepends heading level (`#`, `##`, `###`, etc.) |
| `#[md(prefix = "...")]` | `&str` | Overrides the field prefix string |
| `#[md(bold)]` | Flag | Formats text with `**bold**` |
| `#[md(italic)]` | Flag | Formats text with `*italic*` |
| `#[md(code)]` | Flag | Formats text with inline `` `code` `` |
| `#[md(strikethrough)]` | Flag | Formats text with `~~strikethrough~~` |
| `#[md(highlight)]` | Flag | Formats text with `==highlight==` |
| `#[md(subscript)]` | Flag | Formats text with `~subscript~` |
| `#[md(superscript)]` | Flag | Formats text with `^superscript^` |
| `#[md(quote)]` | Flag | Prepends `> ` blockquote marker |
| `#[md(list = "...")]` | `"unordered"`, `"ordered"`, `"definition"` | Formats vector elements into Markdown lists |

Multiple formatting attributes can be combined on a single field:

```rust
#[md(bold, italic, code)]
identifier: String, // Output: identifier: ***`value`***
```

## License

Licensed under either of:

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>) at your option.
