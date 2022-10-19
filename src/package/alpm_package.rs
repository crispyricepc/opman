use crate::Package;

use super::Dependency;

impl<'a> From<alpm::Dep<'a>> for Dependency {
    fn from(f: alpm::Dep<'a>) -> Self {
        Dependency {
            name: f.name().to_string(),
            version: f.version().map(|v| v.to_string()),
        }
    }
}

impl<'a> From<alpm::Package<'a>> for Package {
    fn from(f: alpm::Package) -> Self {
        Package {
            name: f.name().to_owned(),
            version: f.version().to_string(),
            desc: f.desc().map(|s| s.to_owned()),
            arch: f.arch().map(|s| s.to_owned()),
            url: f.url().map(|s| s.to_owned()),
            licenses: f.licenses().into_iter().map(|s| s.to_owned()).collect(),
            groups: f.groups().into_iter().map(|s| s.to_owned()).collect(),
            provides: f.provides().into_iter().map(|d| d.into()).collect(),
            depends: f.depends().into_iter().map(|d| d.into()).collect(),
            depends_optional: f.optdepends().into_iter().map(|d| d.into()).collect(),
            required_by: f.required_by().into_iter().collect(),
            required_by_optional: f.optional_for().into_iter().collect(),
            conflicts: f.conflicts().into_iter().map(|d| d.into()).collect(),
            replaces: f.replaces().into_iter().map(|d| d.into()).collect(),
            installed_size: f.size().try_into().unwrap(),
            packager: f.packager().map(|s| s.to_owned()),
            build_date: f.build_date().try_into().unwrap(),
            install_date: f.install_date().map(|d| d.try_into().unwrap()),
            install_reason: f.reason(),
            validation: f.validation(),
        }
    }
}
