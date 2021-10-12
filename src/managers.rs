pub(crate) mod npm;
pub(crate) mod pnpm;
pub(crate) mod yarn;

pub(crate) enum Manager {
    Npm,
    Yarn,
    Pnpm,
    Multiple,
}

pub(crate) fn detect() -> Manager {
    let lockfiles = (
        crate::managers::npm::detect(),
        crate::managers::yarn::detect(),
        crate::managers::pnpm::detect(),
    );

    print_lockfiles(lockfiles.0, lockfiles.1, lockfiles.2);
    match lockfiles {
        (true, true, _) | (true, _, true) | (_, true, true) => Manager::Multiple,
        (true, false, false) => Manager::Npm,
        (false, true, false) => Manager::Yarn,
        (false, false, true) => Manager::Pnpm,
        _ => Manager::Npm,
    }
}

fn print_lockfiles(npm: bool, yarn: bool, pnpm: bool) {
    let npm = if npm { "✓" } else { " " };
    let yarn = if yarn { "✓" } else { " " };
    let pnpm = if pnpm { "✓" } else { " " };
    println!("{:1} npm\t{:1} yarn\t{:1} pnpm", npm, yarn, pnpm);
}

pub(crate) fn run(manager: Manager) {
    match manager {
        Manager::Multiple => {
            eprintln!("Multiple lockfiles found. Cannot disambiguate.");
            std::process::exit(1)
        }
        Manager::Npm => {
            crate::managers::npm::run();
        }
        Manager::Yarn => {
            crate::managers::yarn::run();
        }
        Manager::Pnpm => {
            crate::managers::pnpm::run();
        }
    }
}
