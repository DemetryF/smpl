use smplc_ast::expr::{Atom, Ident, Literal};
use smplc_ast::operators::*;
use smplc_ast::statement::*;
use smplc_ast::{Block, Expr, Statement};
use smplc_lexer::token::Pos;
use smplc_lexer::token::{Token, TokenValue};
use smplc_lexer::Lexer;

use crate::{parse, TokenStream};

macro_rules! num {
    ($num:literal) => {
        Expr::Atom(Atom::Literal(Literal::Num($num)))
    };
}

macro_rules! ident {
    ($str:literal) => {
        Expr::Atom(Atom::Ident(Ident {
            value: $str,
            pos: Pos::default(),
        }))
    };
}

macro_rules! bin {
    ($lhs:expr, $Op:ident, $rhs:expr) => {
        Expr::Binary {
            lhs: Box::new($lhs),
            op: BinOp::$Op,
            rhs: Box::new($rhs),
        }
    };
}

macro_rules! un {
    ($Op:ident, $rhs:expr) => {
        Expr::Unary {
            op: UnOp::$Op,
            rhs: Box::new($rhs),
        }
    };
}

macro_rules! stmt {
    (break) => {
        Statement::Break(BreakStatement)
    };

    (continue) => {
        Statement::Continue(ContinueStatement)
    };

    (let $ident:literal $(= $expr:expr)?) => {
        {
            let expr = [
                $($expr)?
            ].into_iter().next();

            let id = Ident {
                value: $ident,
                pos: Pos::default()
            };

            Statement::Declare(DeclareStatement {
                id,
                expr
            })
        }
    };

    (expr $expr:expr) => {
        Statement::Expr(ExprStatement::Expr($expr))
    };

    (assign $ident:literal $Op:ident $value:expr) => {
        Statement::Expr(ExprStatement::Assign {
            lhs: ident!($ident),
            op: AssignOp::$Op,
            rhs: $value,
        })
    };

    (fn $name:literal( $($arg:literal),* ) $body:tt) => {
        {
            let id = Ident {
                value: $name,
                pos: Pos::default()
            };

            let args = vec![
                $(
                    Ident { value: $arg, pos: Pos::default() },
                )*
            ];

            let body = block!( $body );

            Statement::Function(FunctionStatement {
                id,
                args,
                body,
            })
        }
    };

    (if ($cond:expr) $then_body:tt $(else $else_body:tt)?) => {
        Statement::If(IfStatement {
            cond: $cond,
            then_branch: block!( $then_body ),
            else_branch: [ $(block!( $else_body ))? ].into_iter().next()
        })
    };

    (return $($expr:expr)?) => {
        Statement::Return(ReturnStatement {
            expr: [ $($expr)? ].into_iter().next()
        })
    };

    (while ($cond:expr) $body:tt) => {
        Statement::While(WhileStatement {
            cond: $cond,
            body: block!( $body )
        })
    };
}

macro_rules! block {
    ( { $( $hueta:expr )* } ) => {
        {
            let statements = vec![
                $(
                    $hueta,
                )*
            ];

            Block { statements }
        }
    };
}

macro_rules! parser_test {
    ($code:literal; $stmt:expr) => {
        let lexer = Lexer::new($code);
        let mut tokens = lexer.collect::<Result<Vec<_>, _>>().unwrap();

        tokens.extend(vec![Token {
            value: TokenValue::EOF,
            pos: Pos::default(),
        }]);

        let token_stream = TokenStream::new(tokens.into_iter());

        assert_eq!(parse(token_stream).unwrap()[0], $stmt);
    };
}

#[test]
pub fn break_statement() {
    parser_test! {
        "
            while 1 {
                break; 
            }
        ";

        stmt![
            while (num!(1.0)) {
                stmt![break]
            }
        ]
    };
}

#[test]
pub fn continue_statement() {
    parser_test! {
        "
            while 1 {
                continue; 
            }
        ";

        stmt![
            while (num!(1.0)) {
                stmt![continue]
            }
        ]
    };
}

#[test]
pub fn declare_statement() {
    parser_test! {
        "let a;";

        stmt![let "a"]
    };

    parser_test! {
        "let a = 0;";

        stmt![let "a" = num!(0.0)]
    };
}

#[test]
pub fn expr_statement() {
    parser_test! {
        "0;";

        stmt![expr num!(0.0)]
    }
}

#[test]
pub fn function_statement() {
    parser_test! {
        "fn a() {}";

        stmt![fn "a" () {}]
    }

    parser_test! {
        "fn a(a) {}";

        stmt![fn "a" ("a") {}]
    }

    parser_test! {
        "fn a(a, b) {}";

        stmt![fn "a" ("a", "b") {}]
    }
}

#[test]
pub fn if_statement() {
    parser_test! {
        "if 1 {}";

        stmt![if (num!(1.0)) {}]
    }

    parser_test! {
        "if 1 {} else {}";

        stmt![if (num!(1.0)) {} else {}]
    }
}

#[test]
pub fn while_statement() {
    parser_test! {
        "while 1 {}";

        stmt![while (num!(1.0)) {}]
    }
}

#[test]
pub fn block() {
    parser_test! {
        "
            if 1 {
                let a;
                let a = 1;
            }
        ";

        stmt![
            if (num!(1.0)) {
                stmt![let "a"]
                stmt![let "a" = num!(1.0)]
            }
        ]
    }
}

#[test]
pub fn return_statement() {
    parser_test! {
        "
            fn a() {
                return;
            }
        ";

        stmt![
            fn "a"() {
                stmt![return]
            }
        ]
    }

    parser_test! {
        "
            fn a() {
                return 1.0;
            }
        ";

        stmt![
            fn "a"() {
                stmt![return num!(1.0)]
            }
        ]
    }
}
