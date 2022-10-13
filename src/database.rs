mod aur;
mod pacman;

use std::{collections::HashSet, hash::Hash};

use alpm::{Alpm, SigLevel};

pub use aur::Aur;
pub use pacman::Pacman;

use crate::Package;

/// Get an Alpm handle
///
/// Seems wasteful to get a new handle each time this function is called but
/// alpm doesn't implement Sync so we can't use a static variable.
pub fn handle() -> Alpm {
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

pub trait Database<Pkg>
where
    Pkg: Package + Hash + Eq,
{
    /// Get a package by its name
    fn get_package(&self, name: String) -> Option<Pkg>;
    /// Get all the packages in the database
    fn get_packages(&self) -> Vec<Pkg>;
    /// Search for packages by queries
    fn search(&self, queries: Vec<String>) -> Vec<Pkg>;
    /// Get the dependencies of packages
    fn dependencies(&self, pkgs: &Vec<impl Package>) -> HashSet<String>;
    /// Recursively search for dependencies for given packages.
    ///
    /// This operation tries to evaluate the dependencies of given packages,
    /// which may not be available in the current database.
    ///
    /// Any package names that couldn't have their dependencies resolved are
    /// returned to be processed by the caller.
    fn dependencies_recursive(&self, pkgs: &Vec<impl Package>) -> (HashSet<&Pkg>, Vec<String>) {
        let mut deps = HashSet::new();
        let mut unresolved = vec![];

        let deps_strs = self.dependencies(pkgs);
        for dep_str in deps_strs {
            if let Some(dep) = self.get_package(dep_str) {
                let (dep_deps, dep_unresolved) = self.dependencies_recursive(&vec![dep]);
                deps.extend(dep_deps);
                unresolved.extend(dep_unresolved);
            }
        }

        (deps, unresolved)
    }
}
