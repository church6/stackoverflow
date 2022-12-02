// @filename   : stackoverflow_0182_29490670.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/29490670
// @title      : How does Rust provide move semantics?

#[allow(dead_code)]
mod answer1 {
    // Rust doesn't have constructors at all, let alone move constructors.
    /*
    You do not need move constructors. Rust moves everything that "does not have a copy constructor", a.k.a. "does not implement the Copy trait".
    */

    mod code1 {
        //#[derive(Debug)]
        struct A(i32);
        impl A {
            fn new() -> A {
                A(5)
            }
        }

        fn example() {
            let a = A;
            //println!("{:?}", a);
            let _b = a;
            //println!("{:?}", b);
            let _c = a; // error, a is moved
        }
        pub fn test() {
            // add your code here
            example();
        }
    }
    mod code2 {
        trait Ref {
            fn test(&self);
        }

        trait Move {
            fn test(self);
        }

        struct A;
        impl Ref for A {
            fn test(&self) {
                println!("by ref");
            }
        }
        impl Move for A {
            fn test(self) {
                println!("by value");
            }
        }
        fn example() {
            let a = A;
            #[allow(clippy::needless_borrow)]
            (&a).test(); // prints "by ref"
            a.test(); // prints "by value"
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
