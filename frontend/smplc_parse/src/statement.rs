use smplc_ast::*;
use smplc_lexer::TokenValue;

use crate::{error::ParseError, TokenStream};

use super::Collect;

impl Collect for Statement {
    fn collect(token_stream: &mut TokenStream) -> Result<Self, ParseError> {
        let stmt = match token_stream.current().value {
            TokenValue::Let => Self::Declare(DeclareStatement::collect(token_stream)?),
            TokenValue::Fn => Self::Function(FunctionStatement::collect(token_stream)?),
            TokenValue::If => Self::If(IfStatement::collect(token_stream)?),
            TokenValue::Return => Self::Return(ReturnStatement::collect(token_stream)?),
            TokenValue::While => Self::While(WhileStatement::collect(token_stream)?),

            _ => Self::Expr(ExprStatement::collect(token_stream)?),
        };

        Ok(stmt)
    }
}

impl Collect for DeclareStatement {
    fn collect(token_stream: &mut TokenStream) -> Result<Self, ParseError> {
        token_stream.consume(TokenValue::Let)?;

        let id = Id::collect(token_stream)?;
        let init_expr = parse_init_expr(token_stream)?;

        token_stream.consume(TokenValue::Semicolon)?;

        Ok(DeclareStatement { id, init_expr })
    }
}

fn parse_init_expr(token_stream: &mut TokenStream) -> Result<Option<Expr>, ParseError> {
    if token_stream.try_consume(TokenValue::Assignment) {
        let expr = Expr::collect(token_stream)?;

        return Ok(Some(expr));
    }

    Ok(None)
}

impl Collect for ExprStatement {
    fn collect(token_stream: &mut TokenStream) -> Result<Self, ParseError> {
        let expr = Expr::collect(token_stream)?;
        token_stream.consume(TokenValue::Semicolon)?;

        Ok(ExprStatement(expr))
    }
}

impl Collect for FunctionStatement {
    fn collect(token_stream: &mut TokenStream) -> Result<Self, ParseError> {
        token_stream.consume(TokenValue::Fn)?;

        let id = Id::collect(token_stream)?;
        let args = parse_args(token_stream)?;

        token_stream.in_function = true;
        let body = Block::collect(token_stream)?;
        token_stream.in_function = false;

        Ok(FunctionStatement { id, args, body })
    }
}

fn parse_args(token_stream: &mut TokenStream) -> Result<Vec<Id>, ParseError> {
    let mut args = Vec::new();

    token_stream.consume(TokenValue::LParen)?;

    if token_stream.try_consume(TokenValue::RParen) {
        return Ok(args);
    }

    args.push(Id::collect(token_stream)?);

    while token_stream.try_consume(TokenValue::Comma) {
        args.push(Id::collect(token_stream)?);
    }

    token_stream.consume(TokenValue::RParen)?;

    Ok(args)
}

impl Collect for IfStatement {
    fn collect(token_stream: &mut TokenStream) -> Result<Self, ParseError> {
        token_stream.consume(TokenValue::If)?;

        let condition = Expr::collect(token_stream)?;
        let then_body = Block::collect(token_stream)?;
        let else_body = parse_else_body(token_stream)?;

        Ok(IfStatement {
            condition,
            then_body,
            else_body,
        })
    }
}

fn parse_else_body(token_stream: &mut TokenStream) -> Result<Option<Block>, ParseError> {
    let else_body = if token_stream.try_consume(TokenValue::Else) {
        let block = Block::collect(token_stream)?;

        Some(block)
    } else {
        None
    };

    Ok(else_body)
}

impl Collect for ReturnStatement {
    fn collect(token_stream: &mut TokenStream) -> Result<Self, ParseError> {
        check_in_function(token_stream)?;

        token_stream.consume(TokenValue::Return)?;
        let expr = return_expr(token_stream)?;
        token_stream.consume(TokenValue::Semicolon)?;

        Ok(ReturnStatement(expr))
    }
}

fn return_expr(token_stream: &mut TokenStream) -> Result<Option<Expr>, ParseError> {
    let maybe_expr = if token_stream.check(TokenValue::Semicolon) {
        None
    } else {
        let expr = Expr::collect(token_stream)?;

        Some(expr)
    };

    Ok(maybe_expr)
}

fn check_in_function(token_stream: &TokenStream) -> Result<(), ParseError> {
    if !token_stream.in_function {
        let error = ParseError::return_outside_function(token_stream.get_pos());

        return Err(error);
    }

    Ok(())
}

impl Collect for WhileStatement {
    fn collect(token_stream: &mut TokenStream) -> Result<Self, ParseError> {
        token_stream.consume(TokenValue::While)?;

        let condition = Expr::collect(token_stream)?;
        let body = Block::collect(token_stream)?;

        Ok(WhileStatement { condition, body })
    }
}
