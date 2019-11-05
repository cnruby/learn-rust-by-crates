mod clone_derive;
mod clone_struct;
mod closure_string;
mod closure_vec;
mod dbg_marco;
mod expand_struct;
mod kw_fn;
mod kw_mut;
mod kw_mut_fn;
mod move_tuple;
mod move_vec;
mod new_var;
mod stack_head;
mod string_mut_type;
mod string_ref_thread;
mod string_thread;
mod string_type;
mod string_type_str;
mod vec_for;
/*mod*/

pub const HELLO: &str = r#"fn main() { println!("Hello, World!") }"#;

pub fn get_rs_ok(file_name: &str) -> &str {
    match file_name.as_ref() {
        "kw_fn" => kw_fn::KW_FN_OK,
        "kw_mut" => kw_mut::KW_MUT_OK,
        "string_mut_type" => string_mut_type::STRING_MUT_TYPE_OK,
        "clone_derive" => clone_derive::CLONE_DERIVE_OK,
        "string_ref_thread" => string_ref_thread::STRING_REF_THREAD_OK,
        "clone_struct" => clone_struct::CLONE_STRUCT_OK,
        "kw_mut_fn" => kw_mut_fn::KW_MUT_FN_OK,
        "string_thread" => string_thread::STRING_THREAD_OK,
        "closure_string" => closure_string::CLOSURE_STRING_OK,
        "move_tuple" => move_tuple::MOVE_TUPLE_OK,
        "string_type" => string_type::STRING_TYPE_OK,
        "string_type_str" => string_type_str::STRING_TYPE_STR_OK,
        "dbg_marco" => dbg_marco::DBG_MARCO_OK,
        "move_vec" => move_vec::MOVE_VEC_OK,
        "expand_struct" => expand_struct::EXPAND_STRUCT_OK,
        "new_var" => new_var::NEW_VAR_OK,
        "stack_head" => stack_head::STACK_HEAD_OK,
        "vec_for" => vec_for::VEC_FOR_OK,
        "closure_vec" => closure_vec::CLOSURE_VEC_OK,
        /*ok*/
        _ => HELLO,
    }
}

pub fn get_rs_err(file_name: &str) -> &str {
    match file_name.as_ref() {
        "kw_fn" => kw_fn::KW_FN_ERR,
        "kw_mut" => kw_mut::KW_MUT_ERR,
        "string_mut_type" => string_mut_type::STRING_MUT_TYPE_ERR,
        "clone_derive" => clone_derive::CLONE_DERIVE_ERR,
        "string_ref_thread" => string_ref_thread::STRING_REF_THREAD_ERR,
        "clone_struct" => clone_struct::CLONE_STRUCT_ERR,
        "kw_mut_fn" => kw_mut_fn::KW_MUT_FN_ERR,
        "string_thread" => string_thread::STRING_THREAD_ERR,
        "closure_string" => closure_string::CLOSURE_STRING_ERR,
        "move_tuple" => move_tuple::MOVE_TUPLE_ERR,
        "string_type" => string_type::STRING_TYPE_ERR,
        "string_type_str" => string_type_str::STRING_TYPE_STR_ERR,
        "dbg_marco" => dbg_marco::DBG_MARCO_ERR,
        "move_vec" => move_vec::MOVE_VEC_ERR,
        "expand_struct" => expand_struct::EXPAND_STRUCT_ERR,
        "new_var" => new_var::NEW_VAR_ERR,
        "stack_head" => stack_head::STACK_HEAD_ERR,
        "vec_for" => vec_for::VEC_FOR_ERR,
        "closure_vec" => closure_vec::CLOSURE_VEC_ERR,
        /*err*/
        _ => HELLO,
    }
}
