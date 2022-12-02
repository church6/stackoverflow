// @filename   : stackoverflow_0215_37506672.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/37506672
// @title      : Convert float to integer in Rust

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        fn example1() {
            let a = 123_456.7f64;
            let b = a / 100_000.0; // underscore in number to increase readability
            let b = b as i64;
            let b = b * 100_000;
            println!("{a} to {b}");
            assert_eq!(100_000i64, b);
        }
        // Which, of course, can be written in one line, too:
        fn example2() {
            let a = 123_456.7f64;
            let b = ((a / 100_000.0) as i64) * 100_000;
            println!("{a} to {b}");
            assert_eq!(100_000i64, b);
        }

        // If you wanted to round instead of just taking the integer part, you can use the round method of f64:
        fn example3() {
            let a = 123_456.7f64;
            let b = ((a / 100_000.0).round() as i64) * 100_000;
            println!("{a} to {b}");
            assert_eq!(100_000i64, b);
        }

        pub fn test() {
            // add your code here
            example1();
            example2();
            example3();
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
