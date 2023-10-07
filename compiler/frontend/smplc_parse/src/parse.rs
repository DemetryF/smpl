use smplc_ast::{
    expr::{Atom, Ident},
    operators::AssignOp,
    statement::{
        BreakStatement, ContinueStatement, DeclareStatement, ExprStatement, FunctionStatement,
        IfStatement, ReturnStatement, WhileStatement,
    },
    Block, Expr, Statement,
};

use smplc_lexer::token::TokenValue;

use crate::{
    error::{ParseError, ParseResult},
    token_stream::TokenStream,
};

pub trait Parse<'source>: Sized {
    fn parse(token_stream: &mut TokenStream<'source>) -> ParseResult<'source, Self>;
}

impl<'source> Parse<'source> for Statement<'source> {
    fn parse(token_stream: &mut TokenStream<'source>) -> ParseResult<'source, Self> {
        Ok({
            match token_stream.current().value {
                TokenValue::Break => Self::Break(BreakStatement::parse(token_stream)?),
                TokenValue::Continue => Self::Continue(ContinueStatement::parse(token_stream)?),
                TokenValue::Fn => Self::Function(FunctionStatement::parse(token_stream)?),
                TokenValue::If => Self::If(IfStatement::parse(token_stream)?),
                TokenValue::Let => Self::Declare(DeclareStatement::parse(token_stream)?),
                TokenValue::Return => Self::Return(ReturnStatement::parse(token_stream)?),
                TokenValue::While => Self::While(WhileStatement::parse(token_stream)?),

                _ => Self::Expr(ExprStatement::parse(token_stream)?),
            }
        })
    }
}

impl<'source> Parse<'source> for BreakStatement {
    fn parse(token_stream: &mut TokenStream<'source>) -> ParseResult<'source, Self> {
        let break_pos = token_stream.consume(TokenValue::Break)?.pos;

        token_stream.consume(TokenValue::Semicolon)?;

        if !token_stream.in_cycle {
            return Err(ParseError::break_outside_cycle(break_pos));
        }

        Ok(BreakStatement)
    }
}

impl<'source> Parse<'source> for ContinueStatement {
    fn parse(token_stream: &mut TokenStream<'source>) -> ParseResult<'source, Self> {
        let continue_pos = token_stream.consume(TokenValue::Continue)?.pos;

        token_stream.consume(TokenValue::Break)?;

        if !token_stream.in_cycle {
            return Err(ParseError::continue_outside_cycle(continue_pos));
        }

        Ok(ContinueStatement)
    }
}

impl<'source> Parse<'source> for DeclareStatement<'source> {
    fn parse(token_stream: &mut TokenStream<'source>) -> ParseResult<'source, Self> {
        token_stream.consume(TokenValue::Let)?;

        let id = Ident::parse(token_stream)?;

        let expr = {
            if token_stream.try_consume(TokenValue::Assign) {
                Some(Expr::parse(token_stream)?)
            } else {
                None
            }
        };

        token_stream.consume(TokenValue::Semicolon)?;

        Ok(DeclareStatement { id, expr })
    }
}

impl<'source> Parse<'source> for ExprStatement<'source> {
    fn parse(token_stream: &mut TokenStream<'source>) -> ParseResult<'source, Self> {
        let expr = Expr::parse(token_stream)?;

        let result = {
            if let Expr::Atom(Atom::Ident(lhs)) = expr {
                if let Ok(op) = AssignOp::try_from(token_stream.current().value) {
                    let rhs = Expr::parse(token_stream)?;

                    ExprStatement::Assign { lhs, op, rhs }
                } else {
                    ExprStatement::Expr(expr)
                }
            } else {
                ExprStatement::Expr(expr)
            }
        };

        token_stream.consume(TokenValue::Semicolon)?;

        Ok(result)
    }
}

impl<'source> Parse<'source> for FunctionStatement<'source> {
    fn parse(token_stream: &mut TokenStream<'source>) -> ParseResult<'source, Self> {
        token_stream.consume(TokenValue::Fn)?;

        let id = Ident::parse(token_stream)?;

        token_stream.consume(TokenValue::LParen)?;

        let mut args = Vec::new();

        while let Ok(id) = Ident::parse(token_stream) {
            args.push(id);

            if token_stream.try_consume(TokenValue::RParen) {
                break;
            }

            token_stream.consume(TokenValue::Comma)?;
        }

        let body = Block::parse(token_stream)?;

        Ok(FunctionStatement { id, args, body })
    }
}

impl<'source> Parse<'source> for IfStatement<'source> {
    fn parse(token_stream: &mut TokenStream<'source>) -> ParseResult<'source, Self> {
        token_stream.consume(TokenValue::If)?;

        let cond = Expr::parse(token_stream)?;

        let then_branch = Block::parse(token_stream)?;

        let else_branch = token_stream
            .try_consume(TokenValue::Else)
            .then(|| Block::parse(token_stream))
            .transpose()?;

        Ok(IfStatement {
            cond,
            then_branch,
            else_branch,
        })
    }
}

impl<'source> Parse<'source> for ReturnStatement<'source> {
    fn parse(token_stream: &mut TokenStream<'source>) -> ParseResult<'source, Self> {
        let return_pos = token_stream.consume(TokenValue::Return)?.pos;

        let expr = (!token_stream.check(TokenValue::Semicolon))
            .then(|| Expr::parse(token_stream))
            .transpose()?;

        if !token_stream.in_function {
            return Err(ParseError::return_outside_function(return_pos));
        }

        Ok(ReturnStatement { expr })
    }
}

impl<'source> Parse<'source> for WhileStatement<'source> {
    fn parse(token_stream: &mut TokenStream<'source>) -> ParseResult<'source, Self> {
        token_stream.consume(TokenValue::While)?;

        let cond = Expr::parse(token_stream)?;
        let body = Block::parse(token_stream)?;

        Ok(WhileStatement { cond, body })
    }
}

impl<'source> Parse<'source> for Block<'source> {
    fn parse(token_stream: &mut TokenStream<'source>) -> ParseResult<'source, Self> {
        token_stream.consume(TokenValue::LBrace)?;

        let mut statements = Vec::new();

        while let Ok(statement) = Statement::parse(token_stream) {
            statements.push(statement);
        }

        token_stream.consume(TokenValue::RBrace)?;

        Ok(Block { statements })
    }
}

impl<'source> Parse<'source> for Ident<'source> {
    fn parse(token_stream: &mut TokenStream<'source>) -> ParseResult<'source, Self> {
        match token_stream.current().value {
            TokenValue::Ident(value) => Ok(token_stream.next().map(value)),

            _ => Err(token_stream.unexpected_token()),
        }
    }
}
