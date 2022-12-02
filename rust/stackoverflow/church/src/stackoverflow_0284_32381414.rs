// @filename   : stackoverflow_0284_32381414.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/32381414
// @title      : Converting a hexadecimal string to a decimal integer

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        use std::i64;

        fn example() {
            let z = i64::from_str_radix("1f", 16);
            println!("{:?}", z);
        }

        pub fn test() {
            // add your code here
            example();
        }
    }
    mod code2 {
        use std::i64;

        fn example() {
            let raw = "0x1f";
            let without_prefix = raw.trim_start_matches("0x");
            let z = i64::from_str_radix(without_prefix, 16);
            println!("{:?}", z);
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
