use crate::{AssignRhs, Call, Id, Instruction, Instructions, Operand};

pub trait ContainsVar {
    fn contains(&self, sought_id: &Id) -> bool;
}

impl<T: YieldVariables> ContainsVar for T {
    fn contains(&self, sought_id: &Id) -> bool {
        self.yield_vars(&mut |id| {
            if id == sought_id {
                Err(())
            } else {
                Ok(())
            }
        })
        .is_ok()
    }
}

pub trait YieldAllVariables {
    fn yield_all_vars(&self, yield_fn: &mut impl FnMut(&Id));
}

impl<T: YieldVariables> YieldAllVariables for T {
    fn yield_all_vars(&self, yield_fn: &mut impl FnMut(&Id)) {
        <Self as YieldVariables>::yield_vars(self, &mut |id| {
            yield_fn(id);

            Ok(())
        })
        .unwrap();
    }
}

pub trait YieldVariables {
    fn yield_vars(&self, yield_fn: &mut impl FnMut(&Id) -> Result<(), ()>) -> Result<(), ()>;
}

impl YieldVariables for Instructions {
    fn yield_vars(&self, yield_fn: &mut impl FnMut(&Id) -> Result<(), ()>) -> Result<(), ()> {
        for instr in self.iter() {
            instr.yield_vars(yield_fn)?;
        }

        Ok(())
    }
}

impl YieldVariables for Instruction {
    fn yield_vars(&self, yield_fn: &mut impl FnMut(&Id) -> Result<(), ()>) -> Result<(), ()> {
        match self {
            Instruction::Assign { res, rhs } => {
                yield_fn(res)?;
                rhs.yield_vars(yield_fn)?;
            }

            Instruction::IfRel { lhs, rhs, .. } => {
                lhs.yield_vars(yield_fn)?;
                rhs.yield_vars(yield_fn)?;
            }

            Instruction::Call(call) => {
                call.yield_vars(yield_fn)?;
            }

            Instruction::Ret(Some(op)) => {
                op.yield_vars(yield_fn)?;
            }

            _ => {}
        }

        Ok(())
    }
}

impl YieldVariables for AssignRhs {
    fn yield_vars(&self, yield_fn: &mut impl FnMut(&Id) -> Result<(), ()>) -> Result<(), ()> {
        match self {
            AssignRhs::Arithm { lhs, rhs, .. } => {
                lhs.yield_vars(yield_fn)?;
                rhs.yield_vars(yield_fn)?;
            }

            AssignRhs::Neg { rhs, .. } => {
                rhs.yield_vars(yield_fn)?;
            }

            AssignRhs::Call(call) => {
                call.yield_vars(yield_fn)?;
            }

            AssignRhs::Operand(op) => {
                op.yield_vars(yield_fn)?;
            }
        }

        Ok(())
    }
}

impl YieldVariables for Call {
    fn yield_vars(&self, yield_fn: &mut impl FnMut(&Id) -> Result<(), ()>) -> Result<(), ()> {
        for op in self.args.iter() {
            op.yield_vars(yield_fn)?;
        }

        Ok(())
    }
}

impl YieldVariables for Operand {
    fn yield_vars(&self, yield_fn: &mut impl FnMut(&Id) -> Result<(), ()>) -> Result<(), ()> {
        if let Operand::Id(id) = self {
            yield_fn(id)?;
        }

        Ok(())
    }
}
