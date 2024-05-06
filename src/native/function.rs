use crate::interpreter::context::Context;
use crate::interpreter::error::InterpreterError;
use crate::native::error::NativeFnError;
use crate::primitives::any::Any;

/// Functions that get executed natively by the interpreter. Functions will receive the whole call
/// tree, where the node is the called function, and the children are its arguments
pub type NativeFn = for<'a, 'b, 'c, 'data>
    fn(&'a mut Context<'b, 'data>, &'c [Any<'data>]) -> Result<Any<'data>, InterpreterError>;

pub struct NativeFunction {
    fun: NativeFn
}

impl NativeFunction {
    pub fn new(fun: NativeFn) -> Self {
        NativeFunction {
            fun
        }
    }

    pub fn call<'a>(&self, cx: &mut Context<'_, 'a>, args: &[Any<'a>]) -> Result<Any<'a>, InterpreterError> {
        (self.fun)(cx, args)
    }
}
