use jwalk::DirEntry as JwalkDirEntry;
use jwalk::WalkDir;
use proc_macro::TokenStream;
use quote::quote;
use std::collections::HashMap;
use std::path::PathBuf;

#[proc_macro]
pub fn my_custom_macro(_input: TokenStream) -> TokenStream {
    let path = _input.to_string();
    let path_buf = path.into();
    let map = parse_dir(&path_buf);
    let input: proc_macro2::TokenStream = _input.into();
    quote! {
        pub mod #input {}
    }
    .into()
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
