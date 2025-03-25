use smplc_ast::*;
use smplc_lexer::TokenTag;

use crate::{
    error::{ParseError, ParseErrorKind, ParseResult},
    token_stream::Tokens,
    Parse, TokenStream,
};

impl<'source> Parse<'source> for Statement<'source> {
    fn parse<TS: Tokens<'source>>(
        token_stream: &mut TokenStream<'source, TS>,
    ) -> ParseResult<'source, Self> {
        match token_stream.current().tag {
            TokenTag::Let => DeclareStatement::parse(token_stream).map(Self::Declare),
            TokenTag::If => IfStatement::parse(token_stream).map(Self::If),
            TokenTag::Return => ReturnStatement::parse(token_stream).map(Self::Return),
            TokenTag::While => WhileStatement::parse(token_stream).map(Self::While),

            TokenTag::Continue => {
                if !token_stream.in_loop {
                    return Err(ParseError {
                        kind: ParseErrorKind::ContinueOutsideLoop,
                        span: token_stream.current().span,
                    });
                }

                token_stream.next_token()?;
                token_stream.consume(TokenTag::Semicolon)?;

                Ok(Self::Continue)
            }

            TokenTag::Break => {
                if !token_stream.in_loop {
                    return Err(ParseError {
                        kind: ParseErrorKind::BreakOutsideLoop,
                        span: token_stream.current().span,
                    });
                }

                token_stream.next_token()?;
                token_stream.consume(TokenTag::Semicolon)?;

                Ok(Self::Break)
            }

            _ => ExprStatement::parse(token_stream).map(Self::Expr),
        }
    }
}

impl<'source> Parse<'source> for DeclareStatement<'source> {
    fn parse<TS: Tokens<'source>>(
        token_stream: &mut TokenStream<'source, TS>,
    ) -> ParseResult<'source, Self> {
        token_stream.consume(TokenTag::Let)?;

        let id = Id::parse(token_stream)?;

        let ty = {
            token_stream
                .try_consume(TokenTag::Colon)?
                .then(|| Id::parse(token_stream))
                .transpose()?
        };

        let value = {
            token_stream
                .try_consume(TokenTag::Assign)?
                .then(|| Spanned::<Expr>::parse(token_stream))
                .transpose()?
        };

        token_stream.consume(TokenTag::Semicolon)?;

        Ok(DeclareStatement { id, ty, value })
    }
}

impl<'source> Parse<'source> for ExprStatement<'source> {
    fn parse<TS: Tokens<'source>>(
        token_stream: &mut TokenStream<'source, TS>,
    ) -> ParseResult<'source, Self> {
        let expr = Spanned::<Expr>::parse(token_stream)?;

        if let Expr::Atom(Atom::Id(id)) = expr.0 {
            if token_stream.try_consume(TokenTag::Assign)? {
                let rhs = Spanned::<Expr>::parse(token_stream)?;

                token_stream.consume(TokenTag::Semicolon)?;

                return Ok(ExprStatement::Assign { id, rhs });
            }
        }

        token_stream.consume(TokenTag::Semicolon)?;

        Ok(ExprStatement::Expr(expr))
    }
}

impl<'source> Parse<'source> for IfStatement<'source> {
    fn parse<TS: Tokens<'source>>(
        token_stream: &mut TokenStream<'source, TS>,
    ) -> ParseResult<'source, Self> {
        token_stream.consume(TokenTag::If)?;

        let cond = Spanned::<Expr>::parse(token_stream)?;
        let body = Block::parse(token_stream)?;

        let else_body = {
            token_stream
                .try_consume(TokenTag::Else)?
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
    fn parse<TS: Tokens<'source>>(
        token_stream: &mut TokenStream<'source, TS>,
    ) -> ParseResult<'source, Self> {
        token_stream.consume(TokenTag::Return)?;

        let value = {
            (!token_stream.check(TokenTag::Semicolon))
                .then(|| Spanned::<Expr>::parse(token_stream))
                .transpose()?
        };

        token_stream.consume(TokenTag::Semicolon)?;

        Ok(ReturnStatement { value })
    }
}

impl<'source> Parse<'source> for WhileStatement<'source> {
    fn parse<TS: Tokens<'source>>(
        token_stream: &mut TokenStream<'source, TS>,
    ) -> ParseResult<'source, Self> {
        token_stream.consume(TokenTag::While)?;

        let cond = Spanned::<Expr>::parse(token_stream)?;

        let in_loop = token_stream.in_loop;
        token_stream.in_loop = true;

        let body = Block::parse(token_stream)?;

        token_stream.in_loop = in_loop;

        Ok(WhileStatement { cond, body })
    }
}
