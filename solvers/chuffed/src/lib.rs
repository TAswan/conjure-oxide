pub mod bindings {
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
    include!(concat!(env!("OUT_DIR"), "/chuffed_bindings.rs"));
}


