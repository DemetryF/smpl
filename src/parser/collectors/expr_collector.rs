use crate::{
    lexer::token::token_value::TokenValue,
    parser::{
        ast::{Atom, Expr},
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
        let mut lhs = match token_stream.skip().value {
            TokenValue::Literal(literal) => Expr::Atom(Atom::Literal(literal)),

            TokenValue::Id(id) => {
                if token_stream.check(&TokenValue::OpeningParen) {
                    token_stream.skip();
                    let mut args = Vec::new();

                    if !token_stream.check(&TokenValue::ClosingParen) {
                        args.push(Self::collect(token_stream));
                        while token_stream.check(&TokenValue::Comma) {
                            token_stream.skip();
                            args.push(Self::collect(token_stream));
                        }
                    }

                    token_stream.accept(&TokenValue::ClosingParen);

                    Expr::Call { id, args }
                } else {
                    Expr::Atom(Atom::Id(id))
                }
            }

            TokenValue::OpeningParen => {
                let expr = Self::collect(token_stream);
                token_stream.accept(&TokenValue::ClosingParen);
                expr
            }

            TokenValue::Operator(op) => {
                let ((), r_bp) = PowerBinding::prefix(op);
                let rhs = Self::expr_bp(token_stream, r_bp);

                Expr::Unary {
                    op,
                    expr: Box::new(rhs),
                }
            }

            t => panic!("bad token: {:?}", t),
        };

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
}
