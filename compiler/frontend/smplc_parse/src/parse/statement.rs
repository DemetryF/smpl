use smplc_ast::*;
use smplc_lexer::TokenValue;

use crate::error::{ParseError, ParseErrorKind, ParseResult};
use crate::{Parse, TokenStream};

impl<'source> Parse<'source> for Statement<'source> {
    fn parse(token_stream: &mut TokenStream<'source>) -> ParseResult<'source, Self> {
        match token_stream.current().value {
            TokenValue::Let => DeclareStatement::parse(token_stream).map(Self::Declare),
            TokenValue::If => IfStatement::parse(token_stream).map(Self::If),
            TokenValue::Return => ReturnStatement::parse(token_stream).map(Self::Return),
            TokenValue::While => WhileStatement::parse(token_stream).map(Self::While),

            TokenValue::Continue => {
                if !token_stream.in_cycle {
                    return Err(ParseError {
                        kind: ParseErrorKind::ContinueOutsideCycle,
                        pos: token_stream.get_pos(),
                    });
                }

                token_stream.next();
                token_stream.consume(TokenValue::Semicolon)?;

                Ok(Self::Continue)
            }

            TokenValue::Break => {
                if !token_stream.in_cycle {
                    return Err(ParseError {
                        kind: ParseErrorKind::BreakOutsideCycle,
                        pos: token_stream.get_pos(),
                    });
                }

                token_stream.next();
                token_stream.consume(TokenValue::Semicolon)?;

                Ok(Self::Break)
            }

            _ => ExprStatement::parse(token_stream).map(Self::Expr),
        }
    }
}

impl<'source> Parse<'source> for DeclareStatement<'source> {
    fn parse(token_stream: &mut TokenStream<'source>) -> ParseResult<'source, Self> {
        token_stream.consume(TokenValue::Let)?;

        let id = Id::parse(token_stream)?;
        let init_expr = parse_init_expr(token_stream)?;

        token_stream.consume(TokenValue::Semicolon)?;

        Ok(DeclareStatement { id, init_expr })
    }
}

fn parse_init_expr<'source>(
    token_stream: &mut TokenStream<'source>,
) -> ParseResult<'source, Option<Expr<'source>>> {
    if token_stream.try_consume(TokenValue::Assign) {
        let expr = Expr::parse(token_stream)?;

        return Ok(Some(expr));
    }

    Ok(None)
}

impl<'source> Parse<'source> for ExprStatement<'source> {
    fn parse(token_stream: &mut TokenStream<'source>) -> ParseResult<'source, Self> {
        let expr = Expr::parse(token_stream)?;

        if let Expr::Atom(Atom::Id(id)) = expr {
            if token_stream.try_consume(TokenValue::Assign) {
                let expr = Expr::parse(token_stream)?;

                token_stream.consume(TokenValue::Semicolon)?;

                return Ok(ExprStatement::Assign { id, expr });
            }
        }

        token_stream.consume(TokenValue::Semicolon)?;

        Ok(ExprStatement::Expr(expr))
    }
}

impl<'source> Parse<'source> for IfStatement<'source> {
    fn parse(token_stream: &mut TokenStream<'source>) -> ParseResult<'source, Self> {
        token_stream.consume(TokenValue::If)?;

        let condition = Expr::parse(token_stream)?;
        let then_body = Block::parse(token_stream)?;
        let else_body = parse_else_body(token_stream)?;

        Ok(IfStatement {
            condition,
            then_body,
            else_body,
        })
    }
}

fn parse_else_body<'source>(
    token_stream: &mut TokenStream<'source>,
) -> ParseResult<'source, Option<Block<'source>>> {
    let else_body = {
        if token_stream.try_consume(TokenValue::Else) {
            Some(Block::parse(token_stream)?)
        } else {
            None
        }
    };

    Ok(else_body)
}

impl<'source> Parse<'source> for ReturnStatement<'source> {
    fn parse(token_stream: &mut TokenStream<'source>) -> ParseResult<'source, Self> {
        token_stream.consume(TokenValue::Return)?;
        let expr = parse_return_expr(token_stream)?;
        token_stream.consume(TokenValue::Semicolon)?;

        Ok(ReturnStatement(expr))
    }
}

fn parse_return_expr<'source>(
    token_stream: &mut TokenStream<'source>,
) -> ParseResult<'source, Option<Expr<'source>>> {
    Ok({
        if token_stream.check(TokenValue::Semicolon) {
            None
        } else {
            Some(Expr::parse(token_stream)?)
        }
    })
}

impl<'source> Parse<'source> for WhileStatement<'source> {
    fn parse(token_stream: &mut TokenStream<'source>) -> ParseResult<'source, Self> {
        token_stream.consume(TokenValue::While)?;

        let in_cycle = token_stream.in_cycle;
        token_stream.in_cycle = true;

        let condition = Expr::parse(token_stream)?;
        let body = Block::parse(token_stream)?;

        token_stream.in_cycle = in_cycle;

        Ok(WhileStatement { condition, body })
    }
}
