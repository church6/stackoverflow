// @filename   : stackoverflow_0255_27459640.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/27459640
// @title      : How to create a static array of strings?

#[allow(dead_code)]
mod answer1 {
    #[allow(clippy::redundant_static_lifetimes)]
    mod code1 {
        const BROWSERS: &'static [&'static str] = &["firefox", "chrome"];
        pub fn test() {
            // add your code here
            // BROWSERS[0] = "Firefox";
            println!("{:?}", BROWSERS);
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
        code1::test();
        //code2::test();
        //code3::test();
    }
}
#[allow(dead_code)]
mod answer2 {
    #[allow(clippy::redundant_static_lifetimes)]
    mod code1 {
        pub fn test() {
            // add your code here

            const STRHELLO: &'static str = "Hello";
            const STRWORLD: &'static str = "World";
            const ARR: [&'static str; 2] = [STRHELLO, STRWORLD];
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
        code1::test();
        //code2::test();
        //code3::test();
    }
}
#[allow(dead_code)]
mod answer3 {
    mod code1 {
        const ONETWO: [u8; 2] = [1, 2];
        const ARRAY: [&str; 2] = ["Hello", "World"];

        fn main() {
            println!("{} {}", ONETWO[0], ONETWO[1]); // 1 2
            println!("{} {}", ARRAY[0], ARRAY[1]); // Hello World
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
        code1::test();
        //code2::test();
        //code3::test();
    }
}
pub fn test() {
    _enter!();
    answer1::test();
    answer2::test();
    answer3::test();
    _leave!();
}
