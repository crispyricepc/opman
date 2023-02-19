mod aur;
mod pacman;

use std::{
    collections::HashMap,
    fmt::{Debug, Formatter},
};

use anyhow::Result;
pub use aur::Aur;

use crate::{package::Dependency, Package};

pub trait Database {
    /// Get the name of the database
    fn db_name(&self) -> String;
    /// Get a package by its name
    fn get_package(&self, name: &String) -> Result<Package>;
    /// Get all the packages in the database
    fn all_packages(&self) -> Vec<Package>;
    /// Search for packages by queries
    fn search(&self, queries: Vec<String>) -> Result<Vec<Package>>;
    /// Get the dependencies of packages
    fn dependencies(&self, pkgs: &Vec<Package>) -> Vec<Dependency>;
    /// Tries to resolve a dependency to a package
    fn resolve_dependency(&self, dep: &Dependency) -> Result<Package> {
        self.get_package(&dep.name)
    }
    /// Get a list of packages by their names
    fn get_packages(&self, names: &Vec<String>) -> HashMap<String, Result<Package>> {
        let mut ret = HashMap::new();
        for name in names {
            ret.insert(name.clone(), self.get_package(name));
        }

        ret
    }
}

impl Debug for dyn Database {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.db_name())
    }
}
