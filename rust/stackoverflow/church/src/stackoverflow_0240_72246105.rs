// @filename   : stackoverflow_0240_72246105.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/72246105
// @title      : Why does std::fs::write(...) use an inner function?

#[allow(dead_code)]
mod answer1 {
    // Monomorphization costs.

    /*
    write(), like most filesystem functions in Rust, takes AsRef<Path> instead of Path, for convenience (to allow you to pass e.g. a &str). But that also has a cost: it means that the function will be monomorphized and optimized separately for each type, while there is no real need for that. While it is very likely that LLVM will deduplicate all those instances, the time used for optimizing them is still wasted compile time.

    To mitigate this cost, it calls an inner, non-generic function that does all the heavy lifting. The outer function contains only the necessarily-generic code - the conversion to Path.
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
