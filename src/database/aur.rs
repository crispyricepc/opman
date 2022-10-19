use anyhow::Result;

use crate::Package;

use super::Database;

pub struct Aur;

impl Aur {
    pub fn new() -> Self {
        Self
    }
}

impl<'d> Database<'d> for Aur {
    fn db_name(&self) -> String {
        "aur".to_string()
    }
    fn get_package(&self, name: &String) -> Result<Package> {
        todo!()
    }

    fn get_packages(&self) -> Vec<Package> {
        todo!()
    }

    fn search(&self, queries: Vec<String>) -> Result<Vec<Package>> {
        todo!()
    }

    fn dependencies(&self, pkgs: &Vec<Package>) -> Vec<String> {
        todo!()
    }
}
