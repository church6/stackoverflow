// @filename   : stackoverflow_0020_57458460.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/57458460
// @title      : Why is there a large performance impact when looping over an array with 240 or more elements?

#[allow(dead_code)]
mod answer1 {
    #[allow(clippy::needless_range_loop)]
    mod code1 {
        use std::time::Instant;
        pub fn test() {
            // add your code here
            const CAPACITY: usize = 240;
            const IN_LOOPS: usize = 500000;

            let mut arr = [0; CAPACITY];
            for i in 0..CAPACITY {
                arr[i] = i;
            }
            let mut sum = 0;
            let now = Instant::now();
            for _ in 0..IN_LOOPS {
                let mut s = 0;
                for i in 0..arr.len() {
                    s += arr[i];
                }
                sum += s;
            }
            println!("sum:{} time:{:?}", sum, now.elapsed());
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
    #[allow(clippy::needless_range_loop)]
    mod code1 {
        use std::time::Instant;
        const CAPACITY: usize = 240;
        const IN_LOOPS: usize = 500000;

        pub fn bar() -> usize {
            (0..CAPACITY).sum::<usize>() * IN_LOOPS
        }

        pub fn test() {
            // add your code here
            let mut arr = [0; CAPACITY];
            for i in 0..CAPACITY {
                arr[i] = i;
            }
            let now = Instant::now();
            let sum = bar();
            println!("sum:{} time:{:?}", sum, now.elapsed());
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
