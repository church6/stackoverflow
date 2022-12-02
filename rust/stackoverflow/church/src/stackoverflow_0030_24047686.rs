// @filename   : stackoverflow_0030_24047686.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/24047686
// @title      : Default function arguments in Rust

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        fn add(a: Option<i32>, b: Option<i32>) -> i32 {
            a.unwrap_or(1) + b.unwrap_or(2)
        }
        pub fn test() {
            // add your code here
            assert_eq!(add(Some(2), Some(3)), 5);
            assert_eq!(add(None, None), 3);
        }
    }
    mod code2 {
        fn add(a: i32, b: i32) -> i32 {
            a + b
        }

        macro_rules! add {
            ($a: expr) => {
                add($a, 2)
            };
            () => {
                add(1, 2)
            };
        }
        pub fn test() {
            // add your code here
            assert_eq!(add!(), 3);
            assert_eq!(add!(4), 6);
        }
    }
    // #[warn(clippy::redundant_field_names)]
    mod code3 {
        pub struct FooArgs {
            a: f64,
            b: i32,
        }

        impl Default for FooArgs {
            fn default() -> Self {
                FooArgs { a: 1.0, b: 1 }
            }
        }

        impl From<()> for FooArgs {
            fn from(_: ()) -> Self {
                Self::default()
            }
        }

        impl From<f64> for FooArgs {
            fn from(a: f64) -> Self {
                Self {
                    a,
                    ..Self::default()
                }
            }
        }

        impl From<i32> for FooArgs {
            fn from(b: i32) -> Self {
                Self {
                    b,
                    ..Self::default()
                }
            }
        }

        impl From<(f64, i32)> for FooArgs {
            fn from((a, b): (f64, i32)) -> Self {
                Self { a, b }
            }
        }

        pub fn foo<A>(arg_like: A) -> f64
        where
            A: Into<FooArgs>,
        {
            let args = arg_like.into();
            args.a * (args.b as f64)
        }
        pub fn test() {
            // add your code here
            println!("{}", foo(()));
            println!("{}", foo(5.0));
            println!("{}", foo(-3));
            println!("{}", foo((2.0, 6)));
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
        use std::default::Default;

        #[derive(Debug)]
        pub struct Sample {
            a: u32,
            b: u32,
            c: u32,
        }

        impl Default for Sample {
            fn default() -> Self {
                Sample { a: 2, b: 4, c: 6 }
            }
        }
        pub fn test() {
            // add your code here
            let s = Sample {
                c: 23,
                ..Sample::default()
            };
            println!("{:?}", s);
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
