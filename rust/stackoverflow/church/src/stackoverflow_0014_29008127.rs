// @filename   : stackoverflow_0014_29008127.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/29008127
// @title      : Why are Rust executables so huge?

#[allow(dead_code)]
mod answer1 {
    // cargo rustc [--debug or --release] -- -C prefer-dynamic
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
    /*
    Cargo.toml:
    [profile.release]
    opt-level = 'z'     # Optimize for size.
    lto = true          # Enable Link Time Optimization
    codegen-units = 1   # Reduce number of codegen units to increase optimizations.
    panic = 'abort'     # Abort on panic
    strip = true        # Strip symbols from binary*

    Build in release mode using cargo build --release
    */

    // You can also use #![no_std] to remove Rust's libstd. See min-sized-rust for details.

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
