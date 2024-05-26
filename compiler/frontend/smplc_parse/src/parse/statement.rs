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
                if !token_stream.in_loop {
                    return Err(ParseError {
                        kind: ParseErrorKind::ContinueOutsideLoop,
                        span: token_stream.current().span,
                    });
                }

                token_stream.next_token();
                token_stream.consume(TokenValue::Semicolon)?;

                Ok(Self::Continue)
            }

            TokenValue::Break => {
                if !token_stream.in_loop {
                    return Err(ParseError {
                        kind: ParseErrorKind::BreakOutsideLoop,
                        span: token_stream.current().span,
                    });
                }

                token_stream.next_token();
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

        token_stream.consume(TokenValue::Colon)?;

        let ty = Type::parse(token_stream)?;

        let value = {
            token_stream
                .try_consume(TokenValue::Assign)
                .then(|| Spanned::<Expr>::parse(token_stream))
                .transpose()?
        };

        token_stream.consume(TokenValue::Semicolon)?;

        Ok(DeclareStatement { id, ty, value })
    }
}

impl<'source> Parse<'source> for ExprStatement<'source> {
    fn parse(token_stream: &mut TokenStream<'source>) -> ParseResult<'source, Self> {
        let expr = Spanned::<Expr>::parse(token_stream)?;

        if let Expr::Atom(Atom::Id(id)) = expr.0 {
            if token_stream.try_consume(TokenValue::Assign) {
                let id = id;

                let rhs = Spanned::<Expr>::parse(token_stream)?;

                token_stream.consume(TokenValue::Semicolon)?;

                return Ok(ExprStatement::Assign { id, rhs });
            }
        }

        token_stream.consume(TokenValue::Semicolon)?;

        Ok(ExprStatement::Expr(expr))
    }
}

impl<'source> Parse<'source> for IfStatement<'source> {
    fn parse(token_stream: &mut TokenStream<'source>) -> ParseResult<'source, Self> {
        token_stream.consume(TokenValue::If)?;

        let cond = Spanned::<Expr>::parse(token_stream)?;
        let body = Block::parse(token_stream)?;

        let else_body = {
            token_stream
                .try_consume(TokenValue::Else)
                .then(|| Block::parse(token_stream))
                .transpose()?
        };

        Ok(IfStatement {
            cond,
            body,
            else_body,
        })
    }
}

impl<'source> Parse<'source> for ReturnStatement<'source> {
    fn parse(token_stream: &mut TokenStream<'source>) -> ParseResult<'source, Self> {
        token_stream.consume(TokenValue::Return)?;

        let value = {
            (!token_stream.check(TokenValue::Semicolon))
                .then(|| Spanned::<Expr>::parse(token_stream))
                .transpose()?
        };

        token_stream.consume(TokenValue::Semicolon)?;

        Ok(ReturnStatement { value })
    }
}

impl<'source> Parse<'source> for WhileStatement<'source> {
    fn parse(token_stream: &mut TokenStream<'source>) -> ParseResult<'source, Self> {
        token_stream.consume(TokenValue::While)?;

        let cond = Spanned::<Expr>::parse(token_stream)?;

        let in_loop = token_stream.in_loop;
        token_stream.in_loop = true;

        let body = Block::parse(token_stream)?;

        token_stream.in_loop = in_loop;

        Ok(WhileStatement { cond, body })
    }
}
