use std::collections::HashSet;

use crate::{package::AurPackage, Package};

use super::Database;

pub struct Aur;

impl Aur {
    pub fn new() -> Self {
        Self
    }
}

impl Database<AurPackage> for Aur {
    fn get_package(&self, _name: String) -> Option<AurPackage> {
        todo!()
    }

    fn get_packages(&self) -> Vec<AurPackage> {
        todo!()
    }

    fn search(&self, _queries: Vec<String>) -> Vec<AurPackage> {
        todo!()
    }

    fn dependencies(&self, _pkgs: &Vec<AurPackage>) -> HashSet<String> {
        todo!()
    }
}
