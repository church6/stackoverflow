// @filename   : stackoverflow_0007_57259126.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/57259126
// @title      : Why does the Rust compiler not optimize code assuming that two mutable references cannot alias?

#[allow(dead_code)]
mod answer1 {
    /*
    Rust originally did enable LLVM's noalias attribute, but this caused miscompiled code. When all supported LLVM versions no longer miscompile the code, it will be re-enabled.

    If you add -Zmutable-noalias=yes to the compiler options, you get the expected assembly:

    adds:
            mov     eax, dword ptr [rsi]
            add     eax, eax
            add     dword ptr [rdi], eax
            ret
    Simply put, Rust put the equivalent of C's restrict keyword everywhere, far more prevalent than any usual C program. This exercised corner cases of LLVM more than it was able to handle correctly. It turns out that C and C++ programmers simply don't use restrict as frequently as &mut is used in Rust.

    This has happened multiple times.

    Rust 1.0 through 1.7 — noalias enabled
    Rust 1.8 through 1.27 — noalias disabled
    Rust 1.28 through 1.29 — noalias enabled
    Rust 1.30 through 1.54 — noalias disabled
    */

    mod code1 {
        pub fn test() {
            // add your code here
        }
    }
    mod code2 {
        pub fn test() {
            // add your code here
        }
    }
    mod code3 {
        pub fn test() {
            // add your code here
        }
    }
    pub fn test() {
        //code1::test();
        //code2::test();
        //code3::test();
    }
}
#[allow(dead_code)]
mod answer2 {
    mod code1 {
        pub fn test() {
            // add your code here
        }
    }
    mod code2 {
        pub fn test() {
            // add your code here
        }
    }
    mod code3 {
        pub fn test() {
            // add your code here
        }
    }
    pub fn test() {
        //code1::test();
        //code2::test();
        //code3::test();
    }
}
#[allow(dead_code)]
mod answer3 {
    mod code1 {
        pub fn test() {
            // add your code here
        }
    }
    mod code2 {
        pub fn test() {
            // add your code here
        }
    }
    mod code3 {
        pub fn test() {
            // add your code here
        }
    }
    pub fn test() {
        //code1::test();
        //code2::test();
        //code3::test();
    }
}
pub fn test() {
    _enter!();
    //answer1::test();
    //answer2::test();
    //answer3::test();
    _leave!();
}
