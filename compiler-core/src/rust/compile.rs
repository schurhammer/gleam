use std::sync::Arc;

use itertools::Itertools;
use smol_str::SmolStr;

use crate::ast::{self as gleam};
use crate::type_::{Type, TypeVar};

pub fn compile(m: &gleam::TypedModule) -> String {
    m.statements
        .iter()
        .map(compile_statement)
        .collect_vec()
        .join("\n\n")
}

fn compile_statement(s: &gleam::TypedStatement) -> String {
    match s {
        gleam::Statement::Fn {
            location: _,
            end_position: _,
            name,
            arguments,
            body,
            public,
            return_annotation: _,
            return_type,
            doc,
        } => {
            let doc = compile_doc(doc);
            let public = compile_public(public);
            let type_args = compile_type_args(arguments);
            let arguments = compile_arguments(arguments);
            let return_type = compile_type(return_type);
            let body = compile_expression(body);
            format!(
                "{doc}{public}fn {name}{type_args}({arguments}) -> {return_type} {{\n{body}\n}}"
            )
        }
        gleam::Statement::TypeAlias {
            location,
            alias,
            parameters,
            type_ast,
            type_,
            public,
            doc,
        } => todo!(),
        gleam::Statement::CustomType {
            location,
            name,
            parameters,
            public,
            constructors,
            doc,
            opaque,
            typed_parameters,
        } => todo!(),
        gleam::Statement::ExternalFn {
            location,
            public,
            arguments,
            name,
            return_,
            return_type,
            module,
            fun,
            doc,
        } => todo!(),
        gleam::Statement::ExternalType {
            location,
            public,
            name,
            arguments,
            doc,
        } => todo!(),
        gleam::Statement::Import {
            location,
            module,
            as_name,
            unqualified,
            package,
        } => todo!(),
        gleam::Statement::ModuleConstant {
            doc,
            location,
            public,
            name,
            annotation,
            value,
            type_,
        } => todo!(),
    }
}

fn compile_public(c: &bool) -> String {
    match c {
        true => String::from("pub "),
        false => String::from(""),
    }
}

fn compile_doc(d: &Option<SmolStr>) -> String {
    match d {
        Some(d) => format!("// {d}\n"),
        None => String::from(""),
    }
}

fn compile_arguments(a: &Vec<gleam::Arg<Arc<Type>>>) -> String {
    a.iter()
        .map(compile_argument)
        .collect::<Vec<_>>()
        .join(", ")
}

fn compile_argument(a: &gleam::Arg<Arc<Type>>) -> String {
    let name = a.get_variable_name().unwrap_or(&SmolStr::from("_")).clone();
    let typ = compile_type(&a.type_);
    format!("{name}: {typ}")
}

fn compile_type(t: &Arc<Type>) -> String {
    match &**t {
        Type::App {
            public: _,
            module: _,
            name,
            args,
        } => {
            // TODO wrap with Rc<> if needed
            if args.len() == 0 {
                name.to_string()
            } else {
                let args = args.iter().map(compile_type).collect::<Vec<_>>().join(", ");
                format!("{name}<{args}>")
            }
        }
        Type::Var { type_ } => match &*type_.borrow() {
            TypeVar::Link { type_ } => compile_type(type_),
            TypeVar::Generic { id } => format!("T{id}"),
            TypeVar::Unbound { id } => format!("T{id}"),
        },
        Type::Fn { args, retrn } => todo!(),
        Type::Tuple { elems } => todo!(),
    }
}

fn compile_expression(b: &gleam::TypedExpr) -> String {
    match b {
        gleam::TypedExpr::Int {
            location,
            typ,
            value,
        } => value.to_string(),
        gleam::TypedExpr::Float {
            location,
            typ,
            value,
        } => todo!(),
        gleam::TypedExpr::String {
            location,
            typ,
            value,
        } => todo!(),
        gleam::TypedExpr::Sequence {
            location,
            expressions,
        } => {
            let body = expressions
                .iter()
                .map(compile_expression)
                .collect_vec()
                .join(";\n");
            return format!("{body}");
        }
        gleam::TypedExpr::Pipeline {
            location,
            expressions,
        } => todo!(),
        gleam::TypedExpr::Var {
            location,
            constructor,
            name,
        } => {
            // TODO insert clones
            format!("{name}")
        }
        gleam::TypedExpr::Fn {
            location,
            typ,
            is_capture,
            args,
            body,
            return_annotation,
        } => todo!(),
        gleam::TypedExpr::List {
            location,
            typ,
            elements,
            tail,
        } => todo!(),
        gleam::TypedExpr::Call {
            location,
            typ,
            fun,
            args,
        } => {
            let fun = compile_expression(fun);
            let args = args
                .iter()
                .map(compile_call_arg)
                .collect::<Vec<_>>()
                .join(", ");
            format!("{fun}({args})")
        }
        gleam::TypedExpr::BinOp {
            location,
            typ,
            name,
            left,
            right,
        } => {
            let name = compile_binop(name);
            let left = compile_expression(left);
            let right = compile_expression(right);
            format!("({left} {name} {right})")
        }
        gleam::TypedExpr::Assignment {
            location,
            typ,
            value,
            pattern,
            kind,
        } => todo!(),
        gleam::TypedExpr::Try {
            location,
            typ,
            value,
            then,
            pattern,
        } => todo!(),
        gleam::TypedExpr::Case {
            location,
            typ,
            subjects,
            clauses,
        } => todo!(),
        gleam::TypedExpr::RecordAccess {
            location,
            typ,
            label,
            index,
            record,
        } => todo!(),
        gleam::TypedExpr::ModuleSelect {
            location,
            typ,
            label,
            module_name,
            module_alias,
            constructor,
        } => todo!(),
        gleam::TypedExpr::Tuple {
            location,
            typ,
            elems,
        } => todo!(),
        gleam::TypedExpr::TupleIndex {
            location,
            typ,
            index,
            tuple,
        } => todo!(),
        gleam::TypedExpr::Todo {
            location,
            label,
            typ,
        } => todo!(),
        gleam::TypedExpr::BitString {
            location,
            typ,
            segments,
        } => todo!(),
        gleam::TypedExpr::RecordUpdate {
            location,
            typ,
            spread,
            args,
        } => todo!(),
        gleam::TypedExpr::Negate { location, value } => todo!(),
    }
}

fn compile_call_arg(a: &gleam::CallArg<gleam::TypedExpr>) -> String {
    // TODO consider label, out of order?
    compile_expression(&a.value)
}

fn compile_binop(op: &gleam::BinOp) -> String {
    // TODO some ops are not the same
    String::from(op.name())
}

fn compile_type_args(a: &Vec<gleam::Arg<Arc<Type>>>) -> String {
    let a = a
        .iter()
        .flat_map(|x| find_generics(&x.type_))
        .map(|x| format!("T{x}"))
        .collect_vec();
    if a.len() == 0 {
        String::from("")
    } else {
        format!("<{}>", a.join(", "))
    }
}

fn find_generics(t: &Arc<Type>) -> Vec<u64> {
    match &**t {
        Type::App { args, .. } => args.iter().flat_map(find_generics).collect_vec(),
        Type::Var { type_ } => match &*type_.borrow() {
            TypeVar::Unbound { id } => vec![*id],
            TypeVar::Link { type_ } => find_generics(type_),
            TypeVar::Generic { id } => vec![*id],
        },
        Type::Fn { args, retrn } => todo!(),
        Type::Tuple { elems } => todo!(),
    }
}
