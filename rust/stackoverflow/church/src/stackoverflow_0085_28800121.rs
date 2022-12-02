// @filename   : stackoverflow_0085_28800121.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/28800121
// @title      : What do I have to do to solve a "use of moved value" error?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        fn vector_is_prime(num: i64) -> bool {
            let stop = ((num as f64).sqrt() + 1.0) as i64;
            for i in 3..stop {
                if i % 2 != 0 && num % i == 0 {
                    return false;
                }
            }
            true
        }
        const PRIME25: [i64; 25] = [
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97,
        ];
        pub fn test() {
            // add your code here

            let mut count: u32 = 1;
            let mut num: i64 = 1;
            let mut primes: Vec<i64> = Vec::new();
            primes.push(2);

            while count < 25 {
                num += 1;
                if num % 2 != 0 && vector_is_prime(num) {
                    count += 1;
                    primes.push(num);
                }
            }
            println!("{:?}", primes);
            assert_eq!(primes, PRIME25);
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
