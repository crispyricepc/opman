use std::{fmt::Display, hash::Hash};

use alpm::{PackageReason, PackageValidation};

pub mod alpm_package;

#[derive(Debug)]
pub struct Dependency {
    pub name: String,
    pub version: Option<String>,
}

impl Display for Dependency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)?;
        if let Some(version) = &self.version {
            write!(f, " {}", version)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub desc: Option<String>,
    pub arch: Option<String>,
    pub url: Option<String>,
    pub licenses: Vec<String>,
    pub groups: Vec<String>,
    pub provides: Vec<Dependency>,
    pub depends: Vec<Dependency>,
    pub depends_optional: Vec<Dependency>,
    pub required_by: Vec<String>,
    pub required_by_optional: Vec<String>,
    pub conflicts: Vec<Dependency>,
    pub replaces: Vec<Dependency>,
    pub installed_size: usize,
    pub packager: Option<String>,
    pub build_date: usize,
    pub install_date: Option<usize>,
    pub install_reason: PackageReason,
    pub validation: PackageValidation,
}

impl PartialEq for Package {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.version == other.version
    }
}

impl Eq for Package {}

impl Hash for Package {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.version.hash(state);
    }
}
