// @filename   : stackoverflow_0185_32554285.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/32554285
// @title      : Compare enums only by variant, not value

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        struct Add(u8);
        struct Sub(u8);

        enum Op {
            Add(Add),
            Sub(Sub),
        }

        // As of Rust 1.21.0, you can use std::mem::discriminant:
        fn variant_eq(a: &Op, b: &Op) -> bool {
            std::mem::discriminant(a) == std::mem::discriminant(b)
        }

        // This is nice because it can be very generic:
        /*
        fn variant_eq<T>(a: &T, b: &T) -> bool {
            std::mem::discriminant(a) == std::mem::discriminant(b)
        }
        */

        fn example() {
            let a = Op::Add(Add(42));
            let b = Op::Add(Add(42));
            assert!(variant_eq(&a, &b));
        }
        pub fn test() {
            // add your code here
            example();
        }
    }
    mod code2 {
        struct Add(u8);
        struct Sub(u8);

        enum Op {
            Add(Add),
            Sub(Sub),
        }

        #[allow(clippy::match_like_matches_macro)]
        fn variant_eq(a: &Op, b: &Op) -> bool {
            match (a, b) {
                (&Op::Add(..), &Op::Add(..)) => true,
                (&Op::Sub(..), &Op::Sub(..)) => true,
                _ => false,
            }
        }
        // matches!((a, b), (&Op::Add(..), &Op::Add(..)) | (&Op::Sub(..), &Op::Sub(..)))

        fn example() {
            let a = Op::Add(Add(42));
            let b = Op::Add(Add(42));
            let c = Op::Add(Add(21));
            let d = Op::Sub(Sub(42));

            println!("{}", variant_eq(&a, &b));
            println!("{}", variant_eq(&a, &c));
            println!("{}", variant_eq(&a, &d));
        }
        pub fn test() {
            // add your code here
            example();
        }
    }
    mod code3 {
        struct Add(u8);
        struct Sub(u8);

        macro_rules! foo {
        (enum $name:ident {
            $($vname:ident($inner:ty),)*
        }) => {
            enum $name {
                 $($vname($inner),)*
            }

            impl $name {
                fn variant_eq(&self, b: &Self) -> bool {
                    match (self, b) {
                        $((&$name::$vname(..), &$name::$vname(..)) => true,)*
                        _ => false,
                    }
                }
            }
        }
    }

        foo! {
            enum Op {
                Add(Add),
                Sub(Sub),
            }
        }

        fn example() {
            let a = Op::Add(Add(42));

            let b = Op::Add(Add(42));
            let c = Op::Add(Add(21));
            let d = Op::Sub(Sub(42));

            println!("{}", Op::variant_eq(&a, &b));
            println!("{}", Op::variant_eq(&a, &c));
            println!("{}", Op::variant_eq(&a, &d));
        }
        pub fn test() {
            // add your code here
            example();
        }
    }
    pub fn test() {
        code1::test();
        code2::test();
        code3::test();
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
