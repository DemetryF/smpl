use smplc_ast::Type;
use smplc_hir as hir;
use smplc_hir::Atom;
use smplc_thir::*;

pub trait Typed<'source> {
    type Typed;

    fn typed(self, symbols: &Symbols<'source>) -> Self::Typed;
}

impl<'source> Typed<'source> for hir::Statement<'source> {
    type Typed = Statement<'source>;

    fn typed(self, symbols: &Symbols<'source>) -> Self::Typed {
        match self {
            hir::Statement::Expr(expr_statement) => Statement::Expr(expr_statement.typed(symbols)),

            hir::Statement::If(if_statement) => {
                return Statement::If(IfStatement {
                    cond: if_statement.cond.typed(symbols),
                    body: if_statement.body.typed(symbols),
                    else_body: if_statement.else_body.map(|block| block.typed(symbols)),
                });
            }

            hir::Statement::Return(return_statement) => {
                return Statement::Return(ReturnStatement {
                    value: return_statement.value.map(|expr| expr.typed(symbols)),
                });
            }

            hir::Statement::While(while_statement) => {
                return Statement::While(WhileStatement {
                    cond: while_statement.cond.typed(symbols),
                    body: while_statement.body.typed(symbols),
                });
            }

            hir::Statement::Break => Statement::Break,
            hir::Statement::Continue => Statement::Continue,
        }
    }
}

impl<'source> Typed<'source> for hir::ExprStatement<'source> {
    type Typed = ExprStatement<'source>;

    fn typed(self, symbols: &Symbols<'source>) -> Self::Typed {
        match self {
            hir::ExprStatement::Assign { var, rhs } => ExprStatement::Assign {
                var,
                rhs: rhs.typed(symbols),
            },

            hir::ExprStatement::Expr(expr) => ExprStatement::Expr(expr.typed(symbols)),
        }
    }
}

impl<'source> Typed<'source> for hir::Block<'source> {
    type Typed = Block<'source>;

    fn typed(self, symbols: &Symbols<'source>) -> Self::Typed {
        let statements = self
            .statements
            .into_iter()
            .map(|stmt| stmt.typed(symbols))
            .collect();

        Block { statements }
    }
}

impl<'source> Typed<'source> for hir::Expr<'source> {
    type Typed = Expr<'source>;

    fn typed(self, symbols: &Symbols<'source>) -> Self::Typed {
        match self {
            hir::Expr::Binary { lhs, op, rhs } => {
                let lhs = lhs.typed(symbols);
                let rhs = rhs.typed(symbols);

                let ty = expr_ty(&rhs, symbols);

                Expr::Binary {
                    lhs: Box::new(lhs),
                    op: bin_op_typed(op, ty),
                    rhs: Box::new(rhs),
                }
            }

            hir::Expr::Unary { op, rhs } => {
                let rhs = rhs.typed(symbols);

                let op = match op {
                    hir::UnOp::Not => UnOp::Not,
                    hir::UnOp::Neg => UnOp::Neg(expr_ty(&rhs, symbols).try_into().unwrap()),
                };

                Expr::Unary {
                    op,
                    rhs: Box::new(rhs),
                }
            }

            hir::Expr::Call { fun, args } => {
                let args = args.into_iter().map(|expr| expr.typed(symbols)).collect();

                Expr::Call { fun, args }
            }

            hir::Expr::Atom(atom) => Expr::Atom(atom),
        }
    }
}

fn expr_ty(expr: &Expr, symbols: &Symbols) -> Type {
    match expr {
        Expr::Binary { op, .. } => match op {
            &BinOp::Arithm(_, ty) => ty.into(),
            BinOp::Rel(_, _) | BinOp::And | BinOp::Or => Type::Bool,
        },

        Expr::Unary { op, .. } => match op {
            &UnOp::Neg(ty) => ty.into(),
            UnOp::Not => Type::Bool,
        },

        &Expr::Call { fun: id, .. } => symbols.functions[id].ret_ty.unwrap(),

        Expr::Atom(Atom::Literal(lit)) => lit.ty,

        &Expr::Atom(Atom::Var(id)) => symbols.variables[id].ty,
    }
}

fn bin_op_typed(op: hir::BinOp, ty: Type) -> BinOp {
    if let Ok(op) = ArithmOp::try_from(op) {
        return BinOp::Arithm(op, ty.try_into().unwrap());
    }

    if let Ok(op) = RelOp::try_from(op) {
        return BinOp::Rel(op, ty.try_into().unwrap());
    }

    match op {
        hir::BinOp::Or => BinOp::Or,
        hir::BinOp::And => BinOp::And,

        _ => unreachable!(),
    }
}
