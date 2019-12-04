mod closure_immut_string;
mod closure_kw_move;
pub mod closure_move_vec;
mod closure_mut_move;
mod dbg_mut_macro;
mod expand_struct_str;
mod expand_struct_string;
mod expand_struct_u8;
mod for_loop_for_arr;
mod for_loop_for_enumerate;
mod for_loop_for_into_iter;
mod for_loop_for_iter_mut;
mod for_loop_for_next;
mod for_loop_for_vec;
mod for_loop_for_vec_iter;
mod kw_fn_vec_str;
mod kw_fn_vec_u8;
mod mut_base_mut_count;
mod mut_fn_base_str;
mod mut_fn_base_string;
mod mut_fn_double_refs;
mod mut_immut_mut_string;
mod mut_immut_ref_str;
mod mut_var_sized_string_ref;
mod mut_var_sized_string_refs;
mod mut_var_sized_struct_ref;
mod string_type_string_str;
//mod
                                                                                                                                                                                                                                                                                                                                                                                                                                                        
pub const HELLO: &str = r#"fn main() { println!("Hello, World!") }"#;

pub const FEATURE_MODE :[&str; 14] = [ 
    "ok", "cp", "okay", "okey", 
    "err_01", "err_02", "err_03", "err_04", "err_05",
    "err_06", "err_07", "err_08", "err_09", "err_10"
];

