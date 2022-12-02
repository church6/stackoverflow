// @filename   : stackoverflow_0036_15619320.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/15619320
// @title      : How can I access command line parameters in Rust?

#[allow(dead_code)]
mod answer1 {
    // clap: you describe the options you want to parse using a fluent API. Faster than docopt and  gives you more control.
    // getopts: port of the popular C library. Lower-level and even more control.

    mod code1 {
        use std::env;
        pub fn test() {
            // add your code here
            let args: Vec<_> = env::args().collect();
            if args.len() > 1 {
                println!("The first argument is {}", args[1]);
            }
        }
    }
    mod code2 {
        use std::env;
        pub fn test() {
            // add your code here
            if let Some(arg1) = env::args().nth(1) {
                println!("The first argument is {}", arg1);
            }
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
