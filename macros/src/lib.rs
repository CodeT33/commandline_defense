use jwalk::DirEntry as JwalkDirEntry;
use jwalk::WalkDir;
use proc_macro::TokenStream;
use quote::quote;
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;

#[proc_macro]
pub fn generate_dir_structure_as_modules(dir_path: TokenStream) -> TokenStream {
    let path = dir_path.to_string();
    println!("path: {}", path);
    let path_buf: PathBuf = path.into();
    if !path_buf.exists() {
        panic!("Path {:?} doesn't exit.", path_buf);
    }
    let map = parse_dir(&path_buf);
    println!("generated map with {} entries", map.len());
    let stream = generate_tokens(&map, &path_buf).unwrap();
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

fn format_as_const(s: &str) -> String {
    let mut string = s.rsplit_once(".").unwrap().0.replace(['-', '.'], "_").to_uppercase();
    if string.is_empty() {
        panic!("String is empty")
    };
    if string.chars().next().unwrap().is_numeric() {
        string.insert(0, 'S');
    }
    string
}

fn to_ident(s: &str) -> syn::Result<syn::Ident> {
    syn::parse_str(s)
}

fn generate_tokens(
    map: &HashMap<PathBuf, DirEntry>, path: &Path,
) -> Option<proc_macro2::TokenStream> {
    let entry = map.get(path)?;
    let name = &entry.name;
    println!("{}", name);

    if let Some(enemies) = &entry.sub {
        let sub_modules =
            enemies.iter().filter_map(|p| generate_tokens(map, p)).collect::<Vec<_>>();
        let mod_name = to_ident(name).unwrap();
        Some(quote! {
            pub mod #mod_name {
                #(#sub_modules)*
            }
        })
    } else {
        let path_str = path.to_string_lossy().to_string();
        let const_name = to_ident(&format_as_const(name)).unwrap();
        Some(quote! {
            pub const #const_name: &str = #path_str;
        })
    }
}
