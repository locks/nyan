use structopt::StructOpt;

pub(crate) fn detect() -> bool {
    std::path::Path::new("pnpm-lock.yaml").exists()
}

#[derive(StructOpt, Debug)]
#[structopt(name = "nyan")]
enum Opt {
    Run { runnable: String },
}

pub(crate) fn run() {
    let mut args = std::env::args();
    let mut command = std::process::Command::new("pnpm");

    if args.len() <= 1 {
        command
            .arg("install")
            .spawn()
            .expect("Could not find npm binary")
            .wait()
            .ok();
    }

    let first_arg = args.nth(1).unwrap_or("".to_string());
    if first_arg == "i" || first_arg == "install" {
        command
            .arg("install")
            .spawn()
            .expect("Could not find npm binary")
            .wait()
            .ok();
    }

    if first_arg == "run" {
        let matches = Opt::from_args();
        println!("{:?}", matches);
        command
            .args(args)
            .spawn()
            .expect("npm run failed.")
            .wait()
            .ok();
    }
}
