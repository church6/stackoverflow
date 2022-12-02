// @filename   : stackoverflow_0232_30186037.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/30186037
// @title      : How can I read a single line from stdin?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        use std::io::{self, BufRead};

        fn example1() {
            let stdin = io::stdin();
            for line in stdin.lock().lines() {
                println!("{}", line.unwrap());
            }
        }

        //use std::io::{self, BufRead};
        fn example2() {
            let stdin = io::stdin();
            let mut iterator = stdin.lock().lines();
            let _line1 = iterator.next().unwrap().unwrap();
            let _line2 = iterator.next().unwrap().unwrap();
        }

        // use std::io::{self, BufRead};
        fn example3() {
            let stdin = io::stdin();
            let _line1 = stdin.lock().lines().next().unwrap().unwrap();
        }

        pub fn test() {
            // add your code here
            example1();
            example2();
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
