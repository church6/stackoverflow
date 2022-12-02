// @filename   : stackoverflow_0245_16421033.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/16421033
// @title      : Lazy sequence generation in Rust

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        // Works in stable Rust 1.0 and above

        fn example() {
            let sum: u64 = (0..1_000_000).sum();
            println!("{}", sum)
        }
        pub fn test() {
            // add your code here
            example();
        }
    }
    mod code2 {
        // What if Range didn't exist? We can create an iterator that models it!
        struct MyRange {
            start: u64,
            end: u64,
        }

        impl MyRange {
            fn new(start: u64, end: u64) -> MyRange {
                MyRange { start, end }
            }
        }

        impl Iterator for MyRange {
            type Item = u64;

            fn next(&mut self) -> Option<u64> {
                if self.start == self.end {
                    None
                } else {
                    let result = Some(self.start);
                    self.start += 1;
                    result
                }
            }
        }

        fn example() {
            let sum: u64 = MyRange::new(0, 1_000_000).sum();
            println!("{}", sum)
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
