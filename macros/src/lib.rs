use jwalk::DirEntry as JwalkDirEntry;
use jwalk::WalkDir;
use proc_macro::TokenStream;
use quote::quote;
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, LitStr, Token, parse_macro_input};

struct MacroInput {
    mod_name: Ident,
    path: LitStr,
}

impl Parse for MacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mod_name = input.parse()?;
        let _: Token![,] = input.parse()?;
        let path = input.parse()?;
        Ok(Self { mod_name, path })
    }
}

/// Expands a directory tree into a nested module tree of `pub const &str` paths.
///
/// Each directory becomes a `pub mod`, each file becomes a `pub const` holding
/// the file's path relative to the source directory. The root directory itself
/// becomes the module named by `mod_name`.
///
/// # Syntax
///
/// ```text
/// generate_dir_structure_as_modules!(mod_name, "relative/path/to/dir");
/// ```
///
/// `mod_name` is a Rust identifier (the name of the root module); `path` is a
/// literal string path to the directory. The path is resolved relative to the
/// crate root (i.e. where `cargo build` is invoked) and must exist, otherwise
/// the macro panics.
///
/// # Example
///
/// Given:
///
/// ```text
/// assets/texture_packs/default/
/// ├── core_runes/
/// │   └── core.png
/// └── troops/
///     └── assault/
///         └── assault_troop_lvl1.png
/// ```
///
/// `generate_dir_structure_as_modules!(default_pack, "assets/texture_packs/default")`
/// expands to approximately:
///
/// ```text
/// pub mod default_pack {
///     pub mod core_runes {
///         pub const CORE: &str = "core_runes/core.png";
///     }
///     pub mod troops {
///         pub mod assault {
///             pub const ASSAULT_TROOP_LVL1: &str = "troops/assault/assault_troop_lvl1.png";
///         }
///     }
/// }
/// ```
///
/// # Constant naming rules
///
/// - The file extension is stripped, then `-` and `.` are replaced with `_`,
///   and the remaining name is upper-cased.
/// - Names that would start with a digit are prefixed with `S` so they form a
///   valid Rust identifier (e.g. `2x.png` → `S2X`).
/// - Names that become empty after stripping the extension (e.g. dotfiles)
///   cause a panic.
///
/// # Note
///
/// The directory is read at compile time. Cargo does not automatically track
/// files read inside a proc macro, so a clean build (or touching the crate)
/// is required for added/removed assets to be picked up.
#[proc_macro]
pub fn generate_dir_structure_as_modules(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as MacroInput);
    let path = input.path.value();
    let outer_module_name = input.mod_name.to_string();

    #[cfg(feature = "macro_debug")]
    {
        println!("path: {}", path);
        println!("outer_module_name: {}", outer_module_name);
    }

    let path_buf: PathBuf = path.into();
    if !path_buf.exists() {
        panic!("Path {:?} doesn't exit.", path_buf);
    }
    let map = parse_dir(&path_buf);
    #[cfg(feature = "macro_debug")]
    println!("generated map with {} entries", map.len());
    let stream = generate_tokens_outer(&map, &path_buf, &outer_module_name).unwrap();
    #[cfg(feature = "macro_debug")]
    println!("generated tokens\n{}", stream);
    stream.into()
}

struct DirEntry {
    name: String,
    sub: Option<Vec<PathBuf>>,
}

impl From<JwalkDirEntry<((), ())>> for DirEntry {
    fn from(value: JwalkDirEntry<((), ())>) -> Self {
        Self {
            name: value.file_name().to_string_lossy().to_string(),
            sub: value.file_type.is_dir().then_some(Vec::new()),
        }
    }
}

fn parse_dir(path: &PathBuf) -> HashMap<PathBuf, DirEntry> {
    let mut map: HashMap<PathBuf, DirEntry> = HashMap::new();
    map.insert(
        path.clone(),
        DirEntry {
            name: path.file_name().unwrap().to_string_lossy().to_string(),
            sub: Some(Vec::new()),
        },
    );
    for e in WalkDir::new(path).follow_links(false).sort(true).into_iter().flatten() {
        if let Some(parent) = map.get_mut(e.parent_path()) {
            match &mut parent.sub {
                Some(sub) => {
                    sub.push(e.path());
                },
                None => parent.sub = Some(vec![e.path()]),
            }
        }
        map.insert(e.path(), DirEntry::from(e));
    }
    map
}

/// Formats a file name into a `SCREAMING_SNAKE_CASE` const identifier.
///
/// Strips a leading dot and the file extension, replaces `-` and `.` with
/// `_`, upper-cases, and prefixes `A` if the filename starts with a digit
/// (e.g. `25x.png` → `A25X`). Panics with the file name if the result is empty
/// or not a valid Rust identifier.
fn format_as_const_name(s: &str) -> Ident {
    let s = s.strip_prefix(".").unwrap_or(s);
    let mut string =
        s.rsplit_once(".").map(|both| both.0).unwrap_or(s).replace(['-', '.'], "_").to_uppercase();
    if string.is_empty() {
        panic!("file name {:?} becomes an empty const name", s);
    }
    if string.chars().next().is_some_and(|c| c.is_numeric()) {
        string.insert(0, 'A');
    }
    to_ident(&string).unwrap_or_else(|_| {
        panic!("file name {:?} cannot become a valid const identifier: {:?}", s, string)
    })
}

fn to_ident(s: &str) -> syn::Result<Ident> {
    syn::parse_str(s)
}

fn generate_tokens_outer(
    map: &HashMap<PathBuf, DirEntry>, path: &Path, outer_module_name: &str,
) -> Option<proc_macro2::TokenStream> {
    let entry = map.get(path)?;

    let skip_components = path.components().count();

    Some(if let Some(enemies) = &entry.sub {
        let sub_modules = enemies
            .iter()
            .filter_map(|p| generate_tokens(map, p, skip_components))
            .collect::<Vec<_>>();
        let mod_name = to_ident(outer_module_name).unwrap();
        create_module(mod_name, sub_modules)
    } else {
        panic!("root path {:?} is not a directory", path);
    })
}

fn generate_tokens(
    map: &HashMap<PathBuf, DirEntry>, path: &Path, skip_components: usize,
) -> Option<proc_macro2::TokenStream> {
    let entry = map.get(path)?;
    let name = &entry.name;
    #[cfg(feature = "macro_debug")]
    println!("{}", name);

    Some(if let Some(enemies) = &entry.sub {
        let sub_modules = enemies
            .iter()
            .filter_map(|p| generate_tokens(map, p, skip_components))
            .collect::<Vec<_>>();
        let mod_name = to_ident(name).unwrap();
        create_module(mod_name, sub_modules)
    } else {
        let path_str = path
            .components()
            .skip(skip_components)
            .collect::<PathBuf>()
            .to_string_lossy()
            .to_string();
        let const_name = format_as_const_name(name);
        create_constant(const_name, path_str)
    })
}

fn create_module(
    mod_name: Ident, sub_modules: Vec<proc_macro2::TokenStream>,
) -> proc_macro2::TokenStream {
    quote! {
        pub mod #mod_name {
            #(#sub_modules)*
        }
    }
}

fn create_constant(name: Ident, path_str: String) -> proc_macro2::TokenStream {
    quote! {
        pub const #name: &str = #path_str;
    }
}
