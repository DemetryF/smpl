use smplc_ast::*;
use smplc_lexer::TokenValue;

use crate::error::{ParseError, ParseResult};
use crate::TokenStream;

use super::Parse;

impl<'source> Parse<'source> for Statement<'source> {
    fn parse(token_stream: &mut TokenStream<'source>) -> ParseResult<'source, Self> {
        match token_stream.current().value {
            TokenValue::Let => DeclareStatement::parse(token_stream).map(Self::Declare),
            TokenValue::Fn => FunctionStatement::parse(token_stream).map(Self::Function),
            TokenValue::If => IfStatement::parse(token_stream).map(Self::If),
            TokenValue::Return => ReturnStatement::parse(token_stream).map(Self::Return),
            TokenValue::While => WhileStatement::parse(token_stream).map(Self::While),

            TokenValue::Continue => {
                token_stream.next();
                token_stream.consume(TokenValue::Semicolon)?;

                Ok(Self::Continue)
            }

            TokenValue::Break => {
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

impl<'source> Parse<'source> for FunctionStatement<'source> {
    fn parse(token_stream: &mut TokenStream<'source>) -> ParseResult<'source, Self> {
        token_stream.consume(TokenValue::Fn)?;

        let id = Id::parse(token_stream)?;
        let args = parse_args(token_stream)?;

        token_stream.in_function = true;
        let body = Block::parse(token_stream)?;
        token_stream.in_function = false;

        Ok(FunctionStatement { id, args, body })
    }
}

fn parse_args<'source>(
    token_stream: &mut TokenStream<'source>,
) -> ParseResult<'source, Vec<Id<'source>>> {
    let mut args = Vec::new();

    token_stream.consume(TokenValue::LParen)?;

    if token_stream.try_consume(TokenValue::RParen) {
        return Ok(args);
    }

    args.push(Id::parse(token_stream)?);

    while token_stream.try_consume(TokenValue::Comma) {
        args.push(Id::parse(token_stream)?);
    }

    token_stream.consume(TokenValue::RParen)?;

    Ok(args)
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
    let else_body = if token_stream.try_consume(TokenValue::Else) {
        let block = Block::parse(token_stream)?;

        Some(block)
    } else {
        None
    };

    Ok(else_body)
}

impl<'source> Parse<'source> for ReturnStatement<'source> {
    fn parse(token_stream: &mut TokenStream<'source>) -> ParseResult<'source, Self> {
        check_in_function(token_stream)?;

        token_stream.consume(TokenValue::Return)?;
        let expr = parse_return_expr(token_stream)?;
        token_stream.consume(TokenValue::Semicolon)?;

        Ok(ReturnStatement(expr))
    }
}

fn parse_return_expr<'source>(
    token_stream: &mut TokenStream<'source>,
) -> ParseResult<'source, Option<Expr<'source>>> {
    let maybe_expr = if token_stream.check(TokenValue::Semicolon) {
        None
    } else {
        let expr = Expr::parse(token_stream)?;

        Some(expr)
    };

    Ok(maybe_expr)
}

fn check_in_function<'source>(token_stream: &TokenStream<'source>) -> ParseResult<'source, ()> {
    if !token_stream.in_function {
        let error = ParseError::return_outside_function(token_stream.get_pos());

        return Err(error);
    }

    Ok(())
}

impl<'source> Parse<'source> for WhileStatement<'source> {
    fn parse(token_stream: &mut TokenStream<'source>) -> ParseResult<'source, Self> {
        token_stream.consume(TokenValue::While)?;

        let condition = Expr::parse(token_stream)?;
        let body = Block::parse(token_stream)?;

        Ok(WhileStatement { condition, body })
    }
}
