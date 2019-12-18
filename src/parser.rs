// auto-generated: "lalrpop 0.15.2"
// sha256: 76dc68435d2af6efadfeccaeaa2ebaba1bb7f4ccafb030693deb819af95433
use std::str::FromStr;
use ast::{Expr, Operator, Value, Identifier};
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Expr {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use std::str::FromStr;
    use ast::{Expr, Operator, Value, Identifier};
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    use super::__intern_token::Token;
    #[allow(dead_code)]
    pub enum __Symbol<'input>
     {
        Variant0(&'input str),
        Variant1(Box<Expr>),
        Variant2(Operator),
        Variant3(Expr),
        Variant4(::std::vec::Vec<Expr>),
        Variant5(bool),
        Variant6(f64),
        Variant7(Identifier),
        Variant8(::std::vec::Vec<Identifier>),
        Variant9(i32),
    }
    const __ACTION: &'static [i8] = &[
        // State 0
        12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 14, 15, 16, 17, 18,
        // State 1
        0, 0, 0, 0, 20, 21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, -4, 0, 23, -4, -4, 24, -4, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, -46, 0, -46, -46, -46, -46, -46, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, -43, 0, -43, -43, -43, -43, -43, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        25, -28, 0, -28, -28, -28, -28, -28, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, -47, 0, -47, -47, -47, -47, -47, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, -44, 0, -44, -44, -44, -44, -44, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, -5, 0, 0, -5, -5, 0, -5, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, -18, 0, -18, -18, -18, -18, -18, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, -45, 0, -45, -45, -45, -45, -45, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        12, 0, 29, 0, 0, 0, 0, 0, 0, 0, 13, 0, 14, 15, 16, 17, 18,
        // State 12
        0, -21, 0, -21, -21, -21, -21, -21, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, -49, 0, -49, -49, -49, -49, -49, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, -36, 0, 0, -36, -36, 0, -36, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, -34, 0, -34, -34, -34, -34, -34, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        -23, -23, -23, -23, -23, -23, -23, -23, -23, 0, 0, 0, 0, 0, 0, -23, 0,
        // State 17
        0, -22, 0, -22, -22, -22, -22, -22, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 14, 0, 16, 17, 18,
        // State 19
        -6, 0, 0, 0, 0, 0, 0, 0, 0, 0, -6, 0, -6, 0, -6, -6, -6,
        // State 20
        -7, 0, 0, 0, 0, 0, 0, 0, 0, 0, -7, 0, -7, 0, -7, -7, -7,
        // State 21
        12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 14, 0, 16, 17, 18,
        // State 22
        -19, 0, 0, 0, 0, 0, 0, 0, 0, 0, -19, 0, -19, 0, -19, -19, -19,
        // State 23
        -20, 0, 0, 0, 0, 0, 0, 0, 0, 0, -20, 0, -20, 0, -20, -20, -20,
        // State 24
        0, 35, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 17, 0,
        // State 25
        0, 36, 0, 0, 20, 21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        25, -28, -26, -28, -28, -28, -28, 0, 0, 0, 0, 0, 0, 0, 0, -26, 0,
        // State 27
        0, 0, 38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 17, 0,
        // State 28
        12, -39, 0, -39, -39, -39, -39, -39, 0, 47, 13, 48, 14, 15, 16, 17, 18,
        // State 29
        0, -3, 0, 23, -3, -3, 24, -3, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, -17, 0, -17, -17, -17, -17, -17, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, -29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -29, 0,
        // State 32
        0, -32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -32, 0,
        // State 33
        0, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 17, 0,
        // State 34
        0, -37, 0, -37, -37, -37, -37, -37, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, -48, 0, -48, -48, -48, -48, -48, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, -27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -27, 0,
        // State 37
        12, -40, 0, -40, -40, -40, -40, -40, 0, 47, 13, 48, 14, 15, 16, 17, 18,
        // State 38
        0, 0, 0, 0, 0, 0, 0, -11, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 20, 21, 0, -8, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 0, 0, 0, 52, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        -15, -15, 0, -15, -15, -15, -15, -15, 0, -15, -15, -15, -15, -15, -15, -15, -15,
        // State 43
        12, -41, 0, -41, -41, -41, -41, -41, 0, 47, 13, 48, 14, 15, 16, 17, 18,
        // State 44
        25, 0, 0, -28, -28, -28, -28, -28, 54, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, 0, 0, 0, 0, 0, 0, -9, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 17, 0,
        // State 47
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 17, 0,
        // State 48
        0, -33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -33, 0,
        // State 49
        0, -38, 0, -38, -38, -38, -38, -38, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        12, -42, 0, -42, -42, -42, -42, -42, 0, 47, 13, 48, 14, 15, 16, 17, 18,
        // State 51
        -12, -12, 0, -12, -12, -12, -12, -12, 0, -12, -12, -12, -12, -12, -12, -12, -12,
        // State 52
        -16, -16, 0, -16, -16, -16, -16, -16, 0, -16, -16, -16, -16, -16, -16, -16, -16,
        // State 53
        12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 14, 15, 16, 17, 18,
        // State 54
        0, 0, 0, 0, 0, 0, 0, 0, 58, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 55
        0, 0, 0, 0, 0, 0, 0, 0, 59, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 56
        0, 0, 0, 0, 20, 21, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 57
        12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 14, 15, 16, 17, 18,
        // State 58
        12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 14, 15, 16, 17, 18,
        // State 59
        0, 0, 0, 0, 20, 21, 0, -2, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 60
        0, 0, 0, 0, 20, 21, 0, -35, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    const __EOF_ACTION: &'static [i8] = &[
        // State 0
        0,
        // State 1
        -50,
        // State 2
        -4,
        // State 3
        -46,
        // State 4
        -43,
        // State 5
        -28,
        // State 6
        -47,
        // State 7
        -44,
        // State 8
        -5,
        // State 9
        -18,
        // State 10
        -45,
        // State 11
        0,
        // State 12
        -21,
        // State 13
        -49,
        // State 14
        -36,
        // State 15
        -34,
        // State 16
        -23,
        // State 17
        -22,
        // State 18
        0,
        // State 19
        0,
        // State 20
        0,
        // State 21
        0,
        // State 22
        0,
        // State 23
        0,
        // State 24
        0,
        // State 25
        0,
        // State 26
        0,
        // State 27
        0,
        // State 28
        -39,
        // State 29
        -3,
        // State 30
        -17,
        // State 31
        0,
        // State 32
        0,
        // State 33
        0,
        // State 34
        -37,
        // State 35
        -48,
        // State 36
        0,
        // State 37
        -40,
        // State 38
        0,
        // State 39
        0,
        // State 40
        0,
        // State 41
        0,
        // State 42
        -15,
        // State 43
        -41,
        // State 44
        0,
        // State 45
        0,
        // State 46
        0,
        // State 47
        0,
        // State 48
        0,
        // State 49
        -38,
        // State 50
        -42,
        // State 51
        -12,
        // State 52
        -16,
        // State 53
        0,
        // State 54
        0,
        // State 55
        0,
        // State 56
        0,
        // State 57
        0,
        // State 58
        0,
        // State 59
        0,
        // State 60
        0,
    ];
    const __GOTO: &'static [i8] = &[
        // State 0
        0, 0, 2, 0, 0, 0, 0, 0, 3, 0, 4, 5, 6, 0, 0, 7, 0, 0, 0, 8, 0, 9, 10, 11, 0, 0,
        // State 1
        0, 0, 0, 19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 26, 0, 0, 0, 0, 0, 3, 0, 4, 5, 27, 0, 28, 7, 0, 0, 0, 8, 0, 9, 10, 11, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 30, 0, 4, 5, 6, 0, 0, 7, 0, 0, 0, 8, 0, 0, 10, 11, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 5, 6, 0, 0, 7, 0, 0, 0, 8, 0, 0, 31, 11, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0, 33, 0, 34, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        39, 40, 41, 0, 42, 43, 0, 44, 3, 0, 4, 5, 45, 0, 0, 7, 0, 0, 0, 8, 46, 9, 10, 11, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0, 49, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        39, 40, 41, 0, 42, 43, 0, 51, 3, 0, 4, 5, 45, 0, 0, 7, 0, 0, 0, 8, 46, 9, 10, 11, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 43
        39, 40, 41, 0, 42, 53, 0, 0, 3, 0, 4, 5, 45, 0, 0, 7, 0, 0, 0, 8, 46, 9, 10, 11, 0, 0,
        // State 44
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 55, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 49
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        39, 40, 41, 0, 42, 53, 0, 0, 3, 0, 4, 5, 45, 0, 0, 7, 0, 0, 0, 8, 46, 9, 10, 11, 0, 0,
        // State 51
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 52
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 53
        0, 0, 57, 0, 0, 0, 0, 0, 3, 0, 4, 5, 6, 0, 0, 7, 0, 0, 0, 8, 0, 9, 10, 11, 0, 0,
        // State 54
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 55
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 56
        0, 0, 0, 19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 57
        0, 0, 60, 0, 0, 0, 0, 0, 3, 0, 4, 5, 6, 0, 0, 7, 0, 0, 0, 8, 0, 9, 10, 11, 0, 0,
        // State 58
        0, 0, 61, 0, 0, 0, 0, 0, 3, 0, 4, 5, 6, 0, 0, 7, 0, 0, 0, 8, 0, 9, 10, 11, 0, 0,
        // State 59
        0, 0, 0, 19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 60
        0, 0, 0, 19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""(""###,
            r###"")""###,
            r###"") => ""###,
            r###""*""###,
            r###""+""###,
            r###""-""###,
            r###""/""###,
            r###"";""###,
            r###""=""###,
            r###""const""###,
            r###""false""###,
            r###""let""###,
            r###""true""###,
            r###"r#"\"[^\"]+\""#"###,
            r###"r#"[0-9]+"#"###,
            r###"r#"[A-z]([A-z|0-9|_])*"#"###,
            r###"r#"\\d+\\.\\d+"#"###,
        ];
        __ACTION[(__state * 17)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub struct ExprParser {
        builder: super::__intern_token::__MatcherBuilder,
        _priv: (),
    }

    impl ExprParser {
        pub fn new() -> ExprParser {
            let __builder = super::__intern_token::__MatcherBuilder::new();
            ExprParser {
                builder: __builder,
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            'input,
        >(
            &self,
            input: &'input str,
        ) -> Result<Box<Expr>, __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>
        {
            let mut __tokens = self.builder.matcher(input);
            let mut __states = vec![0_i8];
            let mut __symbols = vec![];
            let mut __integer;
            let mut __lookahead;
            let __last_location = &mut Default::default();
            '__shift: loop {
                __lookahead = match __tokens.next() {
                    Some(Ok(v)) => v,
                    None => break '__shift,
                    Some(Err(e)) => return Err(e),
                };
                *__last_location = __lookahead.2.clone();
                __integer = match __lookahead.1 {
                    Token(4, _) if true => 0,
                    Token(5, _) if true => 1,
                    Token(6, _) if true => 2,
                    Token(7, _) if true => 3,
                    Token(8, _) if true => 4,
                    Token(9, _) if true => 5,
                    Token(10, _) if true => 6,
                    Token(11, _) if true => 7,
                    Token(12, _) if true => 8,
                    Token(13, _) if true => 9,
                    Token(14, _) if true => 10,
                    Token(15, _) if true => 11,
                    Token(16, _) if true => 12,
                    Token(0, _) if true => 13,
                    Token(1, _) if true => 14,
                    Token(2, _) if true => 15,
                    Token(3, _) if true => 16,
                    _ => {
                        let __state = *__states.last().unwrap() as usize;
                        let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                            token: Some(__lookahead),
                            expected: __expected_tokens(__state),
                        };
                        return Err(__error);
                    }
                };
                '__inner: loop {
                    let __state = *__states.last().unwrap() as usize;
                    let __action = __ACTION[__state * 17 + __integer];
                    if __action > 0 {
                        let __symbol = match __integer {
                            0 => match __lookahead.1 {
                                Token(4, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            1 => match __lookahead.1 {
                                Token(5, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            2 => match __lookahead.1 {
                                Token(6, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            3 => match __lookahead.1 {
                                Token(7, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            4 => match __lookahead.1 {
                                Token(8, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            5 => match __lookahead.1 {
                                Token(9, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            6 => match __lookahead.1 {
                                Token(10, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            7 => match __lookahead.1 {
                                Token(11, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            8 => match __lookahead.1 {
                                Token(12, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            9 => match __lookahead.1 {
                                Token(13, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            10 => match __lookahead.1 {
                                Token(14, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            11 => match __lookahead.1 {
                                Token(15, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            12 => match __lookahead.1 {
                                Token(16, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            13 => match __lookahead.1 {
                                Token(0, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            14 => match __lookahead.1 {
                                Token(1, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            15 => match __lookahead.1 {
                                Token(2, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            16 => match __lookahead.1 {
                                Token(3, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            _ => unreachable!(),
                        };
                        __states.push(__action - 1);
                        __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                        continue '__shift;
                    } else if __action < 0 {
                        if let Some(r) = __reduce(input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                            if r.is_err() {
                                return r;
                            }
                            return Err(__lalrpop_util::ParseError::ExtraToken { token: __lookahead });
                        }
                    } else {
                        let mut __err_lookahead = Some(__lookahead);
                        let mut __err_integer: Option<usize> = Some(__integer);
                        let __state = *__states.last().unwrap() as usize;
                        let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                            token: __err_lookahead,
                            expected: __expected_tokens(__state),
                        };
                        return Err(__error)
                    }
                }
            }
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __EOF_ACTION[__state];
                if __action < 0 {
                    if let Some(r) = __reduce(input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let mut __err_lookahead = None;
                    let mut __err_integer: Option<usize> = None;
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: __err_lookahead,
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
    }
    pub(crate) fn __reduce<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Box<Expr>,__lalrpop_util::ParseError<usize, Token<'input>, &'static str>>>
    {
        let (__pop_states, __symbol, __nonterminal) = match -__action {
            1 => {
                __reduce1(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            2 => {
                __reduce2(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            3 => {
                __reduce3(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            4 => {
                __reduce4(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            5 => {
                __reduce5(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            6 => {
                __reduce6(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            7 => {
                __reduce7(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            8 => {
                __reduce8(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            9 => {
                __reduce9(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            10 => {
                __reduce10(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            11 => {
                __reduce11(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            12 => {
                __reduce12(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            13 => {
                __reduce13(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            14 => {
                __reduce14(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            15 => {
                __reduce15(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            16 => {
                __reduce16(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            17 => {
                __reduce17(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            18 => {
                __reduce18(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            19 => {
                __reduce19(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            20 => {
                __reduce20(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            21 => {
                __reduce21(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            22 => {
                __reduce22(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            23 => {
                __reduce23(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            24 => {
                __reduce24(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            25 => {
                __reduce25(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            26 => {
                __reduce26(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            27 => {
                __reduce27(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            28 => {
                __reduce28(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            29 => {
                __reduce29(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            30 => {
                __reduce30(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            31 => {
                __reduce31(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            32 => {
                __reduce32(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            33 => {
                __reduce33(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            34 => {
                __reduce34(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            35 => {
                __reduce35(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            36 => {
                __reduce36(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            37 => {
                __reduce37(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            38 => {
                __reduce38(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            39 => {
                __reduce39(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            40 => {
                __reduce40(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            41 => {
                __reduce41(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            42 => {
                __reduce42(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            43 => {
                __reduce43(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            44 => {
                __reduce44(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            45 => {
                __reduce45(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            46 => {
                __reduce46(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            47 => {
                __reduce47(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            48 => {
                __reduce48(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            49 => {
                __reduce49(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            50 => {
                // __Expr = Expr => ActionFn(1);
                let __sym0 = __pop_Variant1(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            51 => {
                __reduce51(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        __symbols.push(__symbol);
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 26 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant1(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Expr, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant3(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant7<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Identifier, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant7(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Operator, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant2(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant5<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, bool, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant5(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant6<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, f64, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant6(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant9<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, i32, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant9(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Expr>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant4(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant8<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Identifier>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant8(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant0<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant0(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    pub(crate) fn __reduce1<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Assignment = Identifier, "=", Expr => ActionFn(31);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action31::<>(input, __sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (3, __symbol, 0)
    }
    pub(crate) fn __reduce2<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // ConstAssignment = "const", Identifier, "=", Expr => ActionFn(30);
        let __sym3 = __pop_Variant1(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action30::<>(input, __sym0, __sym1, __sym2, __sym3);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (4, __symbol, 1)
    }
    pub(crate) fn __reduce3<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Expr = Expr, ExprOp, Factor => ActionFn(6);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action6::<>(input, __sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (3, __symbol, 2)
    }
    pub(crate) fn __reduce4<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Expr = Factor => ActionFn(7);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action7::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 2)
    }
    pub(crate) fn __reduce5<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Expr = String => ActionFn(8);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action8::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 2)
    }
    pub(crate) fn __reduce6<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // ExprOp = "+" => ActionFn(9);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action9::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 3)
    }
    pub(crate) fn __reduce7<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // ExprOp = "-" => ActionFn(10);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action10::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 3)
    }
    pub(crate) fn __reduce8<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // ExprOrStmt = Expr => ActionFn(2);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 4)
    }
    pub(crate) fn __reduce9<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // ExprOrStmt = LetAssignment => ActionFn(3);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 4)
    }
    pub(crate) fn __reduce10<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // ExprOrStmt = ConstAssignment => ActionFn(4);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 4)
    }
    pub(crate) fn __reduce11<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // ExprOrStmt = Assignment => ActionFn(5);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action5::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 4)
    }
    pub(crate) fn __reduce12<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // ExprOrStmtSep = ExprOrStmt, ";" => ActionFn(23);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action23::<>(input, __sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (2, __symbol, 5)
    }
    pub(crate) fn __reduce13<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // ExprOrStmtSep* =  => ActionFn(35);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action35::<>(input, &__start, &__end);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (0, __symbol, 6)
    }
    pub(crate) fn __reduce14<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // ExprOrStmtSep* = ExprOrStmtSep+ => ActionFn(36);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action36::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (1, __symbol, 6)
    }
    pub(crate) fn __reduce15<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // ExprOrStmtSep+ = ExprOrStmtSep => ActionFn(45);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action45::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (1, __symbol, 7)
    }
    pub(crate) fn __reduce16<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // ExprOrStmtSep+ = ExprOrStmtSep+, ExprOrStmtSep => ActionFn(46);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action46::<>(input, __sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (2, __symbol, 7)
    }
    pub(crate) fn __reduce17<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Factor = Factor, FactorOp, Term => ActionFn(11);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action11::<>(input, __sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (3, __symbol, 8)
    }
    pub(crate) fn __reduce18<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Factor = Term => ActionFn(12);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action12::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 8)
    }
    pub(crate) fn __reduce19<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // FactorOp = "*" => ActionFn(13);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action13::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 9)
    }
    pub(crate) fn __reduce20<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // FactorOp = "/" => ActionFn(14);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action14::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 9)
    }
    pub(crate) fn __reduce21<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // False = "false" => ActionFn(27);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action27::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (1, __symbol, 10)
    }
    pub(crate) fn __reduce22<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Float = r#"\\d+\\.\\d+"# => ActionFn(25);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action25::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (1, __symbol, 11)
    }
    pub(crate) fn __reduce23<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Identifier = r#"[A-z]([A-z|0-9|_])*"# => ActionFn(34);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action34::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant7(__nt), __end);
        (1, __symbol, 12)
    }
    pub(crate) fn __reduce24<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Identifier* =  => ActionFn(37);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action37::<>(input, &__start, &__end);
        let __symbol = (__start, __Symbol::Variant8(__nt), __end);
        (0, __symbol, 13)
    }
    pub(crate) fn __reduce25<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Identifier* = Identifier+ => ActionFn(38);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action38::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant8(__nt), __end);
        (1, __symbol, 13)
    }
    pub(crate) fn __reduce26<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Identifier+ = Identifier => ActionFn(43);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action43::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant8(__nt), __end);
        (1, __symbol, 14)
    }
    pub(crate) fn __reduce27<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Identifier+ = Identifier+, Identifier => ActionFn(44);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action44::<>(input, __sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant8(__nt), __end);
        (2, __symbol, 14)
    }
    pub(crate) fn __reduce28<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // IdentifierExpr = Identifier => ActionFn(32);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action32::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 15)
    }
    pub(crate) fn __reduce29<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // IdentifierExprUnboxed = Identifier => ActionFn(33);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action33::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (1, __symbol, 16)
    }
    pub(crate) fn __reduce30<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // IdentifierExprUnboxed* =  => ActionFn(39);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action39::<>(input, &__start, &__end);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (0, __symbol, 17)
    }
    pub(crate) fn __reduce31<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // IdentifierExprUnboxed* = IdentifierExprUnboxed+ => ActionFn(40);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action40::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (1, __symbol, 17)
    }
    pub(crate) fn __reduce32<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // IdentifierExprUnboxed+ = IdentifierExprUnboxed => ActionFn(41);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action41::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (1, __symbol, 18)
    }
    pub(crate) fn __reduce33<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // IdentifierExprUnboxed+ = IdentifierExprUnboxed+, IdentifierExprUnboxed => ActionFn(42);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action42::<>(input, __sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (2, __symbol, 18)
    }
    pub(crate) fn __reduce34<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Int = r#"[0-9]+"# => ActionFn(24);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action24::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant9(__nt), __end);
        (1, __symbol, 19)
    }
    pub(crate) fn __reduce35<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // LetAssignment = "let", Identifier, "=", Expr => ActionFn(29);
        let __sym3 = __pop_Variant1(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action29::<>(input, __sym0, __sym1, __sym2, __sym3);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (4, __symbol, 20)
    }
    pub(crate) fn __reduce36<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // String = r#"\"[^\"]+\""# => ActionFn(28);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action28::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 21)
    }
    pub(crate) fn __reduce37<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Term = Identifier, "(", ")" => ActionFn(53);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action53::<>(input, __sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (3, __symbol, 22)
    }
    pub(crate) fn __reduce38<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Term = Identifier, "(", IdentifierExprUnboxed+, ")" => ActionFn(54);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action54::<>(input, __sym0, __sym1, __sym2, __sym3);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (4, __symbol, 22)
    }
    pub(crate) fn __reduce39<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Term = "(", ") => " => ActionFn(49);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action49::<>(input, __sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (2, __symbol, 22)
    }
    pub(crate) fn __reduce40<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Term = "(", Identifier+, ") => " => ActionFn(50);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action50::<>(input, __sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (3, __symbol, 22)
    }
    pub(crate) fn __reduce41<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Term = "(", ") => ", ExprOrStmtSep+ => ActionFn(51);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action51::<>(input, __sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (3, __symbol, 22)
    }
    pub(crate) fn __reduce42<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Term = "(", Identifier+, ") => ", ExprOrStmtSep+ => ActionFn(52);
        let __sym3 = __pop_Variant4(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action52::<>(input, __sym0, __sym1, __sym2, __sym3);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (4, __symbol, 22)
    }
    pub(crate) fn __reduce43<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Term = Float => ActionFn(17);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action17::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 22)
    }
    pub(crate) fn __reduce44<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Term = Int => ActionFn(18);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action18::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 22)
    }
    pub(crate) fn __reduce45<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Term = True => ActionFn(19);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action19::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 22)
    }
    pub(crate) fn __reduce46<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Term = False => ActionFn(20);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action20::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 22)
    }
    pub(crate) fn __reduce47<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Term = IdentifierExpr => ActionFn(21);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action21::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 22)
    }
    pub(crate) fn __reduce48<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Term = "(", Expr, ")" => ActionFn(22);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action22::<>(input, __sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (3, __symbol, 22)
    }
    pub(crate) fn __reduce49<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // True = "true" => ActionFn(26);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action26::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (1, __symbol, 23)
    }
    pub(crate) fn __reduce51<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // __ExprOrStmt = ExprOrStmt => ActionFn(0);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action0::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 25)
    }
}
pub use self::__parse__Expr::ExprParser;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__ExprOrStmt {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use std::str::FromStr;
    use ast::{Expr, Operator, Value, Identifier};
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    use super::__intern_token::Token;
    #[allow(dead_code)]
    pub enum __Symbol<'input>
     {
        Variant0(&'input str),
        Variant1(Box<Expr>),
        Variant2(Operator),
        Variant3(Expr),
        Variant4(::std::vec::Vec<Expr>),
        Variant5(bool),
        Variant6(f64),
        Variant7(Identifier),
        Variant8(::std::vec::Vec<Identifier>),
        Variant9(i32),
    }
    const __ACTION: &'static [i8] = &[
        // State 0
        16, 0, 0, 0, 0, 0, 0, 0, 0, 17, 18, 19, 20, 21, 22, 23, 24,
        // State 1
        0, 0, 0, 0, 0, 0, 0, -11, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 26, 27, 0, -8, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, -4, 0, 29, -4, -4, 30, -4, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, -46, 0, -46, -46, -46, -46, -46, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, -43, 0, -43, -43, -43, -43, -43, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        31, 0, 0, -28, -28, -28, -28, -28, 32, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, -47, 0, -47, -47, -47, -47, -47, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, -44, 0, -44, -44, -44, -44, -44, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, -9, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, -5, 0, 0, -5, -5, 0, -5, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, -18, 0, -18, -18, -18, -18, -18, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, -45, 0, -45, -45, -45, -45, -45, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        16, 0, 36, 0, 0, 0, 0, 0, 0, 0, 18, 0, 20, 21, 22, 23, 24,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 23, 0,
        // State 17
        0, -21, 0, -21, -21, -21, -21, -21, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 23, 0,
        // State 19
        0, -49, 0, -49, -49, -49, -49, -49, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, -36, 0, 0, -36, -36, 0, -36, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, -34, 0, -34, -34, -34, -34, -34, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        -23, -23, -23, -23, -23, -23, -23, -23, -23, 0, 0, 0, 0, 0, 0, -23, 0,
        // State 23
        0, -22, 0, -22, -22, -22, -22, -22, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 18, 0, 20, 0, 22, 23, 24,
        // State 25
        -6, 0, 0, 0, 0, 0, 0, 0, 0, 0, -6, 0, -6, 0, -6, -6, -6,
        // State 26
        -7, 0, 0, 0, 0, 0, 0, 0, 0, 0, -7, 0, -7, 0, -7, -7, -7,
        // State 27
        16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 18, 0, 20, 0, 22, 23, 24,
        // State 28
        -19, 0, 0, 0, 0, 0, 0, 0, 0, 0, -19, 0, -19, 0, -19, -19, -19,
        // State 29
        -20, 0, 0, 0, 0, 0, 0, 0, 0, 0, -20, 0, -20, 0, -20, -20, -20,
        // State 30
        0, 45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 23, 0,
        // State 31
        16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 18, 0, 20, 21, 22, 23, 24,
        // State 32
        0, 47, 0, 0, 26, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        31, -28, -26, -28, -28, -28, -28, 0, 0, 0, 0, 0, 0, 0, 0, -26, 0,
        // State 34
        0, 0, 49, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 23, 0,
        // State 35
        16, -39, 0, -39, -39, -39, -39, -39, 0, 17, 18, 19, 20, 21, 22, 23, 24,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 53, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 54, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, -3, 0, 29, -3, -3, 30, -3, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        31, -28, 0, -28, -28, -28, -28, -28, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, -17, 0, -17, -17, -17, -17, -17, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        0, -29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -29, 0,
        // State 42
        0, -32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -32, 0,
        // State 43
        0, 56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 23, 0,
        // State 44
        0, -37, 0, -37, -37, -37, -37, -37, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, 0, 0, 0, 26, 27, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        0, -48, 0, -48, -48, -48, -48, -48, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        0, 0, -27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -27, 0,
        // State 48
        16, -40, 0, -40, -40, -40, -40, -40, 0, 17, 18, 19, 20, 21, 22, 23, 24,
        // State 49
        0, 0, 0, 0, 0, 0, 0, 58, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        -15, -15, 0, -15, -15, -15, -15, -15, 0, -15, -15, -15, -15, -15, -15, -15, -15,
        // State 51
        16, -41, 0, -41, -41, -41, -41, -41, 0, 17, 18, 19, 20, 21, 22, 23, 24,
        // State 52
        16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 18, 0, 20, 21, 22, 23, 24,
        // State 53
        16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 18, 0, 20, 21, 22, 23, 24,
        // State 54
        0, -33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -33, 0,
        // State 55
        0, -38, 0, -38, -38, -38, -38, -38, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 56
        16, -42, 0, -42, -42, -42, -42, -42, 0, 17, 18, 19, 20, 21, 22, 23, 24,
        // State 57
        -12, -12, 0, -12, -12, -12, -12, -12, 0, -12, -12, -12, -12, -12, -12, -12, -12,
        // State 58
        -16, -16, 0, -16, -16, -16, -16, -16, 0, -16, -16, -16, -16, -16, -16, -16, -16,
        // State 59
        0, 0, 0, 0, 26, 27, 0, -2, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 60
        0, 0, 0, 0, 26, 27, 0, -35, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    const __EOF_ACTION: &'static [i8] = &[
        // State 0
        0,
        // State 1
        -11,
        // State 2
        -10,
        // State 3
        -8,
        // State 4
        -51,
        // State 5
        -4,
        // State 6
        -46,
        // State 7
        -43,
        // State 8
        -28,
        // State 9
        -47,
        // State 10
        -44,
        // State 11
        -9,
        // State 12
        -5,
        // State 13
        -18,
        // State 14
        -45,
        // State 15
        0,
        // State 16
        0,
        // State 17
        -21,
        // State 18
        0,
        // State 19
        -49,
        // State 20
        -36,
        // State 21
        -34,
        // State 22
        -23,
        // State 23
        -22,
        // State 24
        0,
        // State 25
        0,
        // State 26
        0,
        // State 27
        0,
        // State 28
        0,
        // State 29
        0,
        // State 30
        0,
        // State 31
        0,
        // State 32
        0,
        // State 33
        0,
        // State 34
        0,
        // State 35
        -39,
        // State 36
        0,
        // State 37
        0,
        // State 38
        -3,
        // State 39
        -28,
        // State 40
        -17,
        // State 41
        0,
        // State 42
        0,
        // State 43
        0,
        // State 44
        -37,
        // State 45
        -1,
        // State 46
        -48,
        // State 47
        0,
        // State 48
        -40,
        // State 49
        0,
        // State 50
        -15,
        // State 51
        -41,
        // State 52
        0,
        // State 53
        0,
        // State 54
        0,
        // State 55
        -38,
        // State 56
        -42,
        // State 57
        -12,
        // State 58
        -16,
        // State 59
        -2,
        // State 60
        -35,
    ];
    const __GOTO: &'static [i8] = &[
        // State 0
        2, 3, 4, 0, 5, 0, 0, 0, 6, 0, 7, 8, 9, 0, 0, 10, 0, 0, 0, 11, 12, 13, 14, 15, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 33, 0, 0, 0, 0, 0, 6, 0, 7, 8, 34, 0, 35, 10, 0, 0, 0, 11, 0, 13, 14, 15, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 7, 8, 40, 0, 0, 10, 0, 0, 0, 11, 0, 0, 14, 15, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 8, 40, 0, 0, 10, 0, 0, 0, 11, 0, 0, 41, 15, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 42, 0, 0, 0, 43, 0, 44, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 46, 0, 0, 0, 0, 0, 6, 0, 7, 8, 40, 0, 0, 10, 0, 0, 0, 11, 0, 13, 14, 15, 0, 0,
        // State 32
        0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        2, 3, 4, 0, 50, 51, 0, 52, 6, 0, 7, 8, 9, 0, 0, 10, 0, 0, 0, 11, 12, 13, 14, 15, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 43
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 42, 0, 0, 0, 55, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        2, 3, 4, 0, 50, 51, 0, 57, 6, 0, 7, 8, 9, 0, 0, 10, 0, 0, 0, 11, 12, 13, 14, 15, 0, 0,
        // State 49
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 51
        2, 3, 4, 0, 50, 59, 0, 0, 6, 0, 7, 8, 9, 0, 0, 10, 0, 0, 0, 11, 12, 13, 14, 15, 0, 0,
        // State 52
        0, 0, 60, 0, 0, 0, 0, 0, 6, 0, 7, 8, 40, 0, 0, 10, 0, 0, 0, 11, 0, 13, 14, 15, 0, 0,
        // State 53
        0, 0, 61, 0, 0, 0, 0, 0, 6, 0, 7, 8, 40, 0, 0, 10, 0, 0, 0, 11, 0, 13, 14, 15, 0, 0,
        // State 54
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 55
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 56
        2, 3, 4, 0, 50, 59, 0, 0, 6, 0, 7, 8, 9, 0, 0, 10, 0, 0, 0, 11, 12, 13, 14, 15, 0, 0,
        // State 57
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 58
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 59
        0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 60
        0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""(""###,
            r###"")""###,
            r###"") => ""###,
            r###""*""###,
            r###""+""###,
            r###""-""###,
            r###""/""###,
            r###"";""###,
            r###""=""###,
            r###""const""###,
            r###""false""###,
            r###""let""###,
            r###""true""###,
            r###"r#"\"[^\"]+\""#"###,
            r###"r#"[0-9]+"#"###,
            r###"r#"[A-z]([A-z|0-9|_])*"#"###,
            r###"r#"\\d+\\.\\d+"#"###,
        ];
        __ACTION[(__state * 17)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub struct ExprOrStmtParser {
        builder: super::__intern_token::__MatcherBuilder,
        _priv: (),
    }

    impl ExprOrStmtParser {
        pub fn new() -> ExprOrStmtParser {
            let __builder = super::__intern_token::__MatcherBuilder::new();
            ExprOrStmtParser {
                builder: __builder,
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            'input,
        >(
            &self,
            input: &'input str,
        ) -> Result<Box<Expr>, __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>
        {
            let mut __tokens = self.builder.matcher(input);
            let mut __states = vec![0_i8];
            let mut __symbols = vec![];
            let mut __integer;
            let mut __lookahead;
            let __last_location = &mut Default::default();
            '__shift: loop {
                __lookahead = match __tokens.next() {
                    Some(Ok(v)) => v,
                    None => break '__shift,
                    Some(Err(e)) => return Err(e),
                };
                *__last_location = __lookahead.2.clone();
                __integer = match __lookahead.1 {
                    Token(4, _) if true => 0,
                    Token(5, _) if true => 1,
                    Token(6, _) if true => 2,
                    Token(7, _) if true => 3,
                    Token(8, _) if true => 4,
                    Token(9, _) if true => 5,
                    Token(10, _) if true => 6,
                    Token(11, _) if true => 7,
                    Token(12, _) if true => 8,
                    Token(13, _) if true => 9,
                    Token(14, _) if true => 10,
                    Token(15, _) if true => 11,
                    Token(16, _) if true => 12,
                    Token(0, _) if true => 13,
                    Token(1, _) if true => 14,
                    Token(2, _) if true => 15,
                    Token(3, _) if true => 16,
                    _ => {
                        let __state = *__states.last().unwrap() as usize;
                        let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                            token: Some(__lookahead),
                            expected: __expected_tokens(__state),
                        };
                        return Err(__error);
                    }
                };
                '__inner: loop {
                    let __state = *__states.last().unwrap() as usize;
                    let __action = __ACTION[__state * 17 + __integer];
                    if __action > 0 {
                        let __symbol = match __integer {
                            0 => match __lookahead.1 {
                                Token(4, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            1 => match __lookahead.1 {
                                Token(5, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            2 => match __lookahead.1 {
                                Token(6, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            3 => match __lookahead.1 {
                                Token(7, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            4 => match __lookahead.1 {
                                Token(8, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            5 => match __lookahead.1 {
                                Token(9, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            6 => match __lookahead.1 {
                                Token(10, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            7 => match __lookahead.1 {
                                Token(11, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            8 => match __lookahead.1 {
                                Token(12, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            9 => match __lookahead.1 {
                                Token(13, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            10 => match __lookahead.1 {
                                Token(14, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            11 => match __lookahead.1 {
                                Token(15, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            12 => match __lookahead.1 {
                                Token(16, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            13 => match __lookahead.1 {
                                Token(0, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            14 => match __lookahead.1 {
                                Token(1, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            15 => match __lookahead.1 {
                                Token(2, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            16 => match __lookahead.1 {
                                Token(3, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            _ => unreachable!(),
                        };
                        __states.push(__action - 1);
                        __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                        continue '__shift;
                    } else if __action < 0 {
                        if let Some(r) = __reduce(input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                            if r.is_err() {
                                return r;
                            }
                            return Err(__lalrpop_util::ParseError::ExtraToken { token: __lookahead });
                        }
                    } else {
                        let mut __err_lookahead = Some(__lookahead);
                        let mut __err_integer: Option<usize> = Some(__integer);
                        let __state = *__states.last().unwrap() as usize;
                        let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                            token: __err_lookahead,
                            expected: __expected_tokens(__state),
                        };
                        return Err(__error)
                    }
                }
            }
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __EOF_ACTION[__state];
                if __action < 0 {
                    if let Some(r) = __reduce(input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let mut __err_lookahead = None;
                    let mut __err_integer: Option<usize> = None;
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: __err_lookahead,
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
    }
    pub(crate) fn __reduce<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Box<Expr>,__lalrpop_util::ParseError<usize, Token<'input>, &'static str>>>
    {
        let (__pop_states, __symbol, __nonterminal) = match -__action {
            1 => {
                __reduce1(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            2 => {
                __reduce2(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            3 => {
                __reduce3(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            4 => {
                __reduce4(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            5 => {
                __reduce5(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            6 => {
                __reduce6(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            7 => {
                __reduce7(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            8 => {
                __reduce8(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            9 => {
                __reduce9(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            10 => {
                __reduce10(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            11 => {
                __reduce11(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            12 => {
                __reduce12(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            13 => {
                __reduce13(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            14 => {
                __reduce14(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            15 => {
                __reduce15(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            16 => {
                __reduce16(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            17 => {
                __reduce17(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            18 => {
                __reduce18(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            19 => {
                __reduce19(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            20 => {
                __reduce20(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            21 => {
                __reduce21(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            22 => {
                __reduce22(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            23 => {
                __reduce23(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            24 => {
                __reduce24(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            25 => {
                __reduce25(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            26 => {
                __reduce26(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            27 => {
                __reduce27(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            28 => {
                __reduce28(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            29 => {
                __reduce29(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            30 => {
                __reduce30(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            31 => {
                __reduce31(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            32 => {
                __reduce32(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            33 => {
                __reduce33(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            34 => {
                __reduce34(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            35 => {
                __reduce35(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            36 => {
                __reduce36(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            37 => {
                __reduce37(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            38 => {
                __reduce38(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            39 => {
                __reduce39(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            40 => {
                __reduce40(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            41 => {
                __reduce41(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            42 => {
                __reduce42(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            43 => {
                __reduce43(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            44 => {
                __reduce44(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            45 => {
                __reduce45(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            46 => {
                __reduce46(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            47 => {
                __reduce47(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            48 => {
                __reduce48(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            49 => {
                __reduce49(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            50 => {
                __reduce50(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            51 => {
                // __ExprOrStmt = ExprOrStmt => ActionFn(0);
                let __sym0 = __pop_Variant1(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        __symbols.push(__symbol);
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 26 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant1(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Expr, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant3(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant7<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Identifier, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant7(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Operator, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant2(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant5<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, bool, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant5(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant6<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, f64, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant6(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant9<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, i32, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant9(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Expr>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant4(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant8<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Identifier>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant8(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant0<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant0(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    pub(crate) fn __reduce1<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Assignment = Identifier, "=", Expr => ActionFn(31);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action31::<>(input, __sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (3, __symbol, 0)
    }
    pub(crate) fn __reduce2<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // ConstAssignment = "const", Identifier, "=", Expr => ActionFn(30);
        let __sym3 = __pop_Variant1(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action30::<>(input, __sym0, __sym1, __sym2, __sym3);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (4, __symbol, 1)
    }
    pub(crate) fn __reduce3<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Expr = Expr, ExprOp, Factor => ActionFn(6);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action6::<>(input, __sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (3, __symbol, 2)
    }
    pub(crate) fn __reduce4<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Expr = Factor => ActionFn(7);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action7::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 2)
    }
    pub(crate) fn __reduce5<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Expr = String => ActionFn(8);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action8::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 2)
    }
    pub(crate) fn __reduce6<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // ExprOp = "+" => ActionFn(9);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action9::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 3)
    }
    pub(crate) fn __reduce7<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // ExprOp = "-" => ActionFn(10);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action10::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 3)
    }
    pub(crate) fn __reduce8<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // ExprOrStmt = Expr => ActionFn(2);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 4)
    }
    pub(crate) fn __reduce9<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // ExprOrStmt = LetAssignment => ActionFn(3);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 4)
    }
    pub(crate) fn __reduce10<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // ExprOrStmt = ConstAssignment => ActionFn(4);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 4)
    }
    pub(crate) fn __reduce11<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // ExprOrStmt = Assignment => ActionFn(5);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action5::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 4)
    }
    pub(crate) fn __reduce12<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // ExprOrStmtSep = ExprOrStmt, ";" => ActionFn(23);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action23::<>(input, __sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (2, __symbol, 5)
    }
    pub(crate) fn __reduce13<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // ExprOrStmtSep* =  => ActionFn(35);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action35::<>(input, &__start, &__end);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (0, __symbol, 6)
    }
    pub(crate) fn __reduce14<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // ExprOrStmtSep* = ExprOrStmtSep+ => ActionFn(36);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action36::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (1, __symbol, 6)
    }
    pub(crate) fn __reduce15<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // ExprOrStmtSep+ = ExprOrStmtSep => ActionFn(45);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action45::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (1, __symbol, 7)
    }
    pub(crate) fn __reduce16<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // ExprOrStmtSep+ = ExprOrStmtSep+, ExprOrStmtSep => ActionFn(46);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action46::<>(input, __sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (2, __symbol, 7)
    }
    pub(crate) fn __reduce17<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Factor = Factor, FactorOp, Term => ActionFn(11);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action11::<>(input, __sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (3, __symbol, 8)
    }
    pub(crate) fn __reduce18<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Factor = Term => ActionFn(12);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action12::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 8)
    }
    pub(crate) fn __reduce19<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // FactorOp = "*" => ActionFn(13);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action13::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 9)
    }
    pub(crate) fn __reduce20<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // FactorOp = "/" => ActionFn(14);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action14::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 9)
    }
    pub(crate) fn __reduce21<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // False = "false" => ActionFn(27);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action27::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (1, __symbol, 10)
    }
    pub(crate) fn __reduce22<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Float = r#"\\d+\\.\\d+"# => ActionFn(25);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action25::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (1, __symbol, 11)
    }
    pub(crate) fn __reduce23<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Identifier = r#"[A-z]([A-z|0-9|_])*"# => ActionFn(34);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action34::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant7(__nt), __end);
        (1, __symbol, 12)
    }
    pub(crate) fn __reduce24<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Identifier* =  => ActionFn(37);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action37::<>(input, &__start, &__end);
        let __symbol = (__start, __Symbol::Variant8(__nt), __end);
        (0, __symbol, 13)
    }
    pub(crate) fn __reduce25<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Identifier* = Identifier+ => ActionFn(38);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action38::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant8(__nt), __end);
        (1, __symbol, 13)
    }
    pub(crate) fn __reduce26<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Identifier+ = Identifier => ActionFn(43);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action43::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant8(__nt), __end);
        (1, __symbol, 14)
    }
    pub(crate) fn __reduce27<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Identifier+ = Identifier+, Identifier => ActionFn(44);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action44::<>(input, __sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant8(__nt), __end);
        (2, __symbol, 14)
    }
    pub(crate) fn __reduce28<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // IdentifierExpr = Identifier => ActionFn(32);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action32::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 15)
    }
    pub(crate) fn __reduce29<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // IdentifierExprUnboxed = Identifier => ActionFn(33);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action33::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (1, __symbol, 16)
    }
    pub(crate) fn __reduce30<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // IdentifierExprUnboxed* =  => ActionFn(39);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action39::<>(input, &__start, &__end);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (0, __symbol, 17)
    }
    pub(crate) fn __reduce31<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // IdentifierExprUnboxed* = IdentifierExprUnboxed+ => ActionFn(40);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action40::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (1, __symbol, 17)
    }
    pub(crate) fn __reduce32<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // IdentifierExprUnboxed+ = IdentifierExprUnboxed => ActionFn(41);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action41::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (1, __symbol, 18)
    }
    pub(crate) fn __reduce33<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // IdentifierExprUnboxed+ = IdentifierExprUnboxed+, IdentifierExprUnboxed => ActionFn(42);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action42::<>(input, __sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (2, __symbol, 18)
    }
    pub(crate) fn __reduce34<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Int = r#"[0-9]+"# => ActionFn(24);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action24::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant9(__nt), __end);
        (1, __symbol, 19)
    }
    pub(crate) fn __reduce35<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // LetAssignment = "let", Identifier, "=", Expr => ActionFn(29);
        let __sym3 = __pop_Variant1(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action29::<>(input, __sym0, __sym1, __sym2, __sym3);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (4, __symbol, 20)
    }
    pub(crate) fn __reduce36<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // String = r#"\"[^\"]+\""# => ActionFn(28);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action28::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 21)
    }
    pub(crate) fn __reduce37<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Term = Identifier, "(", ")" => ActionFn(53);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action53::<>(input, __sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (3, __symbol, 22)
    }
    pub(crate) fn __reduce38<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Term = Identifier, "(", IdentifierExprUnboxed+, ")" => ActionFn(54);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action54::<>(input, __sym0, __sym1, __sym2, __sym3);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (4, __symbol, 22)
    }
    pub(crate) fn __reduce39<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Term = "(", ") => " => ActionFn(49);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action49::<>(input, __sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (2, __symbol, 22)
    }
    pub(crate) fn __reduce40<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Term = "(", Identifier+, ") => " => ActionFn(50);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action50::<>(input, __sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (3, __symbol, 22)
    }
    pub(crate) fn __reduce41<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Term = "(", ") => ", ExprOrStmtSep+ => ActionFn(51);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action51::<>(input, __sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (3, __symbol, 22)
    }
    pub(crate) fn __reduce42<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Term = "(", Identifier+, ") => ", ExprOrStmtSep+ => ActionFn(52);
        let __sym3 = __pop_Variant4(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action52::<>(input, __sym0, __sym1, __sym2, __sym3);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (4, __symbol, 22)
    }
    pub(crate) fn __reduce43<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Term = Float => ActionFn(17);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action17::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 22)
    }
    pub(crate) fn __reduce44<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Term = Int => ActionFn(18);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action18::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 22)
    }
    pub(crate) fn __reduce45<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Term = True => ActionFn(19);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action19::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 22)
    }
    pub(crate) fn __reduce46<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Term = False => ActionFn(20);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action20::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 22)
    }
    pub(crate) fn __reduce47<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Term = IdentifierExpr => ActionFn(21);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action21::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 22)
    }
    pub(crate) fn __reduce48<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // Term = "(", Expr, ")" => ActionFn(22);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action22::<>(input, __sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (3, __symbol, 22)
    }
    pub(crate) fn __reduce49<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // True = "true" => ActionFn(26);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action26::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (1, __symbol, 23)
    }
    pub(crate) fn __reduce50<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize)
    {
        // __Expr = Expr => ActionFn(1);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(input, __sym0);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (1, __symbol, 24)
    }
}
pub use self::__parse__ExprOrStmt::ExprOrStmtParser;
#[cfg_attr(rustfmt, rustfmt_skip)]
mod __intern_token {
    #![allow(unused_imports)]
    use std::str::FromStr;
    use ast::{Expr, Operator, Value, Identifier};
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    extern crate regex as __regex;
    use std::fmt as __fmt;

    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Token<'input>(pub usize, pub &'input str);
    impl<'a> __fmt::Display for Token<'a> {
        fn fmt(&self, formatter: &mut __fmt::Formatter) -> Result<(), __fmt::Error> {
            __fmt::Display::fmt(self.1, formatter)
        }
    }

    pub struct __MatcherBuilder {
        regex_set: __regex::RegexSet,
        regex_vec: Vec<__regex::Regex>,
    }

    impl __MatcherBuilder {
        pub fn new() -> __MatcherBuilder {
            let __strs: &[&str] = &[
                "^((?u:\")(?u:[\u{0}-!\\#-\u{10ffff}])+(?u:\"))",
                "^((?u:[0-9])+)",
                "^((?u:[A-z])((?u:[0-9A-z\\|-\\|]))*)",
                "^((?u:[0-9٠-٩۰-۹߀-߉०-९০-৯੦-੯૦-૯୦-୯௦-௯౦-౯೦-೯൦-൯෦-෯๐-๙໐-໙༠-༩၀-၉႐-႙០-៩᠐-᠙᥆-᥏᧐-᧙᪀-᪉᪐-᪙᭐-᭙᮰-᮹᱀-᱉᱐-᱙꘠-꘩꣐-꣙꤀-꤉꧐-꧙꧰-꧹꩐-꩙꯰-꯹０-９𐒠-𐒩𑁦-𑁯𑃰-𑃹𑄶-𑄿𑇐-𑇙𑋰-𑋹𑑐-𑑙𑓐-𑓙𑙐-𑙙𑛀-𑛉𑜰-𑜹𑣠-𑣩𑱐-𑱙𑵐-𑵙𖩠-𖩩𖭐-𖭙𝟎-𝟿𞥐-𞥙])+(?u:\\.)(?u:[0-9٠-٩۰-۹߀-߉०-९০-৯੦-੯૦-૯୦-୯௦-௯౦-౯೦-೯൦-൯෦-෯๐-๙໐-໙༠-༩၀-၉႐-႙០-៩᠐-᠙᥆-᥏᧐-᧙᪀-᪉᪐-᪙᭐-᭙᮰-᮹᱀-᱉᱐-᱙꘠-꘩꣐-꣙꤀-꤉꧐-꧙꧰-꧹꩐-꩙꯰-꯹０-９𐒠-𐒩𑁦-𑁯𑃰-𑃹𑄶-𑄿𑇐-𑇙𑋰-𑋹𑑐-𑑙𑓐-𑓙𑙐-𑙙𑛀-𑛉𑜰-𑜹𑣠-𑣩𑱐-𑱙𑵐-𑵙𖩠-𖩩𖭐-𖭙𝟎-𝟿𞥐-𞥙])+)",
                "^((?u:\\())",
                "^((?u:\\)))",
                "^((?u:\\) => ))",
                "^((?u:\\*))",
                "^((?u:\\+))",
                "^((?u:\\-))",
                "^((?u:/))",
                "^((?u:;))",
                "^((?u:=))",
                "^((?u:const))",
                "^((?u:false))",
                "^((?u:let))",
                "^((?u:true))",
            ];
            let __regex_set = __regex::RegexSet::new(__strs).unwrap();
            let __regex_vec = vec![
                __regex::Regex::new("^((?u:\")(?u:[\u{0}-!\\#-\u{10ffff}])+(?u:\"))").unwrap(),
                __regex::Regex::new("^((?u:[0-9])+)").unwrap(),
                __regex::Regex::new("^((?u:[A-z])((?u:[0-9A-z\\|-\\|]))*)").unwrap(),
                __regex::Regex::new("^((?u:[0-9٠-٩۰-۹߀-߉०-९০-৯੦-੯૦-૯୦-୯௦-௯౦-౯೦-೯൦-൯෦-෯๐-๙໐-໙༠-༩၀-၉႐-႙០-៩᠐-᠙᥆-᥏᧐-᧙᪀-᪉᪐-᪙᭐-᭙᮰-᮹᱀-᱉᱐-᱙꘠-꘩꣐-꣙꤀-꤉꧐-꧙꧰-꧹꩐-꩙꯰-꯹０-９𐒠-𐒩𑁦-𑁯𑃰-𑃹𑄶-𑄿𑇐-𑇙𑋰-𑋹𑑐-𑑙𑓐-𑓙𑙐-𑙙𑛀-𑛉𑜰-𑜹𑣠-𑣩𑱐-𑱙𑵐-𑵙𖩠-𖩩𖭐-𖭙𝟎-𝟿𞥐-𞥙])+(?u:\\.)(?u:[0-9٠-٩۰-۹߀-߉०-९০-৯੦-੯૦-૯୦-୯௦-௯౦-౯೦-೯൦-൯෦-෯๐-๙໐-໙༠-༩၀-၉႐-႙០-៩᠐-᠙᥆-᥏᧐-᧙᪀-᪉᪐-᪙᭐-᭙᮰-᮹᱀-᱉᱐-᱙꘠-꘩꣐-꣙꤀-꤉꧐-꧙꧰-꧹꩐-꩙꯰-꯹０-９𐒠-𐒩𑁦-𑁯𑃰-𑃹𑄶-𑄿𑇐-𑇙𑋰-𑋹𑑐-𑑙𑓐-𑓙𑙐-𑙙𑛀-𑛉𑜰-𑜹𑣠-𑣩𑱐-𑱙𑵐-𑵙𖩠-𖩩𖭐-𖭙𝟎-𝟿𞥐-𞥙])+)").unwrap(),
                __regex::Regex::new("^((?u:\\())").unwrap(),
                __regex::Regex::new("^((?u:\\)))").unwrap(),
                __regex::Regex::new("^((?u:\\) => ))").unwrap(),
                __regex::Regex::new("^((?u:\\*))").unwrap(),
                __regex::Regex::new("^((?u:\\+))").unwrap(),
                __regex::Regex::new("^((?u:\\-))").unwrap(),
                __regex::Regex::new("^((?u:/))").unwrap(),
                __regex::Regex::new("^((?u:;))").unwrap(),
                __regex::Regex::new("^((?u:=))").unwrap(),
                __regex::Regex::new("^((?u:const))").unwrap(),
                __regex::Regex::new("^((?u:false))").unwrap(),
                __regex::Regex::new("^((?u:let))").unwrap(),
                __regex::Regex::new("^((?u:true))").unwrap(),
            ];
            __MatcherBuilder { regex_set: __regex_set, regex_vec: __regex_vec }
        }
        pub fn matcher<'input, 'builder>(&'builder self, s: &'input str) -> __Matcher<'input, 'builder> {
            __Matcher {
                text: s,
                consumed: 0,
                regex_set: &self.regex_set,
                regex_vec: &self.regex_vec,
            }
        }
    }

    pub struct __Matcher<'input, 'builder> {
        text: &'input str,
        consumed: usize,
        regex_set: &'builder __regex::RegexSet,
        regex_vec: &'builder Vec<__regex::Regex>,
    }

    impl<'input, 'builder> Iterator for __Matcher<'input, 'builder> {
        type Item = Result<(usize, Token<'input>, usize), __lalrpop_util::ParseError<usize,Token<'input>,&'static str>>;

        fn next(&mut self) -> Option<Self::Item> {
            let __text = self.text.trim_left();
            let __whitespace = self.text.len() - __text.len();
            let __start_offset = self.consumed + __whitespace;
            if __text.is_empty() {
                self.text = __text;
                self.consumed = __start_offset;
                None
            } else {
                let __matches = self.regex_set.matches(__text);
                if !__matches.matched_any() {
                    Some(Err(__lalrpop_util::ParseError::InvalidToken {
                        location: __start_offset,
                    }))
                } else {
                    let mut __longest_match = 0;
                    let mut __index = 0;
                    for __i in 0 .. 17 {
                        if __matches.matched(__i) {
                            let __match = self.regex_vec[__i].find(__text).unwrap();
                            let __len = __match.end();
                            if __len >= __longest_match {
                                __longest_match = __len;
                                __index = __i;
                            }
                        }
                    }
                    let __result = &__text[..__longest_match];
                    let __remaining = &__text[__longest_match..];
                    let __end_offset = __start_offset + __longest_match;
                    self.text = __remaining;
                    self.consumed = __end_offset;
                    Some(Ok((__start_offset, Token(__index, __result), __end_offset)))
                }
            }
        }
    }
}
pub use self::__intern_token::Token;

#[allow(unused_variables)]
fn __action0<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
fn __action1<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
fn __action2<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
fn __action3<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
fn __action4<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
fn __action5<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
fn __action6<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
    (_, __1, _): (usize, Operator, usize),
    (_, __2, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    Box::new(Expr::BinaryOp(__0, __1, __2))
}

#[allow(unused_variables)]
fn __action7<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
fn __action8<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
fn __action9<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Operator
{
    Operator::Add
}

#[allow(unused_variables)]
fn __action10<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Operator
{
    Operator::Sub
}

#[allow(unused_variables)]
fn __action11<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
    (_, __1, _): (usize, Operator, usize),
    (_, __2, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    Box::new(Expr::BinaryOp(__0, __1, __2))
}

#[allow(unused_variables)]
fn __action12<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
fn __action13<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Operator
{
    Operator::Mul
}

#[allow(unused_variables)]
fn __action14<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Operator
{
    Operator::Div
}

#[allow(unused_variables)]
fn __action15<
    'input,
>(
    input: &'input str,
    (_, ident, _): (usize, Identifier, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, exprs, _): (usize, ::std::vec::Vec<Expr>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Box<Expr>
{
    Box::new(Expr::CallFunction(ident, exprs))
}

#[allow(unused_variables)]
fn __action16<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, ids, _): (usize, ::std::vec::Vec<Identifier>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, exprs, _): (usize, ::std::vec::Vec<Expr>, usize),
) -> Box<Expr>
{
    Box::new(Expr::Function(ids, exprs))
}

#[allow(unused_variables)]
fn __action17<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, f64, usize),
) -> Box<Expr>
{
    Box::new(Expr::Literal(Value::Float(__0)))
}

#[allow(unused_variables)]
fn __action18<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, i32, usize),
) -> Box<Expr>
{
    Box::new(Expr::Literal(Value::Int(__0)))
}

#[allow(unused_variables)]
fn __action19<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, bool, usize),
) -> Box<Expr>
{
    Box::new(Expr::Literal(Value::Bool(true)))
}

#[allow(unused_variables)]
fn __action20<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, bool, usize),
) -> Box<Expr>
{
    Box::new(Expr::Literal(Value::Bool(false)))
}

#[allow(unused_variables)]
fn __action21<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
fn __action22<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Box<Expr>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
fn __action23<
    'input,
>(
    input: &'input str,
    (_, expr, _): (usize, Box<Expr>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Expr
{
    *expr
}

#[allow(unused_variables)]
fn __action24<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> i32
{
    i32::from_str(__0).unwrap()
}

#[allow(unused_variables)]
fn __action25<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> f64
{
    f64::from_str(__0).unwrap()
}

#[allow(unused_variables)]
fn __action26<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> bool
{
    true
}

#[allow(unused_variables)]
fn __action27<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> bool
{
    false
}

#[allow(unused_variables)]
fn __action28<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Box<Expr>
{
    Box::new(Expr::Literal(Value::String(String::from_str(__0).unwrap())))
}

#[allow(unused_variables)]
fn __action29<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, ident, _): (usize, Identifier, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, expr, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    Box::new(Expr::LetAssignment(ident, expr))
}

#[allow(unused_variables)]
fn __action30<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, ident, _): (usize, Identifier, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, expr, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    Box::new(Expr::ConstAssignment(ident, expr))
}

#[allow(unused_variables)]
fn __action31<
    'input,
>(
    input: &'input str,
    (_, ident, _): (usize, Identifier, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, expr, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    Box::new(Expr::Assignment(ident, expr))
}

#[allow(unused_variables)]
fn __action32<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Identifier, usize),
) -> Box<Expr>
{
    Box::new(Expr::Identifier(__0))
}

#[allow(unused_variables)]
fn __action33<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Identifier, usize),
) -> Expr
{
    Expr::Identifier(__0)
}

#[allow(unused_variables)]
fn __action34<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Identifier
{
    Identifier::new(String::from_str(__0).unwrap())
}

#[allow(unused_variables)]
fn __action35<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<Expr>
{
    vec![]
}

#[allow(unused_variables)]
fn __action36<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Expr>, usize),
) -> ::std::vec::Vec<Expr>
{
    v
}

#[allow(unused_variables)]
fn __action37<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<Identifier>
{
    vec![]
}

#[allow(unused_variables)]
fn __action38<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Identifier>, usize),
) -> ::std::vec::Vec<Identifier>
{
    v
}

#[allow(unused_variables)]
fn __action39<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<Expr>
{
    vec![]
}

#[allow(unused_variables)]
fn __action40<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Expr>, usize),
) -> ::std::vec::Vec<Expr>
{
    v
}

#[allow(unused_variables)]
fn __action41<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Expr, usize),
) -> ::std::vec::Vec<Expr>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action42<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Expr>, usize),
    (_, e, _): (usize, Expr, usize),
) -> ::std::vec::Vec<Expr>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action43<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Identifier, usize),
) -> ::std::vec::Vec<Identifier>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action44<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Identifier>, usize),
    (_, e, _): (usize, Identifier, usize),
) -> ::std::vec::Vec<Identifier>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action45<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Expr, usize),
) -> ::std::vec::Vec<Expr>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action46<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Expr>, usize),
    (_, e, _): (usize, Expr, usize),
) -> ::std::vec::Vec<Expr>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action47<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, ::std::vec::Vec<Identifier>, usize),
    __2: (usize, &'input str, usize),
) -> Box<Expr>
{
    let __start0 = __2.2.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action35(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action16(
        input,
        __0,
        __1,
        __2,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action48<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, ::std::vec::Vec<Identifier>, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, ::std::vec::Vec<Expr>, usize),
) -> Box<Expr>
{
    let __start0 = __3.0.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action36(
        input,
        __3,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action16(
        input,
        __0,
        __1,
        __2,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action49<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
) -> Box<Expr>
{
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __temp0 = __action37(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action47(
        input,
        __0,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action50<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, ::std::vec::Vec<Identifier>, usize),
    __2: (usize, &'input str, usize),
) -> Box<Expr>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action38(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action47(
        input,
        __0,
        __temp0,
        __2,
    )
}

#[allow(unused_variables)]
fn __action51<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, ::std::vec::Vec<Expr>, usize),
) -> Box<Expr>
{
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __temp0 = __action37(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action48(
        input,
        __0,
        __temp0,
        __1,
        __2,
    )
}

#[allow(unused_variables)]
fn __action52<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, ::std::vec::Vec<Identifier>, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, ::std::vec::Vec<Expr>, usize),
) -> Box<Expr>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action38(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action48(
        input,
        __0,
        __temp0,
        __2,
        __3,
    )
}

#[allow(unused_variables)]
fn __action53<
    'input,
>(
    input: &'input str,
    __0: (usize, Identifier, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, &'input str, usize),
) -> Box<Expr>
{
    let __start0 = __1.2.clone();
    let __end0 = __2.0.clone();
    let __temp0 = __action39(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action15(
        input,
        __0,
        __1,
        __temp0,
        __2,
    )
}

#[allow(unused_variables)]
fn __action54<
    'input,
>(
    input: &'input str,
    __0: (usize, Identifier, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, ::std::vec::Vec<Expr>, usize),
    __3: (usize, &'input str, usize),
) -> Box<Expr>
{
    let __start0 = __2.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action40(
        input,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action15(
        input,
        __0,
        __1,
        __temp0,
        __3,
    )
}

pub trait __ToTriple<'input, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize),Self::Error>;
}

impl<'input, > __ToTriple<'input, > for (usize, Token<'input>, usize) {
    type Error = &'static str;
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize),&'static str> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, Token<'input>, usize),&'static str> {
    type Error = &'static str;
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize),&'static str> {
        value
    }
}
