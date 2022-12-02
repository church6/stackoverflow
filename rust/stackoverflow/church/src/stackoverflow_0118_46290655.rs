// @filename   : stackoverflow_0118_46290655.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/46290655
// @title      : Get the String length in characters in Rust

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        fn example() {
            println!("{}", "é".chars().count()); // 2
            println!("{}", "é".chars().count()); // 1
        }
        pub fn test() {
            // add your code here
            example();
        }
    }
    mod code2 {
        use unicode_segmentation::UnicodeSegmentation;

        fn example() {
            println!("{}", "é".graphemes(true).count()); // 1
            println!("{}", "é".graphemes(true).count()); // 1
            println!("{}", "é".graphemes(true).count()); // 2
        }
        pub fn test() {
            // add your code here
            example();
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
