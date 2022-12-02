// @filename   : stackoverflow_0228_43385142.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/43385142
// @title      : How is Rust compiled to machine code?

#[allow(dead_code)]
mod answer1 {
    /*
    The code-generation phase of the Rust compiler is mainly done by LLVM. LLVM is a set of tools for building a compiler, most notably used by the C[++] Compiler clang[++].

    First, the Rust compiler (just like clang, for example) does all the Rust specific stuff like type and borrow checking; in the end, it generates LLVM-IR. IR stands for intermediate representation and it's... comparable to assembly, but a tiny bit more high level and most importantly: platform independent. Then the Rust compiler just calls ☏ LLVM and says:

    Hey buddy, could you please take this IR and generate machine code for the current platform? That would be fantastic ◕ ◡ ◕

    To which LLVM responds:

    🌈 Sure, no problem, new friend. Here is your highly optimized machine code for [e.g.] x86_64! ♫♪♫

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
