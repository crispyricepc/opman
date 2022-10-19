mod aur;
mod pacman;

use std::collections::HashSet;

use anyhow::Result;
pub use aur::Aur;

use crate::Package;

pub trait Database<'h> {
    /// Get the name of the database
    fn db_name(&self) -> String;
    /// Get a package by its name
    fn get_package(&self, name: &String) -> Result<Package>;
    /// Get all the packages in the database
    fn get_packages(&self) -> Vec<Package>;
    /// Search for packages by queries
    fn search(&self, queries: Vec<String>) -> Result<Vec<Package>>;
    /// Get the dependencies of packages
    fn dependencies(&self, pkgs: &Vec<Package>) -> Vec<String>;
    /// Recursively search for dependencies for given packages.
    ///
    /// This operation tries to evaluate the dependencies of given packages,
    /// which may not be available in the current database.
    ///
    /// Any package names that couldn't have their dependencies resolved are
    /// returned to be processed by the caller.
    fn dependencies_recursive(&self, pkgs: &Vec<Package>) -> (HashSet<Package>, HashSet<String>) {
        let mut deps = HashSet::new();
        let mut unresolved = HashSet::new();

        let deps_strs = self.dependencies(pkgs);
        for dep_str in deps_strs {
            if let Ok(dep) = self.get_package(&dep_str) {
                let (dep_deps, dep_unresolved) = self.dependencies_recursive(&vec![dep]);
                deps.extend(dep_deps);
                unresolved.extend(dep_unresolved);
            }
        }

        (deps, unresolved)
    }
}
