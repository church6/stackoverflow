// @filename   : stackoverflow_0068_22118221.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/22118221
// @title      : How do you iterate over a string by character

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        pub fn test() {
            // add your code here
            let my_str = "my_str";
            for c in my_str.chars() {
                // do something with `c`
                println!("{}", c);
            }
            let my_str = "my_str";
            for (i, c) in my_str.chars().enumerate() {
                // do something with character `c` and index `i`
                println!("[{}]{}", i, c);
            }
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
            let s = "Rust is a programming language";
            for i in s.chars() {
                print!("{}", i);
            }
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
    answer2::test();
    //answer3::test();
    _leave!();
}
