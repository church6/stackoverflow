// @filename   : stackoverflow_0276_31358826.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/31358826
// @title      : How do I convert an enum reference to a number?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        #[derive(Copy, Clone)]
        enum Foo {
            Bar = 1,
        }

        fn f_a(fool: &Foo) -> u8 {
            *fool as u8
        }

        fn f_q(fool: &Foo) {
            //let f = foo as u8;  // error[E0606]: casting `&Foo` as `u8` is invalid
            //let f = foo as &u8; // error[E0605]: non-primitive cast: `&Foo` as `&u8`
            //let f = *foo as u8; // error[E0507]: cannot move out of borrowed content
            println!("{}", *fool as u8);
        }

        fn example() {
            let s = Foo::Bar;
            f_q(&s);
            let s = Foo::Bar;
            println!("{}", f_a(&s));
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
