// @filename   : stackoverflow_0317_21669722.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/21669722
// @title      : What is the simplest way to convert a string to upper case in Rust?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        fn example() {
            let test_str = "übercode"; // type &str

            let uppercase_test_string = test_str.to_uppercase(); // type String

            let uppercase_test_str = uppercase_test_string.as_str(); // back to type &str

            println! {"{}", test_str};
            println! {"{}", uppercase_test_string};
            println! {"{}", uppercase_test_str};
        }
        pub fn test() {
            // add your code here
            example();
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
        fn example() {
            let r = "smash".to_ascii_uppercase();
            println!("Hulk {}!", r); // Hulk SMASH!

            //or one liner
            println!("Hulk {}!", "smash".to_ascii_uppercase());
        }
        pub fn test() {
            // add your code here
            example();
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
        // In Rust 1.2.0, str::to_uppercase() was added.

        fn example() {
            let s = "smash";
            println!("Hulk {}", s.to_uppercase());
        }
        pub fn test() {
            // add your code here
            example();
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
    answer2::test();
    answer3::test();
    _leave!();
}
