use struct2md::ToMarkdown;

#[test]
fn test_field_whitelist() {
    #[derive(ToMarkdown)]
    struct Container {
        #[md(plain)]
        included: String,
        #[allow(dead_code)]
        ignored: String,
    }

    let c = Container {
        included: "public".to_string(),
        ignored: "secret".to_string(),
    };

    assert_eq!(c.to_markdown(), "public\n");
}

#[test]
fn test_primitive_types() {
    #[derive(ToMarkdown)]
    struct Primitives<'a> {
        #[md(plain)]
        text: String,
        #[md(plain)]
        flag: bool,
        #[md(plain)]
        int: i32,
        #[md(plain)]
        float: f64,
        #[md(plain)]
        character: char,
        #[md(plain)]
        big_u: u128,
        #[md(plain)]
        arr: [i32; 3],
        #[md(plain)]
        slice: &'a str,
        #[md(plain)]
        opt_some: Option<String>,
        #[md(plain)]
        opt_none: Option<String>,
    }

    let p = Primitives {
        text: "hello".to_string(),
        flag: true,
        int: 42,
        float: 3.14,
        character: 'Z',
        big_u: 100_000_000_000_000_000_000,
        arr: [10, 20, 30],
        slice: "borrowed",
        opt_some: Some("present".to_string()),
        opt_none: None,
    };

    let expected = r#"hello
true
42
3.14
Z
100000000000000000000
10, 20, 30
borrowed
present
"#;
    assert_eq!(p.to_markdown(), expected);
}

#[test]
fn test_inline_styles() {
    #[derive(ToMarkdown)]
    struct Styled {
        #[md(code)]
        code_field: String,
        #[md(bold)]
        bold_field: String,
        #[md(italic)]
        italic_field: String,
        #[md(strikethrough)]
        strike_field: String,
        #[md(highlight)]
        highlight_field: String,
        #[md(subscript)]
        sub_field: String,
        #[md(superscript)]
        sup_field: String,
        #[md(bold, italic, code)]
        combo: String,
    }

    let s = Styled {
        code_field: "var".to_string(),
        bold_field: "strong".to_string(),
        italic_field: "em".to_string(),
        strike_field: "deleted".to_string(),
        highlight_field: "mark".to_string(),
        sub_field: "2".to_string(),
        sup_field: "3".to_string(),
        combo: "vital".to_string(),
    };

    let expected = r#"code_field: `var`
bold_field: **strong**
italic_field: *em*
strike_field: ~~deleted~~
highlight_field: ==mark==
sub_field: ~2~
sup_field: ^3^
combo: ***`vital`***
"#;
    assert_eq!(s.to_markdown(), expected);
}

#[test]
fn test_headings_and_prefixes() {
    #[derive(ToMarkdown)]
    struct Headings {
        #[md(heading = 1, plain)]
        h1: String,
        #[md(heading = 2)]
        h2: String,
        #[md(heading = 3, prefix = "Custom: ")]
        h3: String,
    }

    let h = Headings {
        h1: "Main Title".to_string(),
        h2: "Sub Title".to_string(),
        h3: "Section".to_string(),
    };

    let expected = r#"# Main Title
## h2: Sub Title
### Custom: Section
"#;
    assert_eq!(h.to_markdown(), expected);
}

#[test]
fn test_blockquotes() {
    #[derive(ToMarkdown)]
    struct Quotes {
        #[md(quote)]
        single: String,
        #[md(quote)]
        multi: String,
        #[md(plain, quote)]
        plain_quote: String,
    }

    let q = Quotes {
        single: "Single line quote".to_string(),
        multi: "First line\nSecond line\nThird line".to_string(),
        plain_quote: "Direct quote line 1\nDirect quote line 2".to_string(),
    };

    let expected = r#"single:
> Single line quote
multi:
> First line
> Second line
> Third line
> Direct quote line 1
> Direct quote line 2
"#;
    assert_eq!(q.to_markdown(), expected);
}

#[test]
fn test_lists() {
    #[derive(ToMarkdown)]
    struct Lists {
        #[md(heading = 2, list = "unordered")]
        bullets: Vec<String>,
        #[md(heading = 2, list = "ordered")]
        numbered: Vec<String>,
        #[md(heading = 2, list = "definition")]
        definitions: Vec<String>,
        #[md(prefix = "Empty:\n", list = "unordered")]
        empty: Vec<String>,
    }

    let l = Lists {
        bullets: vec!["alpha".to_string(), "beta".to_string()],
        numbered: vec!["first".to_string(), "second".to_string()],
        definitions: vec!["term 1".to_string(), "term 2".to_string()],
        empty: vec![],
    };

    let expected = r#"## bullets:
- alpha
- beta
## numbered:
1. first
2. second
## definitions:
: term 1
: term 2
"#;
    assert_eq!(l.to_markdown(), expected);
}

#[test]
fn test_vec_without_list_attribute() {
    #[derive(ToMarkdown)]
    struct UnformattedLists {
        #[md]
        tags: Vec<String>,
        #[md(plain)]
        numbers: Vec<i32>,
    }

    let u = UnformattedLists {
        tags: vec!["rust".to_string(), "proc-macro".to_string()],
        numbers: vec![1, 2, 3],
    };

    let expected = r#"tags: rust, proc-macro
1, 2, 3
"#;
    assert_eq!(u.to_markdown(), expected);
}

#[test]
fn test_nested_structures() {
    #[derive(ToMarkdown)]
    struct Item {
        #[md(prefix = "  - ", bold)]
        name: String,
    }

    #[derive(ToMarkdown)]
    struct Document {
        #[md(heading = 1, plain)]
        header: String,
        #[md(plain)]
        items: Vec<Item>,
    }

    let doc = Document {
        header: "Section".to_string(),
        items: vec![
            Item {
                name: "Alpha".to_string(),
            },
            Item {
                name: "Beta".to_string(),
            },
        ],
    };

    let expected = r#"# Section
  - **Alpha**
  - **Beta**

"#;
    assert_eq!(doc.to_markdown(), expected);
}

#[test]
fn test_generics() {
    #[derive(ToMarkdown)]
    struct Wrapper<T: ToMarkdown> {
        #[md(plain)]
        value: T,
    }

    let w = Wrapper {
        value: "wrapped text".to_string(),
    };
    assert_eq!(w.to_markdown(), "wrapped text\n");
}

#[test]
fn test_canonical_readme_example() {
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

        #[allow(dead_code)]
        author_id: u64,
    }

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

    let expected = r#"# **Crash on startup**
### description:
> Application panics when config is missing.
### steps:
1. Clone repository
2. Run cargo run --bin app
### tags:
- bug
- backend
"#;

    assert_eq!(issue.to_markdown(), expected);
}
