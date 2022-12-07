// @filename   : stackoverflow_0181_25576748.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/25576748
// @title      : How to compare enum without pattern matching

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        #[derive(Debug, PartialEq)]
        enum MyEnum {
            Enum1 = 0,
            Enum2,
            Enum3,
        }

        #[derive(Debug)]
        struct MyStruct {
            my_enum: MyEnum,
            my_data: i32,
        }

        fn example() {
            let my_struct = MyStruct {
                my_enum: MyEnum::Enum3,
                my_data: 1i32,
            };
            assert!(matches!(my_struct.my_enum, MyEnum::Enum3));
            println!("{:?}", my_struct);
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
        #[derive(Debug)]
        enum Thing {
            One(i32),
            Two(String),
            Unknown,
        }

        impl Thing {
            #[allow(clippy::match_like_matches_macro)]
            fn is_unknown(&self) -> bool {
                match *self {
                    Thing::Unknown => true,
                    _ => false,
                }
            }
        }

        fn example() {
            let things = [Thing::One(42), Thing::Two("hello".into()), Thing::Unknown];
            for t in things.iter().filter(|s| !s.is_unknown()) {
                println!("{:?}", t);
            }
        }
        pub fn test() {
            // add your code here
            example();
        }
    }
    mod code2 {
        #[derive(Debug)]
        enum Thing {
            One(i32),
            Two(String),
            Unknown,
        }

        impl Thing {
            /*
                fn is_unknown(&self) -> bool {
                    match *self {
                        Thing::Unknown => true,
                        _ => false,
                    }
                }
            */
            fn is_unknown(&self) -> bool {
                matches!(self, Thing::Unknown)
            }
        }

        fn example() {
            let things = [Thing::One(42), Thing::Two("hello".into()), Thing::Unknown];
            for t in things.iter().filter(|s| !s.is_unknown()) {
                println!("{:?}", t);
            }
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
