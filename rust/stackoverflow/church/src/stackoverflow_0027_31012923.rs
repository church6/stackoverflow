// @filename   : stackoverflow_0027_31012923.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/31012923
// @title      : What is the difference between Copy and Clone?

#[allow(dead_code)]
mod answer1 {
    /*
    Clone is designed for arbitrary duplications: a Clone implementation for a type T can do arbitrarily complicated operations required to create a new T. It is a normal trait (other than being in the prelude), and so requires being used like a normal trait, with method calls, etc.

    The Copy trait represents values that can be safely duplicated via memcpy: things like reassignments and passing an argument by-value to a function are always memcpys, and so for Copy types, the compiler understands that it doesn't need to consider those a move.
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
    // Copy is implicit, inexpensive, and cannot be re-implemented (memcpy).
    // Clone is explicit, may be expensive, and may be re-implement arbitrarily.
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
