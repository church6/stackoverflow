// @filename   : stackoverflow_0138_43079077.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/43079077
// @title      : Proper way to return a new string in Rust

#[allow(dead_code)]
mod answer1 {
    // You cannot return a &str if you've allocated the String in the function.
    // Strings are heap-allocated and built to be mutable.
    mod code1 {
        #[allow(clippy::needless_return)]
        fn hello_string(x: &str) -> &str {
            println!("{x}");
            return "hello world";
        }
        fn long_string(x: &str) -> &str {
            println!("{x}");
            if x.len() > 10 {
                "too long"
            } else {
                x
            }
        }
        /*
        fn hello_string(x: &str) -> &str {
            &String::from("hello world")
        }*/

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
