// @filename   : stackoverflow_0304_51121446.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/51121446
// @title      : How do I assert an enum is a specific variant if I don't care about its fields?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        enum MyEnum {
            WithoutFields,
            WithFields { field: String },
        }

        fn return_with_fields() -> MyEnum {
            MyEnum::WithFields {
                field: "some string".into(),
            }
        }

        // Rust 1.42
        // You can use std::matches:
        #[test]
        fn example() {
            assert!(matches!(return_with_fields(), MyEnum::WithFields { .. }));
        }

        pub fn test() {
            // add your code here
        }
    }
    #[allow(unused_macros)]
    mod code2 {
        enum MyEnum {
            WithoutFields,
            WithFields { field: String },
        }

        fn return_with_fields() -> MyEnum {
            MyEnum::WithFields {
                field: "some string".into(),
            }
        }

        // Previous versions
        // Your original code can be made to work with a new macro:
        macro_rules! is_enum_variant {
            ($v:expr, $p:pat) => {
                if let $p = $v {
                    true
                } else {
                    false
                }
            };
        }

        #[test]
        fn example() {
            assert!(is_enum_variant!(
                return_with_fields(),
                MyEnum::WithFields { .. }
            ));
        }

        pub fn test() {
            // add your code here
        }
    }
    mod code3 {
        enum MyEnum {
            WithoutFields,
            WithFields { field: String },
        }

        fn return_with_fields() -> MyEnum {
            MyEnum::WithFields {
                field: "some string".into(),
            }
        }
        // Personally, I tend to add methods to my enums:
        impl MyEnum {
            fn is_with_fields(&self) -> bool {
                matches!(self, MyEnum::WithFields { .. })
                /*match self {
                    MyEnum::WithFields { .. } => true,
                    _ => false,
                }*/
            }
        }
        #[test]
        fn example() {
            assert!(MyEnum::is_with_fields(&return_with_fields()));
        }

        pub fn test() {
            // add your code here
        }
    }
    mod code4 {
        // I also tend to avoid struct-like enums and instead put in extra work:

        enum MyEnum {
            WithoutFields,
            WithFields(WithFields),
        }

        struct WithFields {
            field: String,
        }

        impl MyEnum {
            fn is_with_fields(&self) -> bool {
                matches!(self, MyEnum::WithFields { .. })
                /*match self {
                    MyEnum::WithFields(_) => true,
                    _ => false,
                }*/
            }

            fn as_with_fields(&self) -> Option<&WithFields> {
                match self {
                    MyEnum::WithFields(x) => Some(x),
                    _ => None,
                }
            }

            fn into_with_fields(self) -> Option<WithFields> {
                match self {
                    MyEnum::WithFields(x) => Some(x),
                    _ => None,
                }
            }
        }
        // I hope that some day, enum variants can be made into their own type to avoid this extra struct.
        #[test]
        fn example() {
            let f = WithFields {
                field: "some string".into(),
            };
            let e = MyEnum::WithFields(f);
            assert!(MyEnum::is_with_fields(&e));
        }
        pub fn test() {
            // add your code here
        }
    }
    pub fn test() {
        //code1::test();
        //code2::test();
        //code3::test();
        //code4::test();
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
    //answer1::test();
    //answer2::test();
    //answer3::test();
    _leave!();
}
