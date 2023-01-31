use std::fmt;

struct Typ {
    name: String,
    body: Vec<Typ>,
}

struct Field {
    name: String,
    typ: Typ,
}

struct Constructor {
    name: String,
    fields: Vec<Field>,
}

#[derive(Copy, Clone)]
enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

struct PatternField {
    name: String,
    pattern: Pattern,
}
enum Pattern {
    Wildcard,
    Integer {
        value: i64,
    },
    Variable {
        name: String,
    },
    Struct {
        name: String,
        fields: Vec<PatternField>,
    },
}

struct MatchBranch {
    pattern: Pattern,
    body: Vec<Expression>,
}

struct ExpressionField {
    name: String,
    expression: Expression,
}

enum Expression {
    Integer {
        value: i64,
    },
    Var {
        name: String,
    },
    BinOp {
        op: BinOp,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Match {
        on: Box<Expression>,
        branches: Vec<MatchBranch>,
    },
    Call {
        name: String,
        arguments: Vec<Expression>,
    },
    Construct {
        name: String,
        fields: Vec<ExpressionField>,
    },
}

enum Statement {
    Enum {
        name: String,
        typ_parameters: Vec<String>,
        constructors: Vec<Constructor>,
    },
    Fn {
        name: String,
        parameters: Vec<Field>,
        return_typ: Typ,
        body: Vec<Expression>,
    },
}

fn compile_field(f: &Field) -> String {
    let name = f.name.clone();
    let typ = compile_typ(&f.typ);
    format!("{name}: {typ}")
}

fn compile_fields(fs: &Vec<Field>) -> Vec<String> {
    fs.iter().map(compile_field).collect()
}

fn compile_constructor(c: &Constructor) -> String {
    let name = c.name.clone();
    if c.fields.len() == 0 {
        format!("{name},")
    } else {
        let body: String = compile_fields(&c.fields)
            .iter()
            .map(|f| format!("\n\t\t{f},"))
            .collect();
        format!("{name} {{{body}\n\t}},")
    }
}

fn compile_constructors(cs: Vec<Constructor>) -> String {
    cs.iter()
        .map(compile_constructor)
        .map(|c| format!("\n\t{c}"))
        .collect()
}

fn compile_op(op: BinOp) -> String {
    match op {
        BinOp::Add => "+".to_string(),
        BinOp::Sub => "-".to_string(),
        BinOp::Mul => "*".to_string(),
        BinOp::Div => "/".to_string(),
    }
}
fn compile_pattern_field(f: &PatternField) -> String {
    let pattern = compile_pattern(&f.pattern);
    format!("{}: {}", f.name, pattern)
}
fn compile_pattern(p: &Pattern) -> String {
    match p {
        Pattern::Wildcard => "_".to_string(),
        Pattern::Integer { value } => format!("{value}i64"),
        Pattern::Variable { name } => name.clone(),
        Pattern::Struct { name, fields } => {
            let fields = fields
                .iter()
                .map(compile_pattern_field)
                .collect::<Vec<String>>()
                .join(", ");
            format!("{name} {{{fields}}}")
        }
    }
}
fn compile_branch(b: &MatchBranch) -> String {
    let pattern = compile_pattern(&b.pattern);
    let body = compile_expressions(&b.body).join(";\n");
    format!("{pattern} => {{\n{body}\n}}")
}

fn compile_expression_field(f: &ExpressionField) -> String {
    let expr = compile_expression(&f.expression);
    format!("{}: {expr}", f.name)
}

fn compile_expression(ex: &Expression) -> String {
    match ex {
        Expression::BinOp { op, left, right } => {
            let op = compile_op(*op);
            let left = compile_expression(left);
            let right = compile_expression(right);
            format!("({left} {op} {right})")
        }
        Expression::Integer { value } => {
            format!("{value}")
        }
        Expression::Var { name } => {
            // TODO optimise when to call clone, not to call clone, or Rc::clone
            format!("{}.clone()", name)
        }
        Expression::Call { name, arguments } => {
            let arguments = arguments
                .iter()
                .map(compile_expression)
                .collect::<Vec<String>>()
                .join(", ");
            format!("{name}({arguments})")
        }
        Expression::Match { on, branches } => {
            let on = compile_expression(on);
            let branches = branches.iter().map(compile_branch).collect::<String>();
            format!("{on} {{{branches}}}")
        }
        Expression::Construct { name, fields } => {
            let fields = fields
                .iter()
                .map(compile_expression_field)
                .collect::<Vec<String>>()
                .join(",");
            format!("{name} {{{fields}}}")
        }
    }
}

fn compile_expressions(body: &Vec<Expression>) -> Vec<String> {
    body.iter().map(compile_expression).collect::<Vec<_>>()
}

fn compile_typ(t: &Typ) -> String {
    if t.body.len() == 0 {
        format!("{}", t.name)
    } else {
        let body = t
            .body
            .iter()
            .map(compile_typ)
            .collect::<Vec<String>>()
            .join(", ");
        format!("{}<{}>", t.name, body)
    }
}

fn compile(t: Statement) -> String {
    match t {
        Statement::Enum {
            name,
            typ_parameters,
            constructors,
        } => {
            let constructors = compile_constructors(constructors);
            if typ_parameters.len() == 0 {
                format!("enum {name} {{{constructors}\n}}\n")
            } else {
                let typ_parameters = typ_parameters.join(", ");
                format!("enum {name}<{typ_parameters}> {{{constructors}\n}}\n")
            }
        }
        Statement::Fn {
            name,
            parameters,
            return_typ,
            body,
        } => {
            let parameters: String = compile_fields(&parameters).join(", ");
            let body = compile_expressions(&body).join(";\n\t");
            let return_typ = compile_typ(&return_typ);
            format!("fn {name} ({parameters}) -> {return_typ} {{\n\t{body}\n}}\n")
        }
    }
}

pub fn main() {
    let recursive = Statement::Enum {
        name: "List".to_string(),
        typ_parameters: vec!["a".to_string()],
        constructors: vec![
            Constructor {
                name: "Empty".to_string(),
                fields: vec![],
            },
            Constructor {
                name: "Cons".to_string(),
                fields: vec![
                    Field {
                        name: "item".to_string(),
                        typ: Typ {
                            name: "a".to_string(),
                            body: vec![],
                        },
                    },
                    Field {
                        name: "next".to_string(),
                        typ: Typ {
                            name: "Rc".to_string(),
                            body: vec![Typ {
                                name: "List".to_string(),
                                body: vec![Typ {
                                    name: "a".to_string(),
                                    body: vec![],
                                }],
                            }],
                        },
                    },
                ],
            },
        ],
    };
    let fun = Statement::Fn {
        name: "plus_one".to_string(),
        parameters: vec![Field {
            name: "x".to_string(),
            typ: Typ {
                name: "Int".to_string(),
                body: vec![],
            },
        }],
        return_typ: Typ {
            name: "Int".to_string(),
            body: vec![],
        },
        body: vec![
            Expression::BinOp {
                op: BinOp::Add,
                left: Box::new(Expression::Var {
                    name: "x".to_string(),
                }),
                right: Box::new(Expression::Integer { value: 1 }),
            },
            Expression::BinOp {
                op: BinOp::Add,
                left: Box::new(Expression::Var {
                    name: "x".to_string(),
                }),
                right: Box::new(Expression::Integer { value: 1 }),
            },
            Expression::Call {
                name: "test0".to_string(),
                arguments: vec![],
            },
            Expression::Call {
                name: "test1".to_string(),
                arguments: vec![Expression::Var {
                    name: "x".to_string(),
                }],
            },
            Expression::Call {
                name: "test2".to_string(),
                arguments: vec![
                    Expression::Var {
                        name: "x".to_string(),
                    },
                    Expression::Var {
                        name: "x".to_string(),
                    },
                ],
            },
        ],
    };
    println!("{}", compile(recursive));
    println!("{}", compile(fun));
}
