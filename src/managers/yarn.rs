pub(crate) fn detect() -> bool {
    std::path::Path::new("yarn.lock").exists()
}

pub(crate) fn run() {
    let mut args = std::env::args();
    let mut command = std::process::Command::new("yarn");

    if args.len() <= 1 {
        command
            .spawn()
            .expect("Could not find yarn binary.")
            .wait()
            .ok();
    }

    let first_arg = args.nth(1).unwrap_or("".to_string());
    if first_arg == "run" {
        command
            .args(args)
            .spawn()
            .expect("Could not run command.")
            .wait()
            .ok();
    }
}
