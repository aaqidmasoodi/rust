use ra_db::{CrateId, Cancelable};

use crate::{Name, db::HirDatabase, DefId, Path, PerNs, module::{ModuleSource, ModuleScope}};

/// hir::Crate describes a single crate. It's the main inteface with which
/// crate's dependencies interact. Mostly, it should be just a proxy for the
/// root module.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Crate {
    pub(crate) crate_id: CrateId,
}

#[derive(Debug)]
pub struct CrateDependency {
    pub krate: Crate,
    pub name: Name,
}

impl Crate {
    pub fn crate_id(&self) -> CrateId {
        self.crate_id
    }
    pub fn dependencies(&self, db: &impl HirDatabase) -> Cancelable<Vec<CrateDependency>> {
        Ok(self.dependencies_impl(db))
    }
    pub fn root_module(&self, db: &impl HirDatabase) -> Cancelable<Option<Module>> {
        self.root_module_impl(db)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Module {
    pub(crate) def_id: DefId,
}

impl Module {
    // FIXME: what is a module source exactly? It should contain two nodes
    pub fn source(&self, db: &impl HirDatabase) -> Cancelable<ModuleSource> {
        Ok(self.source_impl(db))
    }

    /// Returns the crate this module is part of.
    pub fn krate(&self, db: &impl HirDatabase) -> Cancelable<Option<Crate>> {
        self.krate_impl(db)
    }

    pub fn crate_root(&self, db: &impl HirDatabase) -> Cancelable<Module> {
        self.crate_root_impl(db)
    }

    /// Finds a child module with the specified name.
    pub fn child(&self, db: &impl HirDatabase, name: &Name) -> Cancelable<Option<Module>> {
        self.child_impl(db, name)
    }
    /// Finds a parent module.
    pub fn parent(&self, db: &impl HirDatabase) -> Cancelable<Option<Module>> {
        self.parent_impl(db)
    }
    /// Returns a `ModuleScope`: a set of items, visible in this module.
    pub fn scope(&self, db: &impl HirDatabase) -> Cancelable<ModuleScope> {
        self.scope_impl(db)
    }
    pub fn resolve_path(&self, db: &impl HirDatabase, path: &Path) -> Cancelable<PerNs<DefId>> {
        self.resolve_path_impl(db, path)
    }
}
