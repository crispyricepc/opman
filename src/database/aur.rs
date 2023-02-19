use anyhow::Result;

use crate::{package::Dependency, Package};

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
        Err(anyhow::anyhow!("AUR database not implemented"))
    }

    fn all_packages(&self) -> Vec<Package> {
        todo!();
    }

    fn search(&self, queries: Vec<String>) -> Result<Vec<Package>> {
        Err(anyhow::anyhow!("AUR database not implemented"))
    }

    fn dependencies(&self, pkgs: &Vec<Package>) -> Vec<Dependency> {
        todo!();
    }
}
