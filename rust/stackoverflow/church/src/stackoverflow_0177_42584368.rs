// @filename   : stackoverflow_0177_42584368.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/42584368
// @title      : How do you define custom `Error` types in Rust?

#[allow(dead_code)]
mod answer1 {
    /*
    pub trait Error: Debug + Display {
        fn description(&self) -> &str {  }
        fn cause(&self) -> Option<&Error> {  }
        fn source(&self) -> Option<&(Error + 'static)> { }
    }*/

    mod code1 {
        use std::{error::Error, fmt};

        #[derive(Debug)]
        struct Thing;

        impl Error for Thing {}

        impl fmt::Display for Thing {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "Oh no, something bad went down")
            }
        }
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