pub fn get_rs(file_name: &str) -> &str {
    match file_name.as_ref() {
        "hello" => HELLO,
        "closure_immut_string_ok" => closure_immut_string::IMMUT_STRING_OK,
        "closure_kw_move_ok" => closure_kw_move::KW_MOVE_OK,
        "closure_move_vec_ok" => closure_move_vec::MOVE_VEC_OK,
        "closure_mut_move_ok" => closure_mut_move::MUT_MOVE_OK,
        "dbg_mut_macro_ok" => dbg_mut_macro::MUT_MACRO_OK,
        "expand_struct_str_ok" => expand_struct_str::STRUCT_STR_OK,
        "expand_struct_string_ok" => expand_struct_string::STRUCT_STRING_OK,
        "expand_struct_u8_ok" => expand_struct_u8::STRUCT_U8_OK,
        "for_loop_for_arr_ok" => for_loop_for_arr::FOR_ARR_OK,
        "for_loop_for_enumerate_ok" => for_loop_for_enumerate::FOR_ENUMERATE_OK,
        "for_loop_for_into_iter_ok" => for_loop_for_into_iter::FOR_INTO_ITER_OK,
        "for_loop_for_iter_mut_ok" => for_loop_for_iter_mut::FOR_ITER_MUT_OK,
        "for_loop_for_next_ok" => for_loop_for_next::FOR_NEXT_OK,
        "for_loop_for_vec_ok" => for_loop_for_vec::FOR_VEC_OK,
        "for_loop_for_vec_iter_ok" => for_loop_for_vec_iter::FOR_VEC_ITER_OK,
        "kw_fn_vec_str_ok" => kw_fn_vec_str::VEC_STR_OK,
        "kw_fn_vec_u8_ok" => kw_fn_vec_u8::VEC_U8_OK,
        "mut_base_mut_count_ok" => mut_base_mut_count::MUT_COUNT_OK,
        "mut_fn_base_str_ok" => mut_fn_base_str::BASE_STR_OK,
        "mut_fn_base_string_ok" => mut_fn_base_string::BASE_STRING_OK,
        "mut_fn_double_refs_ok" => mut_fn_double_refs::DOUBLE_REFS_OK,
        "mut_immut_mut_string_ok" => mut_immut_mut_string::MUT_STRING_OK,
        "mut_immut_ref_str_ok" => mut_immut_ref_str::REF_STR_OK,
        "mut_var_sized_string_ref_ok" => mut_var_sized_string_ref::STRING_REF_OK,
        "mut_var_sized_string_refs_ok" => mut_var_sized_string_refs::STRING_REFS_OK,
        "mut_var_sized_struct_ref_ok" => mut_var_sized_struct_ref::STRUCT_REF_OK,
        "string_type_string_str_ok" => string_type_string_str::STRING_STR_OK,
        //ok
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                
        //err

        "closure_kw_move_cp" => closure_kw_move::KW_MOVE_CP,
        "dbg_mut_macro_cp" => dbg_mut_macro::MUT_MACRO_CP,
        "expand_struct_str_cp" => expand_struct_str::STRUCT_STR_CP,
        "expand_struct_u8_cp" => expand_struct_u8::STRUCT_U8_CP,
        "for_loop_for_enumerate_cp" => for_loop_for_enumerate::FOR_ENUMERATE_CP,
        "for_loop_for_into_iter_cp" => for_loop_for_into_iter::FOR_INTO_ITER_CP,
        "for_loop_for_iter_mut_cp" => for_loop_for_iter_mut::FOR_ITER_MUT_CP,
        "for_loop_for_next_cp" => for_loop_for_next::FOR_NEXT_CP,
        "for_loop_for_vec_cp" => for_loop_for_vec::FOR_VEC_CP,
        "for_loop_for_vec_iter_cp" => for_loop_for_vec_iter::FOR_VEC_ITER_CP,
        "kw_fn_vec_u8_cp" => kw_fn_vec_u8::VEC_U8_CP,
        "mut_fn_base_str_cp" => mut_fn_base_str::BASE_STR_CP,
        "mut_fn_base_string_cp" => mut_fn_base_string::BASE_STRING_CP,
        "mut_immut_mut_string_cp" => mut_immut_mut_string::MUT_STRING_CP,
        "mut_immut_ref_str_cp" => mut_immut_ref_str::REF_STR_CP,
        "mut_var_sized_string_ref_cp" => mut_var_sized_string_ref::STRING_REF_CP,
        //cp
                                                                                                                                                                                                                                                                
        "for_loop_for_enumerate_okey" => for_loop_for_enumerate::FOR_ENUMERATE_OKEY,
        "for_loop_for_into_iter_okey" => for_loop_for_into_iter::FOR_INTO_ITER_OKEY,
        "for_loop_for_iter_mut_okey" => for_loop_for_iter_mut::FOR_ITER_MUT_OKEY,
        "for_loop_for_next_okey" => for_loop_for_next::FOR_NEXT_OKEY,
        "mut_fn_base_str_okey" => mut_fn_base_str::BASE_STR_OKEY,
        "mut_fn_base_string_okey" => mut_fn_base_string::BASE_STRING_OKEY,
        "mut_immut_mut_string_okey" => mut_immut_mut_string::MUT_STRING_OKEY,
        "mut_immut_ref_str_okey" => mut_immut_ref_str::REF_STR_OKEY,
        //okey
                                                                                                                                
        "closure_mut_move_okay" => closure_mut_move::MUT_MOVE_OKAY,
        "for_loop_for_enumerate_okay" => for_loop_for_enumerate::FOR_ENUMERATE_OKAY,
        "for_loop_for_into_iter_okay" => for_loop_for_into_iter::FOR_INTO_ITER_OKAY,
        "for_loop_for_iter_mut_okay" => for_loop_for_iter_mut::FOR_ITER_MUT_OKAY,
        "for_loop_for_next_okay" => for_loop_for_next::FOR_NEXT_OKAY,
        "mut_fn_base_str_okay" => mut_fn_base_str::BASE_STR_OKAY,
        "mut_fn_base_string_okay" => mut_fn_base_string::BASE_STRING_OKAY,
        "mut_immut_mut_string_okay" => mut_immut_mut_string::MUT_STRING_OKAY,
        "mut_immut_ref_str_okay" => mut_immut_ref_str::REF_STR_OKAY,
        //okay
                                                                                                                                                
        "closure_immut_string_err_01" => closure_immut_string::IMMUT_STRING_ERR_01,
        "dbg_mut_macro_err_01" => dbg_mut_macro::MUT_MACRO_ERR_01,
        "expand_struct_u8_err_01" => expand_struct_u8::STRUCT_U8_ERR_01,
        "for_loop_for_arr_err_01" => for_loop_for_arr::FOR_ARR_ERR_01,
        "kw_fn_vec_u8_err_01" => kw_fn_vec_u8::VEC_U8_ERR_01,
        "mut_base_mut_count_err_01" => mut_base_mut_count::MUT_COUNT_ERR_01,
        "mut_fn_double_refs_err_01" => mut_fn_double_refs::DOUBLE_REFS_ERR_01,
        "mut_immut_mut_string_err_01" => mut_immut_mut_string::MUT_STRING_ERR_01,
        "mut_immut_ref_str_err_01" => mut_immut_ref_str::REF_STR_ERR_01,
        "mut_var_sized_string_ref_err_01" => mut_var_sized_string_ref::STRING_REF_ERR_01,
        "string_type_string_str_err_01" => string_type_string_str::STRING_STR_ERR_01,
        //err_01
                                                                                                                                                                                
        "closure_kw_move_err_02" => closure_kw_move::KW_MOVE_ERR_02,
        "dbg_mut_macro_err_02" => dbg_mut_macro::MUT_MACRO_ERR_02,
        "expand_struct_u8_err_02" => expand_struct_u8::STRUCT_U8_ERR_02,
        "for_loop_for_vec_err_02" => for_loop_for_vec::FOR_VEC_ERR_02,
        "kw_fn_vec_str_err_02" => kw_fn_vec_str::VEC_STR_ERR_02,
        "mut_base_mut_count_err_02" => mut_base_mut_count::MUT_COUNT_ERR_02,
        "mut_fn_double_refs_err_02" => mut_fn_double_refs::DOUBLE_REFS_ERR_02,
        "mut_immut_mut_string_err_02" => mut_immut_mut_string::MUT_STRING_ERR_02,
        "mut_immut_ref_str_err_02" => mut_immut_ref_str::REF_STR_ERR_02,
        "mut_var_sized_string_ref_err_02" => mut_var_sized_string_ref::STRING_REF_ERR_02,
        //err_02
                                                                                                                                                                
        "closure_move_vec_err_03" => closure_move_vec::MOVE_VEC_ERR_03,
        "dbg_mut_macro_err_03" => dbg_mut_macro::MUT_MACRO_ERR_03,
        "expand_struct_u8_err_03" => expand_struct_u8::STRUCT_U8_ERR_03,
        "for_loop_for_vec_iter_err_03" => for_loop_for_vec_iter::FOR_VEC_ITER_ERR_03,
        "mut_base_mut_count_err_03" => mut_base_mut_count::MUT_COUNT_ERR_03,
        "mut_immut_mut_string_err_03" => mut_immut_mut_string::MUT_STRING_ERR_03,
        "mut_immut_ref_str_err_03" => mut_immut_ref_str::REF_STR_ERR_03,
        "mut_var_sized_string_ref_err_03" => mut_var_sized_string_ref::STRING_REF_ERR_03,
        //err_03
                                                                                                                                                        
        "closure_move_vec_err_04" => closure_move_vec::MOVE_VEC_ERR_04,
        "dbg_mut_macro_err_04" => dbg_mut_macro::MUT_MACRO_ERR_04,
        "expand_struct_string_err_04" => expand_struct_string::STRUCT_STRING_ERR_04,
        "for_loop_for_iter_mut_err_04" => for_loop_for_iter_mut::FOR_ITER_MUT_ERR_04,
        "mut_fn_base_string_err_04" => mut_fn_base_string::BASE_STRING_ERR_04,
        "mut_immut_mut_string_err_04" => mut_immut_mut_string::MUT_STRING_ERR_04,
        "mut_var_sized_string_refs_err_04" => mut_var_sized_string_refs::STRING_REFS_ERR_04,
        //err_04
                                                                                                                                        
        "closure_mut_move_err_05" => closure_mut_move::MUT_MOVE_ERR_05,
        "expand_struct_string_err_05" => expand_struct_string::STRUCT_STRING_ERR_05,
        "for_loop_for_iter_mut_err_05" => for_loop_for_iter_mut::FOR_ITER_MUT_ERR_05,
        "mut_fn_base_string_err_05" => mut_fn_base_string::BASE_STRING_ERR_05,
        "mut_immut_mut_string_err_05" => mut_immut_mut_string::MUT_STRING_ERR_05,
        "mut_var_sized_string_refs_err_05" => mut_var_sized_string_refs::STRING_REFS_ERR_05,
        //err_05
                                                                                                
        "expand_struct_string_err_06" => expand_struct_string::STRUCT_STRING_ERR_06,
        "for_loop_for_iter_mut_err_06" => for_loop_for_iter_mut::FOR_ITER_MUT_ERR_06,
        "mut_fn_base_string_err_06" => mut_fn_base_string::BASE_STRING_ERR_06,
        "mut_immut_mut_string_err_06" => mut_immut_mut_string::MUT_STRING_ERR_06,
        "mut_var_sized_string_refs_err_06" => mut_var_sized_string_refs::STRING_REFS_ERR_06,
        //err_06
                                                                                
        "expand_struct_str_err_07" => expand_struct_str::STRUCT_STR_ERR_07,
        "for_loop_for_next_err_07" => for_loop_for_next::FOR_NEXT_ERR_07,
        "mut_fn_base_str_err_07" => mut_fn_base_str::BASE_STR_ERR_07,
        //err_07
                                                
        "expand_struct_str_err_08" => expand_struct_str::STRUCT_STR_ERR_08,
        "for_loop_for_next_err_08" => for_loop_for_next::FOR_NEXT_ERR_08,
        "mut_fn_base_str_err_08" => mut_fn_base_str::BASE_STR_ERR_08,
        "mut_var_sized_struct_ref_err_08" => mut_var_sized_struct_ref::STRUCT_REF_ERR_08,
        //err_08
                                                                
        "expand_struct_str_err_09" => expand_struct_str::STRUCT_STR_ERR_09,
        "for_loop_for_into_iter_err_09" => for_loop_for_into_iter::FOR_INTO_ITER_ERR_09,
        "mut_fn_base_str_err_09" => mut_fn_base_str::BASE_STR_ERR_09,
        //err_09
                                                
        "expand_struct_u8_err_10" => expand_struct_u8::STRUCT_U8_ERR_10,
        "mut_fn_base_str_err_10" => mut_fn_base_str::BASE_STR_ERR_10,
        //err_10
                                
        _ => HELLO,
    }
}
