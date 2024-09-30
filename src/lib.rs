#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use crate::{spral_matrix_type, spral_matrix_type_SPRAL_MATRIX_REAL_UNSYM};

    #[test]
    fn test_spral() {
        let _t: spral_matrix_type = spral_matrix_type_SPRAL_MATRIX_REAL_UNSYM;
    }
}
