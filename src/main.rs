mod managers;

fn main() {
    let manifest = std::path::Path::new("package.json");

    if !manifest.exists() {
        eprintln!("package.json was not found in this directory.");
        std::process::exit(1);
    }

    let manager = crate::managers::detect();
    crate::managers::run(manager);
}
