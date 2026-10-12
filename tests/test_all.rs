use struct2md::ToMarkdown;

#[test]
fn test_field_whitelist() {
    #[derive(ToMarkdown)]
    struct Container {
        #[md(plain)]
        a: String,
        #[allow(dead_code)]
        b: String,
    }

    let c = Container {
        a: "alpha".to_string(),
        b: "beta".to_string(),
    };

    assert_eq!(c.to_markdown(), "alpha\n");
}

#[test]
fn test_style_combinations() {
    #[derive(ToMarkdown)]
    struct Element {
        #[md(bold, italic, code)]
        val: String,
    }

    let e = Element {
        val: "v".to_string(),
    };

    assert_eq!(e.to_markdown(), "val: ***`v`***\n");
}

#[test]
fn test_primitive_types() {
    #[derive(ToMarkdown)]
    struct Data {
        #[md(plain)]
        s: String,
        #[md(plain)]
        b: bool,
        #[md(plain)]
        i: i32,
        #[md(plain)]
        f: f64,
        #[md(plain)]
        opt_some: Option<String>,
        #[md(plain)]
        opt_none: Option<String>,
    }

    let d = Data {
        s: "text".to_string(),
        b: true,
        i: 42,
        f: 3.14,
        opt_some: Some("present".to_string()),
        opt_none: None,
    };

    assert_eq!(d.to_markdown(), "text\ntrue\n42\n3.14\npresent\n");
}

#[test]
fn test_list_formatting() {
    #[derive(ToMarkdown)]
    struct Node {
        #[md(prefix = "Items:\n", list = "unordered")]
        items: Vec<String>,

        #[md(prefix = "Empty:\n", list = "ordered")]
        empty: Vec<String>,
    }

    let n = Node {
        items: vec!["first".to_string(), "second".to_string()],
        empty: vec![],
    };

    assert_eq!(n.to_markdown(), "Items:\n- first\n- second\n");
}

#[test]
fn test_nested_structures() {
    #[derive(ToMarkdown)]
    struct Item {
        #[md(prefix = "  - ", bold)]
        name: String,
    }

    #[derive(ToMarkdown)]
    struct Doc {
        #[md(heading = 1, plain)]
        header: String,

        #[md(plain)]
        items: Vec<Item>,
    }

    let doc = Doc {
        header: "Section".to_string(),
        items: vec![
            Item {
                name: "A".to_string(),
            },
            Item {
                name: "B".to_string(),
            },
        ],
    };

    assert_eq!(doc.to_markdown(), "# Section\n  - **A**\n  - **B**\n\n");
}

#[test]
fn test_readme_example() {
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

    let expected = "\
# **Crash on startup**
### description: > Application panics when config is missing.
### steps: \n\
1. Clone repository
2. Run cargo run --bin app
### tags: \n\
- bug
- backend
";

    assert_eq!(issue.to_markdown(), expected);
}

#[test]
fn test_multiline_quote() {
    #[derive(ToMarkdown)]
    struct Note {
        #[md(quote)]
        content: String,
        #[md(plain, quote)]
        plain_content: String,
    }

    let n = Note {
        content: "First line\nSecond line\nThird line".to_string(),
        plain_content: "Alpha\nBeta".to_string(),
    };

    let expected = "\
content: > First line
> Second line
> Third line
> Alpha
> Beta
";

    assert_eq!(n.to_markdown(), expected);
}

#[test]
fn test_vec_without_list_attribute() {
    #[derive(ToMarkdown)]
    struct Post {
        #[md]
        tags: Vec<String>,
        #[md(plain)]
        numbers: Vec<i32>,
    }

    let post = Post {
        tags: vec!["rust".to_string(), "proc-macro".to_string()],
        numbers: vec![1, 2, 3],
    };

    assert_eq!(post.to_markdown(), "tags: rust, proc-macro\n1, 2, 3\n");
}

#[test]
fn test_definition_list() {
    #[derive(ToMarkdown)]
    struct Dict {
        #[md(heading = 2, list = "definition")]
        terms: Vec<String>,
    }

    let d = Dict {
        terms: vec!["definition 1".to_string(), "definition 2".to_string()],
    };

    assert_eq!(
        d.to_markdown(),
        "## terms: \n: definition 1\n: definition 2\n"
    );
}

#[test]
fn test_extended_primitives_and_arrays() {
    #[derive(ToMarkdown)]
    struct Misc<'a> {
        #[md(plain)]
        c: char,
        #[md(plain)]
        big_u: u128,
        #[md(plain)]
        big_i: i128,
        #[md(plain)]
        arr: [i32; 3],
        #[md(plain)]
        borrowed: &'a str,
    }

    let m = Misc {
        c: 'X',
        big_u: 100_000_000_000_000_000_000,
        big_i: -100_000_000_000_000_000_000,
        arr: [10, 20, 30],
        borrowed: "borrowed string",
    };

    let expected =
        "X\n100000000000000000000\n-100000000000000000000\n10, 20, 30\nborrowed string\n";
    assert_eq!(m.to_markdown(), expected);
}

#[test]
fn test_generic_struct() {
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
