use smplc_ast::*;
use smplc_lexer::TokenValue;

use crate::{error::ParseError, TokenStream};

use super::Parse;

impl Parse for Statement {
    fn parse(token_stream: &mut TokenStream) -> Result<Self, ParseError> {
        let stmt = match token_stream.current().value {
            TokenValue::Let => Self::Declare(DeclareStatement::parse(token_stream)?),
            TokenValue::Fn => Self::Function(FunctionStatement::parse(token_stream)?),
            TokenValue::If => Self::If(IfStatement::parse(token_stream)?),
            TokenValue::Return => Self::Return(ReturnStatement::parse(token_stream)?),
            TokenValue::While => Self::While(WhileStatement::parse(token_stream)?),

            _ => Self::Expr(ExprStatement::parse(token_stream)?),
        };

        Ok(stmt)
    }
}

impl Parse for DeclareStatement {
    fn parse(token_stream: &mut TokenStream) -> Result<Self, ParseError> {
        token_stream.consume(TokenValue::Let)?;

        let id = Id::parse(token_stream)?;
        let init_expr = parse_init_expr(token_stream)?;

        token_stream.consume(TokenValue::Semicolon)?;

        Ok(DeclareStatement { id, init_expr })
    }
}

fn parse_init_expr(token_stream: &mut TokenStream) -> Result<Option<Expr>, ParseError> {
    if token_stream.try_consume(TokenValue::Assign) {
        let expr = Expr::parse(token_stream)?;

        return Ok(Some(expr));
    }

    Ok(None)
}

impl Parse for ExprStatement {
    fn parse(token_stream: &mut TokenStream) -> Result<Self, ParseError> {
        let expr = Expr::parse(token_stream)?;
        token_stream.consume(TokenValue::Semicolon)?;

        Ok(ExprStatement(expr))
    }
}

impl Parse for FunctionStatement {
    fn parse(token_stream: &mut TokenStream) -> Result<Self, ParseError> {
        token_stream.consume(TokenValue::Fn)?;

        let id = Id::parse(token_stream)?;
        let args = parse_args(token_stream)?;

        token_stream.in_function = true;
        let body = Block::parse(token_stream)?;
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

    args.push(Id::parse(token_stream)?);

    while token_stream.try_consume(TokenValue::Comma) {
        args.push(Id::parse(token_stream)?);
    }

    token_stream.consume(TokenValue::RParen)?;

    Ok(args)
}

impl Parse for IfStatement {
    fn parse(token_stream: &mut TokenStream) -> Result<Self, ParseError> {
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

fn parse_else_body(token_stream: &mut TokenStream) -> Result<Option<Block>, ParseError> {
    let else_body = if token_stream.try_consume(TokenValue::Else) {
        let block = Block::parse(token_stream)?;

        Some(block)
    } else {
        None
    };

    Ok(else_body)
}

impl Parse for ReturnStatement {
    fn parse(token_stream: &mut TokenStream) -> Result<Self, ParseError> {
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
        let expr = Expr::parse(token_stream)?;

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

impl Parse for WhileStatement {
    fn parse(token_stream: &mut TokenStream) -> Result<Self, ParseError> {
        token_stream.consume(TokenValue::While)?;

        let condition = Expr::parse(token_stream)?;
        let body = Block::parse(token_stream)?;

        Ok(WhileStatement { condition, body })
    }
}
