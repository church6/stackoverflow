// @filename   : stackoverflow_0227_29548819.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/29548819
// @title      : How do I sum a vector using fold?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        pub fn test() {
            // add your code here

            // Since Rust 1.11
            // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.sum
            let sum: u32 = vec![1, 2, 3, 4, 5, 6].iter().sum();
            assert_eq!(21u32, sum);
            println!("sum={sum}");
            let sum: u32 = vec![1, 2, 3, 4, 5, 6].iter().fold(0u32, |mut sum, val| {
                sum += val;
                sum
            });
            assert_eq!(21u32, sum);
            println!("sum={sum}");

            let sum: u32 = vec![1, 2, 3, 4, 5, 6].iter().fold(0, |mut sum, &val| {
                sum += val;
                sum
            });
            assert_eq!(21u32, sum);
            println!("sum={sum}");
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
    //answer1::test();
    //answer2::test();
    //answer3::test();
    _leave!();
}
