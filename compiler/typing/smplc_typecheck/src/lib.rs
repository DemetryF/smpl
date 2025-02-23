use infer::{infer_expr, TypeInfer, TypeInferrer};
use smplc_hir::HIR;
use smplc_thir::{Constant, Function, THIR};

use error::TypeError;
use typed::Typed;

pub mod error;

mod infer;
mod typed;

pub fn typecheck<'source>(hir: HIR<'source>) -> Result<THIR<'source>, Vec<TypeError<'source>>> {
    let mut inferrer = TypeInferrer::default();

    for function in &hir.functions {
        inferrer.current_fn = Some(function.id);

        function
            .body
            .infer(&mut inferrer, &hir.symbols)
            .map_err(|x| vec![x])?;
    }

    for constant in &hir.constants {
        infer_expr(&constant.value, &mut inferrer, &hir.symbols).map_err(|x| vec![x])?;
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
            value: constant.value.typed(&symbols),
        })
        .collect();

    Ok(THIR {
        symbols,
        functions,
        constants,
    })
}
