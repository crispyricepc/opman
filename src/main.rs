mod display;
mod package_ops;

use alpm::{Alpm, SigLevel};
use clap::{Arg, ArgMatches, Command};
use package_ops::{dependencies, summary};

fn init_handle() -> Alpm {
    let handle = Alpm::new("/", "/var/lib/pacman").unwrap();
    handle
        .register_syncdb("core", SigLevel::USE_DEFAULT)
        .unwrap();
    handle
        .register_syncdb("extra", SigLevel::USE_DEFAULT)
        .unwrap();
    handle
        .register_syncdb("community", SigLevel::USE_DEFAULT)
        .unwrap();
    handle
}

fn build_command() -> ArgMatches {
    Command::new("opman")
        .version("0.0.1")
        .author("Ben Mitchell")
        .about("Opinionated Archlinux Package Manager")
        .subcommand(
            Command::new("summary")
                .about("Summarize the given packages")
                .arg(Arg::new("packages").required(false).multiple_values(true)),
        )
        .subcommand(
            Command::new("dependencies")
                .about("Get the given packages' dependencies")
                .alias("deps")
                .arg(Arg::new("packages").required(true).multiple_values(true)),
        )
        .subcommand(
            Command::new("install")
                .about("Install a package")
                .arg(Arg::new("packages").required(true).multiple_values(true)),
        )
        .get_matches()
}

fn main() {
    let handle = init_handle();

    match build_command().subcommand() {
        Some(("summary", summary_matches)) => {
            let pkgs: Vec<&str> = summary_matches
                .values_of("packages")
                .unwrap_or_default()
                .collect();
            summary(&handle, &pkgs);
        }
        Some(("dependencies", dependencies_matches)) => {
            let pkgs: Vec<&str> = dependencies_matches
                .values_of("packages")
                .unwrap_or_default()
                .collect();
            dependencies(&handle, &pkgs);
        }
        Some(("install", install_matches)) => {
            let _pkgs: Vec<&str> = install_matches
                .values_of("packages")
                .unwrap_or_default()
                .collect();
        }
        _ => {
            println!("No subcommand was used");
        }
    }
}
