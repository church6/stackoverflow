// @filename   : stackoverflow_0108_51208703.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/51208703
// @title      : How to raise a number to a power?

#[allow(dead_code)]
mod answer1 {
    #[allow(clippy::eq_op)]
    mod code1 {
        pub fn test() {
            // add your code here
            let base: i32 = 2; // an explicit type is required
            assert_eq!(base.pow(10), 1024);

            {
                let a = 2; // Can also explicitly define type i.e. i32
                let a = i32::pow(a, 10);
                assert_eq!(a, 1024);
            }

            {
                // integers
                let n = u32::pow(2, 10);
                println!("{}", n == 1024);
                assert_eq!(n, 1024);
            }

            {
                // floats
                // example 1
                let f = f32::powf(2.0, 10.0);
                // example 2
                let g = f32::powi(2.0, 10);
                // print
                println!("{}", f == 1024.0 && g == 1024.0);
            }

            {
                let n = 2 << 9;
                println!("{}", n == 1024);
                assert_eq!(n, 1024);
            }

            {
                let n = 2u32.pow(10);
                assert_eq!(n, 1024);
            }

            {
                assert_eq!(1 << 10, 1024);
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
