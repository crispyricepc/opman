use alpm::{Alpm, SigLevel};

use crate::{
    database::{Aur, Pacman},
    package::AlpmPackage,
    Database, Package,
};

/// Get an Alpm handle
///
/// Seems wasteful to get a new handle each time this function is called but
/// alpm doesn't implement Sync so we can't use a static variable.
fn handle() -> Alpm {
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

fn get_databases<'h>(handle: &'h Alpm) {
    let raw_databases = handle.syncdbs();
    let dbs: Vec<Box<dyn Database<dyn Package>>> = vec![];
    for db in raw_databases {
        dbs.push(Box::new(Pacman::new(&db)));
    }
    dbs.push(Box::new(Pacman::new(&handle.localdb())));
    dbs.push(Box::new(Aur::new()));
}

pub fn summary(packages: Vec<String>) {
    todo!()
}
pub fn dependencies(packages: Vec<String>) {
    let pkgs = packages
        .into_iter()
        .filter_map(|pkg| sync.get_package(pkg))
        .collect::<Vec<AlpmPackage>>();
    sync.dependencies(&pkgs);
}
pub fn search(keywords: Vec<String>) {
    sync.search(keywords);
}
pub fn install(packages: Vec<String>) {}
