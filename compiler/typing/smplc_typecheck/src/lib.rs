use smplc_hir::HIR;
use smplc_thir::{Constant, Function, THIR};

use error::TypeError;
use infer::{infer_expr, TypeInfer, TypeInferrer};
use type_var::TypeVar;
use typed::Typed;

pub mod error;

mod infer;
mod type_var;
mod typed;

pub fn typecheck(hir: HIR) -> Result<THIR, Vec<TypeError>> {
    let mut inferrer = TypeInferrer::default();

    for function in &hir.functions {
        inferrer.current_fn = Some(function.id);

        function
            .body
            .infer(&mut inferrer, &hir.symbols)
            .map_err(|x| vec![x])?;
    }

    for constant in &hir.constants {
        let inference =
            infer_expr(&constant.value, &mut inferrer, &hir.symbols).map_err(|x| vec![x])?;

        TypeVar::max(TypeVar::Type(constant.ty), inference.ty)
            .map_err(|(required, got)| {
                TypeError::mismatched_types(required, got, constant.value.span())
            })
            .map_err(|err| vec![err])?;
    }

    let symbols = inferrer.infer(hir.symbols)?;

    let functions = hir
        .functions
        .into_iter()
        .map(|function| Function {
            id: function.id,
            args: function.args,
            body: function.body.typed(&symbols),
        })
        .collect();

    let constants = hir
        .constants
        .into_iter()
        .map(|constant| Constant {
            id: constant.id,
            ty: constant.ty,
            value: constant.value.0.typed(&symbols),
        })
        .collect();

    Ok(THIR {
        symbols,
        functions,
        constants,
    })
}
