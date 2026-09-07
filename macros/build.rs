use std::fs;

fn main() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../assets/.stamp");
    if fs::exists(path).ok().is_none_or(|val| !val) {
        fs::write(path, String::new()).expect("Unable to write stamp file");
    }
}
