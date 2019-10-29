mod kw_fn;
mod kw_mut;
mod kw_let_mut;
mod string_mut_type;
mod clone_derive;
mod string_ref_thread;
mod clone_struct;
mod kw_mut_fn;
mod string_thread;
mod closure_type;
mod move_tuple;
mod string_type;
mod dbg_marco;
mod move_vec;
mod expand_struct;
mod new_var;
mod stack_head;
mod vec_for;
/*
*/

pub const HELLO :&str = r#"fn main() { println!("Hello, World!") }"#;

pub fn get_rs_ok(file_name: &str) -> &str {
    match file_name.as_ref() {
        "kw_fn" => {
            kw_fn::KW_FN_OK
        },
        "kw_mut" => {
            kw_mut::KW_MUT_OK
        },
        "kw_let_mut" => {
            kw_let_mut::KW_LET_MUT_OK
        },
        "string_mut_type" => {
            string_mut_type::STRING_MUT_TYPE_OK
        },
        "clone_derive" => {
            clone_derive::CLONE_DERIVE_OK
        },
        "string_ref_thread" => {
            string_ref_thread::STRING_REF_THREAD_OK
        },
        "clone_struct" => {
            clone_struct::CLONE_STRUCT_OK
        },
        "kw_mut_fn" => {
            kw_mut_fn::KW_MUT_FN_OK
        },
        "string_thread" => {
            string_thread::STRING_THREAD_OK
        },
        "closure_type" => {
            closure_type::CLOSURE_TYPE_OK
        },
        "move_tuple" => {
            move_tuple::MOVE_TUPLE_OK
        },
        "string_type" => {
            string_type::STRING_TYPE_OK
        },
        "dbg_marco" => {
            dbg_marco::DBG_MARCO_OK
        },
        "move_vec" => {
            move_vec::MOVE_VEC_OK
        },
        "expand_struct" => {
            expand_struct::EXPAND_STRUCT_OK
        },
        "new_var" => {
            new_var::NEW_VAR_OK
        },
        "stack_head" => {
            stack_head::STACK_HEAD_OK
        },
        "vec_for" => {
            vec_for::VEC_FOR_OK
        },
        /*
        */
        _ => {
            HELLO
        },
    }
}

pub fn get_rs_err(file_name: &str) -> &str {
    match file_name.as_ref() {
        "kw_fn" => {
            kw_fn::KW_FN_ERR
        },
        "kw_mut" => {
            kw_mut::KW_MUT_ERR
        },
        "kw_let_mut" => {
            kw_let_mut::KW_LET_MUT_ERR
        },
        "string_mut_type" => {
            string_mut_type::STRING_MUT_TYPE_ERR
        },
        "clone_derive" => {
            clone_derive::CLONE_DERIVE_ERR
        },
        "string_ref_thread" => {
            string_ref_thread::STRING_REF_THREAD_ERR
        },
        "clone_struct" => {
            clone_struct::CLONE_STRUCT_ERR
        },
        "kw_mut_fn" => {
            kw_mut_fn::KW_MUT_FN_ERR
        },
        "string_thread" => {
            string_thread::STRING_THREAD_ERR
        },
        "closure_type" => {
            closure_type::CLOSURE_TYPE_ERR
        },
        "move_tuple" => {
            move_tuple::MOVE_TUPLE_ERR
        },
        "string_type" => {
            string_type::STRING_TYPE_ERR
        },
        "dbg_marco" => {
            dbg_marco::DBG_MARCO_ERR
        },
        "move_vec" => {
            move_vec::MOVE_VEC_ERR
        },
        "expand_struct" => {
            expand_struct::EXPAND_STRUCT_ERR
        },
        "new_var" => {
            new_var::NEW_VAR_ERR
        },
        "stack_head" => {
            stack_head::STACK_HEAD_ERR
        },
        "vec_for" => {
            vec_for::VEC_FOR_ERR
        },
        /*
        */
        _ => {
            HELLO
        },
    }
}

