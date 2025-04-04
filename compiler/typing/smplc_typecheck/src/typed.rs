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
                    cond: if_statement.cond.0.typed(symbols),
                    body: if_statement.body.typed(symbols),
                    else_body: if_statement.else_body.map(|block| block.typed(symbols)),
                });
            }

            hir::Statement::Return(return_statement) => {
                return Statement::Return(ReturnStatement {
                    value: return_statement.value.map(|expr| expr.0.typed(symbols)),
                });
            }

            hir::Statement::While(while_statement) => {
                return Statement::While(WhileStatement {
                    cond: while_statement.cond.0.typed(symbols),
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
                rhs: rhs.0.typed(symbols),
            },

            hir::ExprStatement::Expr(expr) => ExprStatement::Expr(expr.0.typed(symbols)),
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
                let lhs = lhs.0.typed(symbols);
                let rhs = rhs.0.typed(symbols);

                let lhs_ty = expr_ty(&lhs, symbols);
                let rhs_ty = expr_ty(&rhs, symbols);

                Expr::Binary {
                    lhs: Box::new(lhs),
                    op: bin_op_typed(op, lhs_ty, rhs_ty),
                    rhs: Box::new(rhs),
                }
            }

            hir::Expr::Unary { op, rhs } => {
                let rhs = Box::new(rhs.0.typed(symbols));

                let op = match op {
                    hir::UnOp::Not => UnOp::Not,
                    hir::UnOp::Neg => UnOp::Neg(expr_ty(&rhs, symbols).try_into().unwrap()),
                };

                Expr::Unary { op, rhs }
            }

            hir::Expr::Swizzle { lhs, swizzle } => {
                let lhs = Box::new(lhs.0.typed(symbols));

                Expr::Swizzle { lhs, swizzle }
            }

            hir::Expr::Call { fun, args } => {
                let args = args.into_iter().map(|expr| expr.0.typed(symbols)).collect();

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
            &BinOp::Vec(_, ty) => ty.into(),
            BinOp::Rel(_, _) | BinOp::And | BinOp::Or => Type::Bool,
        },

        Expr::Unary { op, .. } => match op {
            &UnOp::Neg(ty) => ty.into(),
            UnOp::Not => Type::Bool,
        },

        Expr::Swizzle { swizzle, .. } => match swizzle.combination.len() {
            1 => Type::Real,
            2 => Type::Vec2,
            3 => Type::Vec3,
            4 => Type::Vec4,
            _ => unreachable!(),
        },

        &Expr::Call { fun: id, .. } => symbols.functions[id].ret_ty.unwrap(),

        Expr::Atom(Atom::Literal(lit)) => lit.ty.into(),

        &Expr::Atom(Atom::Var(id)) => symbols.variables[id].ty,
    }
}

fn bin_op_typed(op: hir::BinOp, lhs: Type, rhs: Type) -> BinOp {
    if let Ok(op) = ArithmOp::try_from(op) {
        if let Ok(ty) = VecType::try_from(lhs).or(VecType::try_from(rhs)) {
            let op = match op {
                ArithmOp::Add => VecOp::Add,
                ArithmOp::Sub => VecOp::Sub,
                ArithmOp::Mul if lhs == Type::Real => VecOp::LeftMul,
                ArithmOp::Mul if rhs == Type::Real => VecOp::RightMul,
                ArithmOp::Div => VecOp::Div,

                _ => unreachable!(),
            };

            return BinOp::Vec(op, ty);
        } else {
            if lhs == Type::Complex || rhs == Type::Complex {
                return BinOp::Arithm(op, NumberType::Complex);
            }
            return BinOp::Arithm(op, lhs.try_into().unwrap());
        }
    }

    if let Ok(op) = RelOp::try_from(op) {
        return BinOp::Rel(op, lhs.try_into().unwrap());
    }

    match op {
        hir::BinOp::Or => BinOp::Or,
        hir::BinOp::And => BinOp::And,

        _ => unreachable!(""),
    }
}
