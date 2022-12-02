// @filename   : stackoverflow_0065_29530011.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/29530011
// @title      : Creating a vector of zeros for a specific size

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        pub fn test() {
            // add your code here
            let len = 10;
            let zero_vec = vec![0; len];
            println!("{:?}", zero_vec);
        }
    }
    #[allow(unused_variables)]
    mod code2 {
        fn zeros(size: u32) -> Vec<i32> {
            let mut zero_vec: Vec<i32> = Vec::with_capacity(size as usize);
            for i in 0..size {
                zero_vec.push(0);
            }
            /*return*/
            zero_vec
        }
        pub fn test() {
            // add your code here
            let zero_vec = zeros(10);
            println!("{:?}", zero_vec);
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
        // vec! is more efficient
        fn zeros(size: u32) -> Vec<i32> {
            vec![0; size as usize]
        }
        pub fn test() {
            // add your code here
            let vec = zeros(10);
            for i in vec.iter() {
                println!("{}", i)
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
        use std::iter;

        fn zeros(size: usize) -> Vec<i32> {
            iter::repeat(0).take(size).collect()
        }
        pub fn test() {
            // add your code here
            let zero_vec = zeros(10);
            println!("{:?}", zero_vec);
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
