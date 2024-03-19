pub mod decl;
pub mod expr;
pub mod module;
pub mod stmt;
pub mod util;

use std::{error::Error, fs, io, path::Path};

use swc_common::{
    errors::{ColorConfig, Handler},
    sync::Lrc,
    SourceMap,
};
use swc_ecma_ast::Module;
use swc_ecma_parser::{lexer::Lexer, Capturing, Parser, StringInput, Syntax, TsConfig};
use syn::File;

use crate::module::transpile_module;

pub fn parse_rust_file(input_file: &Path) -> Result<File, Box<dyn Error>> {
    let content = fs::read_to_string(input_file)?;
    syn::parse_file(&content).map_err(Box::from)
}

pub fn parse_typescript_file(input_file: &Path) -> Result<Module, io::Error> {
    let cm: Lrc<SourceMap> = Default::default();
    let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));

    let fm = cm.load_file(input_file)?;

    let lexer = Lexer::new(
        Syntax::Typescript(TsConfig {
            decorators: false,
            disallow_ambiguous_jsx_like: false,
            dts: false,
            no_early_errors: false,
            tsx: true,
        }),
        Default::default(),
        StringInput::from(&*fm),
        None,
    );

    let capturing = Capturing::new(lexer);
    let mut parser = Parser::new_from(capturing);

    for e in parser.take_errors() {
        e.into_diagnostic(&handler).emit();
    }

    let module = parser
        .parse_module()
        .map_err(|e| e.into_diagnostic(&handler).emit())
        .expect("Failed to parse module.");

    Ok(module)
}

fn write_rust_file(file: File, output_file: &Path) -> Result<(), io::Error> {
    fs::write(output_file, prettyplease::unparse(&file))
}

pub fn transpile_file(input_file: &Path, output_file: &Path) -> Result<(), io::Error> {
    let module = parse_typescript_file(input_file)?;

    let file = transpile_module(module);

    write_rust_file(file, output_file)?;

    Ok(())
}
