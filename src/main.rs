
use std::path::Path;
use std::process::Command;
use std::io::{self, Write};

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "nyan")]
enum Opt {
    Run {
        runnable: String
    }
}

fn run_npm() {
    let mut args = std::env::args();
    let mut command = std::process::Command::new("npm");
    
    println!("npm lockfile detected.");

    println!("LEN {:?}", args.len());

    if args.len() <= 1 {
        command.arg("install").spawn().expect("Could not find npm binary").wait().ok();
    }
    
    let first_arg = args.nth(1).unwrap_or("".to_string());
    if first_arg == "i" || first_arg == "install" {
        command.arg("install").spawn().expect("Could not find npm binary").wait().ok();
    }

    if first_arg == "run" {
        let matches = Opt::from_args();
        println!("{:?}", matches);
        command.args(args).spawn().expect("npm run failed.").wait().ok();
    }
}

fn run_yarn() {
    let mut args = std::env::args();
    let mut command = std::process::Command::new("yarn");

    println!("yarn lockfile detected.");

    if (args.len() <= 1) {
        command.spawn().expect("Could not find yarn binary.").wait().ok();
    }

    let first_arg = args.nth(1).unwrap_or("".to_string());
    if first_arg == "run" {
        command.args(args).spawn().expect("Could not run command.").wait().ok();
    }
}

fn main() {
    let manifest = Path::new("package.json");

    if !manifest.exists() {
        println!("package.json was not found in this directory.");
        std::process::exit(1)
    }

    let npm_lockfile = Path::new("package-lock.json");
    let yarn_lockfile = Path::new("yarn.lock");

    if npm_lockfile.exists() && yarn_lockfile.exists() {
        println!("Lockfiles for npm and yarn present, could not disambiguate.");
        std::process::exit(1)
    }

    if npm_lockfile.exists() {
        run_npm();
    }
    if yarn_lockfile.exists() {
        run_yarn();
    }
}
