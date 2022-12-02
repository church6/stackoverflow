// @filename   : stackoverflow_0273_27589054.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/27589054
// @title      : What is the correct way to use lifetimes with a struct in Rust?

#[allow(dead_code)]
mod answer1 {
    #[allow(clippy::redundant_field_names)]
    mod code1 {
        struct C;

        struct B<'b> {
            c: &'b C,
        }

        struct A<'a> {
            b: B<'a>,
            c: &'a C,
        }

        fn example() {
            let c1 = C;
            let _ = A::new(&c1);
        }

        impl<'a> A<'a> {
            fn new(c: &'a C) -> A<'a> {
                A {
                    c: c,
                    b: B { c: c },
                }
            }
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
