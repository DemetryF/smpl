use crate::lexer::token::{self, operator::Operator, token_value::TokenValue};

use self::{
    ast::{Atom, Block, Expr, Statement},
    token_stream::TokenStream,
};

pub mod ast;
pub mod token_stream;

pub struct Parser<'code> {
    token_stream: TokenStream<'code>,
}

struct PowerBinding;
impl PowerBinding {
    pub fn prefix(op: Operator) -> ((), u8) {
        match op {
            Operator::Subtraction | Operator::Addition | Operator::Not => ((), 17),

            _ => panic!("operator is not prefix"),
        }
    }

    pub fn infix(op: Operator) -> Option<(u8, u8)> {
        Some(match op {
            Operator::Assignment
            | Operator::AdditionAssignment
            | Operator::SubtractionAssignment
            | Operator::MultiplicationAssignment
            | Operator::DivisionAssignment => (2, 1),

            Operator::Or => (3, 4),
            Operator::And => (5, 6),

            Operator::Equal | Operator::NotEqual => (7, 8),

            Operator::GreaterOrEqual
            | Operator::Greater
            | Operator::LessOrEqual
            | Operator::Less => (9, 10),

            Operator::Subtraction | Operator::Addition => (11, 12),

            Operator::Multiplication
            | Operator::Division
            | Operator::WholeDivision
            | Operator::ModuloDivision => (13, 14),

            Operator::Exponentiation => (16, 15),

            _ => return None,
        })
    }
}

impl<'code> Parser<'code> {
    pub fn new(code: &'code str) -> Self {
        Self {
            token_stream: TokenStream::new(code),
        }
    }

    pub fn statement(&mut self) -> Statement<'code> {
        match self.token_stream.current().value {
            TokenValue::Define => self.declare_statement(),
            TokenValue::If => self.if_statement(),
            TokenValue::Function => self.function_statement(),
            TokenValue::While => self.while_statement(),
            TokenValue::Return => self.return_statement(),

            _ => {
                let expr = Statement::Expr(self.expr());
                self.token_stream.accept(&TokenValue::Semicolon);
                expr
            }
        }
    }

    fn declare_statement(&mut self) -> Statement<'code> {
        self.token_stream.accept(&TokenValue::Define);

        let id = self.id();
        let mut expr = None;

        if self
            .token_stream
            .check(&TokenValue::Operator(Operator::Assignment))
        {
            self.token_stream.skip();
            expr = Some(self.expr());
        }

        self.token_stream.accept(&TokenValue::Semicolon);

        Statement::Declare { id, expr }
    }

    fn if_statement(&mut self) -> Statement<'code> {
        self.token_stream.accept(&TokenValue::If);

        let cond = self.parenthesis();
        let then_body = self.block();

        let else_body = if self.token_stream.check(&TokenValue::Else) {
            self.token_stream.skip();
            Some(self.block())
        } else {
            None
        };

        Statement::If {
            cond,
            then_body,
            else_body,
        }
    }

    fn function_statement(&mut self) -> Statement<'code> {
        self.token_stream.accept(&TokenValue::Function);

        let id = self.id();
        let args = self.fargs();
        let body = self.block();

        Statement::Function { id, args, body }
    }

    fn while_statement(&mut self) -> Statement<'code> {
        self.token_stream.accept(&TokenValue::While);

        let cond = self.parenthesis();
        let body = self.block();

        Statement::While { cond, body }
    }

    fn return_statement(&mut self) -> Statement<'code> {
        self.token_stream.accept(&TokenValue::Return);

        let expr = if self.token_stream.check(&TokenValue::Semicolon) {
            None
        } else {
            Some(self.expr())
        };

        self.token_stream.accept(&TokenValue::Semicolon);

        Statement::Return(expr)
    }

    fn fargs(&mut self) -> Vec<&'code str> {
        let mut args = Vec::new();

        self.token_stream.accept(&TokenValue::OpeningParen);

        if !self.token_stream.check(&TokenValue::ClosingParen) {
            args.push(self.id());

            while self.token_stream.check(&TokenValue::Comma) {
                self.token_stream.skip();
                args.push(self.id());
            }
        }

        self.token_stream.accept(&TokenValue::ClosingParen);

        args
    }

    fn block(&mut self) -> Block<'code> {
        let mut stmts = Vec::new();

        self.token_stream.accept(&TokenValue::OpeningBrace);
        while !self.token_stream.check(&TokenValue::ClosingBrace) {
            stmts.push(self.statement())
        }
        self.token_stream.accept(&TokenValue::ClosingBrace);

        Block(stmts)
    }

    fn id(&mut self) -> &'code str {
        match self.token_stream.current().value {
            TokenValue::Id(value) => {
                self.token_stream.skip();
                value
            }
            _ => panic!("expected id"),
        }
    }

    fn expr(&mut self) -> Expr<'code> {
        self.expr_bp(0)
    }

    fn expr_bp(&mut self, bp: u8) -> Expr<'code> {
        let mut lhs = match self.token_stream.skip().value {
            TokenValue::Literal(literal) => Expr::Atom(Atom::Literal(literal)),
            TokenValue::Id(id) => {
                if self.token_stream.check(&TokenValue::OpeningParen) {
                    self.token_stream.skip();
                    let mut args = Vec::new();

                    if !self.token_stream.check(&TokenValue::ClosingParen) {
                        args.push(self.expr());
                        while self.token_stream.check(&TokenValue::Comma) {
                            self.token_stream.skip();
                            args.push(self.expr());
                        }
                    }

                    self.token_stream.accept(&TokenValue::ClosingParen);

                    Expr::Call { id, args }
                } else {
                    Expr::Atom(Atom::Id(id))
                }
            }

            TokenValue::OpeningParen => self.parenthesis(),

            TokenValue::Operator(op) => {
                let ((), r_bp) = PowerBinding::prefix(op);
                let rhs = self.expr_bp(r_bp);

                Expr::Unary {
                    op,
                    expr: Box::new(rhs),
                }
            }

            t => panic!("bad token: {:?}", t),
        };

        loop {
            let op = match self.token_stream.current().value {
                TokenValue::Operator(op) => op,
                _ => break,
            };

            self.token_stream.skip();

            if let Some((l_bp, r_bp)) = PowerBinding::infix(op) {
                if l_bp < bp {
                    break;
                }

                lhs = {
                    let rhs = self.expr_bp(r_bp);
                    Expr::Binary {
                        left: Box::new(lhs),
                        op,
                        right: Box::new(rhs),
                    }
                };

                continue;
            }

            break;
        }

        lhs
    }

    fn parenthesis(&mut self) -> Expr<'code> {
        // self.token_stream.accept(&TokenValue::OpeningParen);
        let expr = self.expr_bp(0);
        self.token_stream.accept(&TokenValue::ClosingParen);

        expr
    }
}
