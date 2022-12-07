// @filename   : stackoverflow_0194_23810032.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/23810032
// @title      : How to specify const array in global scope in Rust?

#[allow(dead_code)]
mod answer1 {
    #[allow(clippy::redundant_static_lifetimes)]
    mod code1 {
        static NUMBERS: &'static [i32] = &[1, 2, 3, 4, 5];

        pub fn test() {
            // add your code here
            println!("{:?}", NUMBERS);
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
        const NUMBERS: &'static [i32] = &[1, 2, 3, 4, 5];
        pub fn test() {
            // add your code here
            println!("{:?}", NUMBERS);
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
