use crate::{
    lexer::token::{operator::Operator, token_value::Literal},
    parser::{
        ast::{
            block::Block,
            expr::{call::Call, unary::Unary, Atom, Binary, Expr},
            statement::{
                declare_statement::DeclareStatement, function_statement::FunctionStatement,
                if_statement::IfStatement, return_statement::ReturnStatement,
                while_statement::WhileStatement, Statement,
            },
        },
        Parser,
    },
};

#[derive(Clone, Debug)]
pub struct Label(pub String);

#[derive(Debug)]
pub enum Instruction {
    Binary {
        result: String,
        left: Atom,
        op: Operator,
        right: Atom,
    },
    Unary {
        result: String,
        op: Operator,
        operand: Atom,
    },
    Assign {
        what: Atom,
        to: String,
    },
    Goto {
        to: Label,
    },
    IfFalse {
        cond: Atom,
        to: Label,
    },
    Call {
        result: String,
        name: String,
        args_count: usize,
    },

    Label(Label),
    Return(Option<Atom>),
    Push(Atom),
    Pop(String),
}

pub struct Translator {
    pub instructions: Vec<Instruction>,
    parser: Parser,

    temps_count: usize,
    pub ifs_count: usize,
    pub whiles_count: usize,
}

impl Translator {
    pub fn new(code: String) -> Self {
        Self {
            instructions: Vec::new(),
            parser: Parser::new(code),

            temps_count: 0,
            ifs_count: 0,
            whiles_count: 0,
        }
    }

    pub fn get_temp_var(&mut self) -> String {
        self.temps_count += 1;
        return String::from("t") + self.temps_count.to_string().as_str();
    }

    pub fn push(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }

    pub fn translate(&mut self) {
        self.parser.parse().translate(self);
    }
}

pub trait Translate {
    fn translate(self, translator: &mut Translator) -> Option<Atom>;
}

impl Translate for Expr {
    fn translate(self, translator: &mut Translator) -> Option<Atom> {
        match self {
            Self::Binary(binary) => binary.translate(translator),
            Self::Unary(unary) => unary.translate(translator),
            Self::Call(call) => call.translate(translator),
            Self::Atom(atom) => Some(atom),
        }
    }
}

impl Translate for Binary {
    fn translate(self, translator: &mut Translator) -> Option<Atom> {
        let result = translator.get_temp_var();

        let left = self.lhs.translate(translator).expect("");
        let right = self.rhs.translate(translator).expect("");

        translator.push(Instruction::Binary {
            result: result.clone(),
            left,
            op: self.op,
            right,
        });

        Some(Atom::Id(result))
    }
}

impl Translate for Unary {
    fn translate(self, translator: &mut Translator) -> Option<Atom> {
        let result = translator.get_temp_var();

        let operand = self.rhs.translate(translator).expect("msg");

        translator.push(Instruction::Unary {
            result: result.clone(),
            op: self.op,
            operand,
        });

        Some(Atom::Id(result))
    }
}

impl Translate for Call {
    fn translate(self, translator: &mut Translator) -> Option<Atom> {
        let args_count = self.args.len();
        let result = translator.get_temp_var();

        for arg in self.args {
            let arg_result = arg.translate(translator).expect("");
            translator.push(Instruction::Push(arg_result));
        }

        translator.push(Instruction::Call {
            result: result.clone(),
            name: self.id,
            args_count,
        });

        Some(Atom::Id(result))
    }
}

impl Translate for DeclareStatement {
    fn translate(self, translator: &mut Translator) -> Option<Atom> {
        let value = if let Some(expr) = self.expr {
            expr.translate(translator).expect("")
        } else {
            Atom::Literal(Literal::Number(0.0))
        };

        translator.push(Instruction::Assign {
            what: value,
            to: self.id,
        });

        None
    }
}

impl Translate for FunctionStatement {
    fn translate(self, translator: &mut Translator) -> Option<Atom> {
        translator.push(Instruction::Label(Label(self.id)));

        for arg in self.args {
            translator.push(Instruction::Pop(arg))
        }

        self.body.translate(translator);

        None
    }
}

impl Translate for IfStatement {
    fn translate(self, translator: &mut Translator) -> Option<Atom> {
        translator.ifs_count += 1;

        let binding = translator.ifs_count.to_string();
        let ifs_count = &binding.as_str();

        let cond = self.cond.translate(translator).expect("");
        let endif_label = Label(String::from("endif") + ifs_count);

        if let Some(else_body) = self.else_body {
            let else_label = Label(String::from("else") + ifs_count);

            translator.push(Instruction::IfFalse {
                cond,
                to: else_label.clone(),
            });

            self.then_body.translate(translator);
            translator.push(Instruction::Goto {
                to: endif_label.clone(),
            });

            translator.push(Instruction::Label(else_label));
            else_body.translate(translator);
        } else {
            translator.push(Instruction::IfFalse {
                cond,
                to: endif_label.clone(),
            });
            self.then_body.translate(translator);
        }

        translator.push(Instruction::Label(endif_label));

        None
    }
}

impl Translate for WhileStatement {
    fn translate(self, translator: &mut Translator) -> Option<Atom> {
        translator.whiles_count += 1;

        let binding = translator.whiles_count.to_string();
        let whiles_count = &binding.to_string();

        let while_start_label = Label(String::from("while_start") + whiles_count);
        let while_end_label = Label(String::from("while_end") + whiles_count);

        translator.push(Instruction::Label(while_start_label.clone()));
        let cond = self.cond.translate(translator).expect("");

        translator.push(Instruction::IfFalse {
            cond,
            to: while_end_label.clone(),
        });

        self.body.translate(translator);

        translator.push(Instruction::Goto {
            to: while_start_label,
        });

        translator.push(Instruction::Label(while_end_label));

        None
    }
}

impl Translate for ReturnStatement {
    fn translate(self, translator: &mut Translator) -> Option<Atom> {
        if let Some(expr) = self.0 {
            let value = expr.translate(translator).expect("");
            translator.push(Instruction::Return(Some(value)));
        } else {
            translator.push(Instruction::Return(None));
        }

        None
    }
}

impl Translate for Statement {
    fn translate(self, translator: &mut Translator) -> Option<Atom> {
        match self {
            Self::Expr(expr) => expr.translate(translator),
            Self::Declare(declare) => declare.translate(translator),
            Self::Function(function) => function.translate(translator),
            Self::If(if_stmt) => if_stmt.translate(translator),
            Self::Return(return_stmt) => return_stmt.translate(translator),
            Self::While(while_stmt) => while_stmt.translate(translator),
        };

        None
    }
}

impl Translate for Block {
    fn translate(self, translator: &mut Translator) -> Option<Atom> {
        for stmt in self.0 {
            stmt.translate(translator);
        }

        None
    }
}
