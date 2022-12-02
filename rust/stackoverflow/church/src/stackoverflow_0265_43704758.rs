// @filename   : stackoverflow_0265_43704758.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/43704758
// @title      : How to idiomatically convert between u32 and usize?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        pub fn test() {
            // add your code here
            let s = "abc";
            let n: u32 = 1;
            let ch = s.chars().nth(n as usize).unwrap();
            println!("{ch}");
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
        use std::convert::TryFrom;

        fn example() {
            let s = "abc";
            let n: u32 = 1;
            let n_us = usize::try_from(n).unwrap();
            let ch = s.chars().nth(n_us).unwrap();
            println!("{}", ch);
        }

        pub fn test() {
            // add your code here
            example();
        }
    }
    mod code2 {
        fn example() {
            let n: u32 = 0x1_FF_FF;
            // Pretend that `usize` is 16-bit
            let n_us: u16 = n as u16;

            println!("{}, {}", n, n_us); // 131071, 65535
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
