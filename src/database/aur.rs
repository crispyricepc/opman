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
        Err(ErrorKind::NotYetImplemented.into())
    }

    fn all_packages(&self) -> Vec<Package> {
        todo!();
    }

    fn search_packages(&self, queries: Vec<String>) -> Result<Vec<Package>> {
        Err(ErrorKind::NotYetImplemented.into())
    }

    fn dependencies(&self, pkgs: &Vec<Package>) -> Vec<Dependency> {
        todo!();
    }
}
