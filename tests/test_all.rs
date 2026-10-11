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
            Item { name: "A".to_string() },
            Item { name: "B".to_string() },
        ],
    };

    assert_eq!(doc.to_markdown(), "# Section\n  - **A**\n  - **B**\n\n");
}
