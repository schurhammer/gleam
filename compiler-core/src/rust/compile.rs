use crate::ast;
use crate::rust;

fn compile(m: &ast::TypedModule) {
    let statements = m
        .statements
        .iter()
        .map(compile_statement)
        .collect::<Vec<_>>();
}

fn compile_statement(s: &ast::TypedStatement) {
    match s {
        ast::Statement::Fn {
            name,
            arguments,
            body,
            public,
            return_annotation,
            return_type,
            ..
        } => todo!(),
        ast::Statement::TypeAlias { .. } => todo!(),
        ast::Statement::CustomType {
            name,
            parameters,
            public,
            constructors,
            typed_parameters,
            ..
        } => todo!(),
        ast::Statement::ExternalFn { .. } => todo!(),
        ast::Statement::ExternalType { .. } => todo!(),
        ast::Statement::Import { .. } => todo!(),
        ast::Statement::ModuleConstant { .. } => todo!(),
    }
}
