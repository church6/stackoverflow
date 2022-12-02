// @filename   : stackoverflow_0299_24245276.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/24245276
// @title      : Why does Rust not have a return value in the main function, and how to return a value anyway?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        // As of Rust 1.26, main can return a Result:

        use std::fs::File;

        fn example1() -> Result<(), std::io::Error> {
            println!("main");
            let _f = File::open("bar.txt")?;
            Ok(())
        }

        // The returned error code in this case is 1 in case of an error. With File::open("bar.txt").expect("file not found"); instead, an error value of 101 is returned (at least on my machine).

        // Also, if you want to return a more generic error, use:

        use std::error::Error;

        fn example2() -> Result<(), Box<dyn Error>> {
            println!("main");
            Ok(())
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
