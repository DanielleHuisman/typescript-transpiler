use swc_ecma_ast as swc;
use syn::{File, Item, Stmt};

use crate::stmt::transpile_stmt_only;

pub fn transpile_module(module: swc::Module) -> File {
    // TODO: add `use ts_std::*;` to top of file

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

pub fn transpile_module_item(module_item: swc::ModuleItem) -> Vec<Item> {
    if module_item.is_module_decl() {
        todo!("module item module decl")
    } else if module_item.is_stmt() {
        transpile_stmt_to_items(module_item.stmt().expect("ModuleItem is Stmt."))
    } else {
        unreachable!("Unknown module item kind.")
    }
}

pub fn transpile_stmt_to_items(stmt: swc::Stmt) -> Vec<Item> {
    let stmt = transpile_stmt_only(stmt);

    // TODO: wrap others in main func or do that in transpile_module?
    match stmt {
        Stmt::Local(_) => todo!("local"),
        Stmt::Item(item) => vec![item],
        Stmt::Expr(_, _) => todo!("expr"),
        Stmt::Macro(_) => todo!("macro"),
    }
}
