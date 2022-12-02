// @filename   : stackoverflow_0105_29611387.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/29611387
// @title      : Why does the println! function use an exclamation mark in Rust?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        //use std::error::Error;
        #[derive(Debug)]
        enum Error {
            Ok,
            Bad,
        }
        fn foo() -> Result<i32, Error> {
            Ok(4)
        }

        fn bar() -> Result<i32, Error> {
            let a = foo()?;
            Ok(a + 4)
        }
        pub fn test() {
            // add your code here
            match foo() {
                Ok(i) => println!("OK={i}"),
                Err(e) => println!("Err={:?}", e),
            };
            match bar() {
                Ok(i) => println!("OK={i}"),
                Err(e) => println!("Err={:?}", e),
            };
        }
    }
    mod code2 {
        fn foo() -> Option<i32> {
            None
        }

        fn bar() -> Option<i32> {
            let a = foo()?;
            Some(a + 4)
        }
        pub fn test() {
            // add your code here
            foo();
            bar();
        }
    }
    mod code3 {
        pub fn test() {
            // add your code here
        }
    }
    pub fn test() {
        code1::test();
        code2::test();
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
    answer1::test();
    //answer2::test();
    //answer3::test();
    _leave!();
}
