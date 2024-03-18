use swc_ecma_ast::{Module, ModuleItem};
use syn::{File, Item};

use crate::stmt::transpile_stmt;

pub fn transpile_module(module: Module) -> File {
    File {
        shebang: None,
        attrs: vec![],
        items: module
            .body
            .into_iter()
            .flat_map(transpile_module_item)
            .collect(),
    }
}

pub fn transpile_module_item(module_item: ModuleItem) -> Vec<Item> {
    if module_item.is_module_decl() {
        todo!("module item module decl")
    } else if module_item.is_stmt() {
        transpile_stmt(module_item.stmt().expect("ModuleItem is Stmt."))
    } else {
        unreachable!("Unknown module item kind.")
    }
}
