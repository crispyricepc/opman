use std::hash::Hash;

use alpm::Dep;

use crate::Package;

pub struct AlpmPackage {
    inner: alpm::Package<'static>,
}

impl Package for AlpmPackage {
    fn name(&self) -> String {
        todo!()
    }

    fn version(&self) -> String {
        todo!()
    }

    fn desc(&self) -> String {
        todo!()
    }

    fn arch(&self) -> String {
        todo!()
    }

    fn url(&self) -> String {
        todo!()
    }

    fn licenses(&self) -> Vec<String> {
        todo!()
    }

    fn groups(&self) -> Vec<String> {
        todo!()
    }

    fn provides(&self) -> Vec<String> {
        todo!()
    }

    fn depends(&self) -> Vec<Dep> {
        todo!()
    }

    fn depends_optional(&self) -> Vec<Dep> {
        todo!()
    }

    fn required_by(&self) -> Vec<String> {
        todo!()
    }

    fn required_by_optional(&self) -> Vec<String> {
        todo!()
    }

    fn conflicts(&self) -> Vec<String> {
        todo!()
    }

    fn replaces(&self) -> Vec<String> {
        todo!()
    }

    fn installed_size(&self) -> usize {
        todo!()
    }

    fn packager(&self) -> String {
        todo!()
    }

    fn build_date(&self) -> String {
        todo!()
    }

    fn install_date(&self) -> String {
        todo!()
    }

    fn install_reason(&self) -> String {
        todo!()
    }

    fn validation(&self) -> String {
        todo!()
    }
}

impl From<alpm::Package<'static>> for AlpmPackage {
    fn from(value: alpm::Package<'static>) -> Self {
        Self { inner: value }
    }
}

impl PartialEq for AlpmPackage {
    fn eq(&self, other: &Self) -> bool {
        self.inner.name() == other.inner.name()
    }
}

impl Eq for AlpmPackage {}

impl Hash for AlpmPackage {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.inner.name().hash(state);
    }
}
