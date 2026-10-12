use darling::{FromDeriveInput, FromField, FromMeta};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// Markdown list formatting variants.
#[derive(FromMeta, Debug, PartialEq)]
enum ListKind {
    Unordered,
    Ordered,
    Definition,
}

/// Field-level attributes parsed from `#[md(...)]`.
#[derive(FromField)]
#[darling(attributes(md))]
struct FieldArgs {
    ident: Option<syn::Ident>,

    #[darling(default)]
    plain: bool,

    #[darling(default)]
    bold: bool,

    #[darling(default)]
    italic: bool,

    #[darling(default)]
    strikethrough: bool,

    #[darling(default)]
    highlight: bool,

    #[darling(default)]
    code: bool,

    #[darling(default)]
    quote: bool,

    #[darling(default)]
    subscript: bool,

    #[darling(default)]
    superscript: bool,

    #[darling(default)]
    heading: Option<u8>,

    #[darling(default)]
    prefix: Option<String>,

    #[darling(default)]
    list: Option<ListKind>,
}

/// Struct-level container for darling parsing.
#[derive(FromDeriveInput)]
#[darling(attributes(md))]
struct StructArgs {
    ident: syn::Ident,
    data: darling::ast::Data<(), FieldArgs>,
}

/// Proc-macro derive entrypoint for `ToMarkdown`.
#[proc_macro_derive(ToMarkdown, attributes(md))]
pub fn derive_to_markdown(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let args = match StructArgs::from_derive_input(&input) {
        Ok(v) => v,
        Err(e) => return TokenStream::from(e.write_errors()),
    };

    let name = &args.ident;

    // Ensure macro is applied to named structs only.
    let struct_data = match args.data.take_struct() {
        Some(data) if data.is_struct() => data,
        _ => {
            return TokenStream::from(
                syn::Error::new_spanned(
                    &input.ident,
                    "ToMarkdown can only be derived for structs with named fields",
                )
                .to_compile_error(),
            );
        }
    };

    let fields = &struct_data.fields;

    // Extract raw syn fields for attribute presence checking (whitelist filtering).
    let syn_fields = match &input.data {
        syn::Data::Struct(s) => &s.fields,
        _ => unreachable!(),
    };

    let field_statements = fields
        .iter()
        .zip(syn_fields.iter())
        .map(|(field, syn_field)| {
            let field_name = match &field.ident {
                Some(id) => id,
                None => return quote! {},
            };

            // Whitelist check: skip fields without #[md] attribute.
            let has_md = syn_field
                .attrs
                .iter()
                .any(|attr| attr.path().is_ident("md"));
            if !has_md {
                return quote! {};
            }

            // Determine field prefix.
            let prefix_str = if let Some(p) = &field.prefix {
                p.clone()
            } else if field.plain {
                String::new()
            } else {
                format!("{}: ", field_name)
            };

            // Determine heading hashes (#, ##, ###).
            let heading_prefix = match field.heading {
                Some(level) => {
                    if !(1..=6).contains(&level) {
                        return quote! {
                            compile_error!("Heading level must be between 1 and 6");
                        };
                    };
                    let hashes = "#".repeat(level as usize);
                    format!("{} ", hashes)
                }
                None => String::new(),
            };

            // Handle list iteration if #[md(list = "...")] is specified.
            if let Some(list_kind) = &field.list {
                let mut list_header = format!("{}{}", heading_prefix, prefix_str);
                if !list_header.is_empty() {
                    list_header = format!("{}\n", list_header.trim_end());
                }

                return match list_kind {
                    ListKind::Unordered => quote! {
                        if !self.#field_name.is_empty() {
                            out.push_str(#list_header);
                            for item in self.#field_name.iter() {
                                out.push_str(&format!("- {}\n", item.to_markdown()));
                            }
                        }
                    },
                    ListKind::Ordered => quote! {
                        if !self.#field_name.is_empty() {
                            out.push_str(#list_header);
                            for (i, item) in self.#field_name.iter().enumerate() {
                                out.push_str(&format!("{}. {}\n", i + 1, item.to_markdown()));
                            }
                        }
                    },
                    ListKind::Definition => quote! {
                        if !self.#field_name.is_empty() {
                            out.push_str(#list_header);
                            for item in self.#field_name.iter() {
                                out.push_str(&format!(": {}\n", item.to_markdown()));
                            }
                        }
                    },
                };
            }

            // Layer text formatting specifiers.
            let mut fmt = String::from("{}");

            let styles = [
                (field.code, "`", "`"),
                (field.bold, "**", "**"),
                (field.italic, "*", "*"),
                (field.strikethrough, "~~", "~~"),
                (field.highlight, "==", "=="),
                (field.subscript, "~", "~"),
                (field.superscript, "^", "^"),
                (field.quote, "> ", ""),
            ];

            for (cond, prefix, suffix) in styles {
                if cond {
                    fmt = format!("{}{}{}", prefix, fmt, suffix);
                }
            }

            let mut header = format!("{}{}", heading_prefix, prefix_str);
            if field.quote && !header.is_empty() {
                header = format!("{}\n", header.trim_end());
            }

            let full_fmt = format!("{}{}\n", header, fmt);
            let val_binding = if field.quote {
                quote! {
                    let val = self.#field_name.to_markdown();
                    let val = val.lines().collect::<Vec<_>>().join("\n> ");
                }
            } else {
                quote! {
                    let val = self.#field_name.to_markdown();
                }
            };

            quote! {
                #val_binding
                if !val.is_empty() {
                    out.push_str(&format!(#full_fmt, val))
                }
            }
        });

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics ::struct2md::ToMarkdown for #name #ty_generics #where_clause {
            fn to_markdown(&self) -> String {
                let mut out = String::new();
                #(#field_statements)*
                out
            }
        }
    };

    TokenStream::from(expanded)
}
