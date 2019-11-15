// cargo run --example usage
// >> Nothing
// cargo run --example usage -- u8_type
// cargo run --example usage -- str_type
// cargo run --example usage -- references_simple
// cargo run --example usage -- ref_and
// cargo run --example usage -- string_len_count
// cargo run --example usage -- closure
// cargo run --example usage -- raw_pointer_str
// cargo run --example usage -- raw_pointer_string
// cargo run --example usage -- raw_pointer
// cargo run --example usage -- clone_array
// cargo run --example usage -- kw_fn_u8_ref
// cargo run --example usage -- kw_fn_u8
// cargo run --example usage -- string_mut_new
// cargo run --example usage -- vec_mut_new
// cargo run --example usage -- convert_string_str
// cargo run --example usage -- prettytable

use std::env::args;

use hello_borrowing::immut::*;
use hello_borrowing::mutable::*;
use hello_borrowing::other::*;

fn main() {
    match args().nth(1) {
        Some(ref x) => {
            println!("Enter mod_name: {}", x);
            match x.as_str() {
                "u8_type" => type_ref::use_u8_type(),
                "str_type" => type_ref::use_str_type(),
                "references_simple" => type_ref::use_references_simple(),
                "ref_and" => type_ref::use_ref_and(),
                "string_len_count" => len_count::use_string_len_count(),
                "closure" => closure::use_closure(),
                "raw_pointer_str" => raw_pointer::use_raw_pointer_str(),
                "raw_pointer_string" => raw_pointer::use_raw_pointer_string(),
                "raw_pointer" => raw_pointer::use_raw_pointer(),
                "clone_array" => clone::use_clone_array(),
                "kw_fn_u8_ref" => kw_fn::use_kw_fn_u8_ref(),
                "kw_fn_u8" => kw_fn::use_kw_fn_u8(),
                "string_mut_new" => mut_new::use_string_mut_new(),
                "vec_mut_new" => mut_new::use_vec_mut_new(),
                "convert_string_str" => string_str::use_convert_string_str(),
                "prettytable" => crate_tools::use_prettytable(),
                _ => println!("No mod_name"),
            };
        },
        None        => println!("Nothing")
    }
}