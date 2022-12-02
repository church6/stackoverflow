// @filename   : stackoverflow_0051_24990520.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/24990520
// @title      : How do I convert from an integer to a string?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        pub fn test() {
            // add your code here
            let x: u32 = 10;
            let s: String = x.to_string();
            println!("{}", s);
        }
    }
    mod code2 {
        pub fn test() {
            // add your code here
            let s: String = (10).to_string();
            let ss: &str = &s; // specifying type is necessary for deref coercion to fire
            println!("{}", ss);
            let ss = &s[..]; // alternatively, use slicing syntax
            println!("{}", ss);
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
