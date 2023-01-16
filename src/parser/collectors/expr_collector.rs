use crate::{
    lexer::token::{
        operator::Operator,
        token_value::{Literal, TokenValue},
    },
    parser::{
        ast::{Atom, Expr},
        parser_utils::ParserUtils,
        power_bindings::PowerBinding,
        token_stream::TokenStream,
    },
};

pub struct ExprCollector;
impl ExprCollector {
    pub fn collect<'code>(token_stream: &mut TokenStream<'code>) -> Expr<'code> {
        Self::expr_bp(token_stream, 0)
    }

    fn expr_bp<'code>(token_stream: &mut TokenStream<'code>, bp: u8) -> Expr<'code> {
        let mut lhs = Self::fact(token_stream);

        while let TokenValue::Operator(op) = token_stream.current().value {
            token_stream.skip();

            if let Some((l_bp, r_bp)) = PowerBinding::infix(op) {
                if l_bp < bp {
                    break;
                }

                lhs = {
                    let rhs = Self::expr_bp(token_stream, r_bp);
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

    fn fact<'code>(token_stream: &mut TokenStream<'code>) -> Expr<'code> {
        match token_stream.current().value {
            TokenValue::Literal(literal) => Self::literal(token_stream, literal),
            TokenValue::OpeningParen => ParserUtils::parenthesis(token_stream),
            TokenValue::Operator(op) => Self::unary(token_stream, op),

            TokenValue::Id(id) => {
                if token_stream.following().value == TokenValue::OpeningParen {
                    Self::call(token_stream, id)
                } else {
                    token_stream.skip();
                    Expr::Atom(Atom::Id(id))
                }
            }

            t => panic!("bad token: {:?}", t),
        }
    }

    fn unary<'code>(token_stream: &mut TokenStream<'code>, op: Operator) -> Expr<'code> {
        token_stream.skip();

        let ((), r_bp) = PowerBinding::prefix(op);
        let rhs = Self::expr_bp(token_stream, r_bp);

        Expr::Unary {
            op,
            expr: Box::new(rhs),
        }
    }

    fn literal<'code>(token_stream: &mut TokenStream<'code>, literal: Literal) -> Expr<'code> {
        token_stream.skip();
        Expr::Atom(Atom::Literal(literal))
    }

    fn call<'code>(token_stream: &mut TokenStream<'code>, id: &'code str) -> Expr<'code> {
        token_stream.skip();
        let args = Self::call_args(token_stream);

        Expr::Call { id, args }
    }

    fn call_args<'code>(token_stream: &mut TokenStream<'code>) -> Vec<Expr<'code>> {
        let mut args = Vec::new();

        token_stream.accept(&TokenValue::OpeningParen);

        if !token_stream.check(&TokenValue::ClosingParen) {
            args.push(Self::collect(token_stream));

            while token_stream.check(&TokenValue::Comma) {
                token_stream.skip();
                args.push(Self::collect(token_stream));
            }
        }

        token_stream.accept(&TokenValue::ClosingParen);

        args
    }
}
