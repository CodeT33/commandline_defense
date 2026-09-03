use jwalk::WalkDir;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use std::collections::HashMap;
use std::path::PathBuf;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, LitStr, Token, parse_macro_input};

struct MacroInput {
    enum_name: Ident,
    path: LitStr,
}

impl Parse for MacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let enum_name = input.parse()?;
        let _: Token![,] = input.parse()?;
        let path = input.parse()?;
        Ok(Self { enum_name, path })
    }
}

/// Generates an enum from a directory tree: one variant per file, plus a
/// `get_path` method that returns the file's path.
///
/// Variant names are the file path relative to the root, with each
/// `/`-separated segment PascalCased and joined by `_` (e.g.
/// `towers/gatling_tower/0_0_0.png` → `Towers_GatlingTower_000`). A leading
/// digit gets an `A` prefix; duplicate names panic.
///
/// ```text
/// generate_dir_structure_as_enum!(EnumName, "relative/path/to/dir");
/// ```
///
/// `path` is relative to the crate root and must exist. The absolute-path
/// variant stores absolute paths and emits `get_abs_path` instead.
#[proc_macro]
pub fn dir_structure_as_enum(input: TokenStream) -> TokenStream {
    implementation(input, false)
}

#[proc_macro]
pub fn dir_structure_as_enum_absolute_paths(input: TokenStream) -> TokenStream {
    implementation(input, true)
}

fn implementation(input: TokenStream, use_absolute_paths: bool) -> TokenStream {
    let input = parse_macro_input!(input as MacroInput);
    let path = input.path.value();
    let enum_name = input.enum_name;

    #[cfg(feature = "macro_debug")]
    {
        println!("path: {}", path);
        println!("enum_name: {}", enum_name);
    }

    let path_buf = PathBuf::from(path);
    if !path_buf.exists() {
        panic!("Path {:?} doesn't exist.", path_buf);
    }

    let mut entries: Vec<(Ident, String)> = Vec::new();
    let mut seen: HashMap<String, String> = HashMap::new();
    for entry in WalkDir::new(&path_buf).follow_links(false).sort(true).into_iter().flatten() {
        if !entry.file_type.is_file() {
            continue;
        }
        let full = entry.path();
        let rel = full.strip_prefix(&path_buf).expect("walker path is under the root directory");
        let name = format_as_variant_name(&rel.to_string_lossy());
        let stored_path = if use_absolute_paths { &full } else { rel };
        let stored = stored_path.to_string_lossy().into_owned();
        if let Some(prev) = seen.insert(name.to_string(), stored.clone()) {
            panic!("files {:?} and {:?} map to the same variant name `{}`", prev, stored, name);
        }
        entries.push((name, stored));
    }
    entries.sort_by(|a, b| a.0.cmp(&b.0));
    #[cfg(feature = "macro_debug")]
    println!("found {} files", entries.len());

    let method_ident =
        if use_absolute_paths { format_ident!("get_abs_path") } else { format_ident!("get_path") };
    let variants = entries.iter().map(|(name, _)| name);
    let arms = entries.iter().map(|(name, path_str)| {
        quote! { Self::#name => #path_str }
    });

    let stream = quote! {
        #[allow(non_camel_case_types)]
        pub enum #enum_name {
            #(#variants),*
        }

        impl #enum_name {
            pub const fn #method_ident(&self) -> &'static str {
                match self {
                    #(#arms),*
                }
            }
        }
    };
    #[cfg(feature = "macro_debug")]
    println!("generated tokens\n{}", stream);
    stream.into()
}

/// Converts a relative asset path into its variant name (PascalCase segments joined by `_`).
fn format_as_variant_name(path: &str) -> Ident {
    let path = path.strip_prefix(".").unwrap_or(path);
    let stem = path.rsplit_once('.').map(|(stem, _)| stem).unwrap_or(path);
    let mut name = stem
        .split('/')
        .map(pascal_case_segment)
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("_");
    if name.is_empty() {
        panic!("path {:?} cannot be converted to a variant name", path);
    }
    if name.starts_with(|c: char| c.is_numeric()) {
        name.insert(0, 'A');
    }
    to_ident(&name).unwrap_or_else(|_| {
        panic!("path {:?} cannot become a valid variant identifier: {:?}", path, name)
    })
}

fn pascal_case_segment(segment: &str) -> String {
    let mut name = String::new();
    for word in segment.split(['.', '-', '_']) {
        if word.is_empty() {
            continue;
        }
        let mut chars = word.chars();
        if let Some(first) = chars.next() {
            name.extend(first.to_uppercase());
            name.push_str(chars.as_str());
        }
    }
    name
}

fn to_ident(s: &str) -> syn::Result<Ident> {
    syn::parse_str(s)
}
