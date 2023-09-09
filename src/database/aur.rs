use crate::{
    error::{ErrorKind, Result},
    package::Dependency,
    Package,
};

use super::Database;

pub struct Aur;

impl Aur {
    pub fn new() -> Self {
        Self
    }
}

impl<'d> Database for Aur {
    fn db_name(&self) -> String {
        "aur".to_string()
    }
    fn get_package(&self, name: &String) -> Result<Package> {
        Err(ErrorKind::PackageNotFound.into())
    }

    fn all_packages(&self) -> Result<Vec<Package>> {
        Ok(vec![])
    }

    fn search_packages(&self, queries: Vec<String>) -> Result<Vec<Package>> {
        Ok(vec![])
    }

    fn dependencies(&self, pkgs: &Vec<Package>) -> Result<Vec<Dependency>> {
        Ok(vec![])
    }
}
