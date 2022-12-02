// @filename   : stackoverflow_0272_41207885.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/41207885
// @title      : Using generic trait methods like .into() when type inference is impossible

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        // You could use From::from:
        // https://doc.rust-lang.org/stable/std/convert/index.html
        use std::convert::*;

        struct NewType(pub i32);

        impl From<NewType> for i32 {
            fn from(src: NewType) -> i32 {
                src.0
            }
        }

        fn example() {
            let a = NewType(5);
            println!("{}", i32::from(a));
        }

        pub fn test() {
            // add your code here
            example();
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
        //#![feature(type_ascription)]
        use std::convert::*;

        #[derive(Debug)]
        struct NewType(pub i32);

        impl From<NewType> for i32 {
            fn from(src: NewType) -> i32 {
                src.0
            }
        }

        fn example() {
            let a = NewType(5);
            // type ascription is experimental
            // println!("{}", a.into(): i32);
            println!("{:?}", a);
        }

        pub fn test() {
            // add your code here
            example();
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
