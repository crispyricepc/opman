use std::collections::HashSet;

use crate::{package::AurPackage, Package};

use super::Database;

pub struct Aur;

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

    fn dependencies(&self, _pkgs: &Vec<impl Package>) -> HashSet<String> {
        todo!()
    }

    fn dependencies_recursive(
        &self,
        _pkgs: &Vec<impl Package>,
    ) -> (HashSet<&AurPackage>, Vec<String>) {
        todo!()
    }
}
