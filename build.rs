// build.rs
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=assets");

    // Place the stamp in a tracked subfolder (e.g., .cargo/ or assets/.stamp)
    let stamp_path = Path::new("assets/.stamp");
    let now =
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();

    let _ = fs::write(stamp_path, now.to_string());
}
