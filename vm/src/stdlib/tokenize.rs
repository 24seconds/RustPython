/*
 * python tokenize module.
 */

extern crate rustpython_parser;
use std::iter::FromIterator;

use self::rustpython_parser::lexer;

use crate::obj::objstr;
use crate::pyobject::{PyContext, PyFuncArgs, PyObjectRef, PyResult, TypeProtocol};
use crate::VirtualMachine;

fn tokenize_tokenize(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    arg_check!(vm, args, required = [(readline, Some(vm.ctx.str_type()))]);
    let source = objstr::get_value(readline);

    // TODO: implement generator when the time has come.
    let lexer1 = lexer::make_tokenizer(&source);

    let tokens = lexer1.map(|st| vm.ctx.new_str(format!("{:?}", st.unwrap().1)));
    let tokens = Vec::from_iter(tokens);
    Ok(vm.ctx.new_list(tokens))
}

// TODO: create main function when called with -m

pub fn mk_module(ctx: &PyContext) -> PyObjectRef {
    py_module!(ctx, "tokenize", {
        "tokenize" => ctx.new_rustfunc(tokenize_tokenize)
    })
}
