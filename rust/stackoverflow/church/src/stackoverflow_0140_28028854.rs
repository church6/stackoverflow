// @filename   : stackoverflow_0140_28028854.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/28028854
// @title      : How do I match enum values with an integer?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        // Since Rust 1.34, I recommend implementing TryFrom:
        use std::convert::TryFrom;

        enum MyEnum {
            A = 1,
            B,
            C,
        }

        impl TryFrom<i32> for MyEnum {
            type Error = ();

            fn try_from(v: i32) -> Result<Self, Self::Error> {
                match v {
                    x if x == MyEnum::A as i32 => Ok(MyEnum::A),
                    x if x == MyEnum::B as i32 => Ok(MyEnum::B),
                    x if x == MyEnum::C as i32 => Ok(MyEnum::C),
                    _ => Err(()),
                }
            }
        }

        //Then you can use TryInto and handle the possible error:
        use std::convert::TryInto;

        fn example() {
            let x = MyEnum::C as i32;

            match x.try_into() {
                Ok(MyEnum::A) => println!("a"),
                Ok(MyEnum::B) => println!("b"),
                Ok(MyEnum::C) => println!("c"),
                Err(_) => eprintln!("unknown number"),
            }
        }

        pub fn test() {
            // add your code here
        }
    }
    mod code2 {
        macro_rules! back_to_enum {
    ($(#[$meta:meta])* $vis:vis enum $name:ident {
        $($(#[$vmeta:meta])* $vname:ident $(= $val:expr)?,)*
    }) => {
        $(#[$meta])*
        $vis enum $name {
            $($(#[$vmeta])* $vname $(= $val)?,)*
        }

        impl std::convert::TryFrom<i32> for $name {
            type Error = ();

            fn try_from(v: i32) -> Result<Self, Self::Error> {
                match v {
                    $(x if x == $name::$vname as i32 => Ok($name::$vname),)*
                    _ => Err(()),
                }
            }
        }
    }
}

        back_to_enum! {
            enum MyEnum {
                A = 1,
                B,
                C,
            }
        }

        fn example() {
            let x = MyEnum::C as i32;

            match x.try_into() {
                Ok(MyEnum::A) => println!("a"),
                Ok(MyEnum::B) => println!("b"),
                Ok(MyEnum::C) => println!("c"),
                Err(_) => eprintln!("unknown number"),
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
mod answer2 {
    mod code1 {
        use num_derive::FromPrimitive;
        use num_traits::FromPrimitive;

        #[derive(FromPrimitive)]
        enum MyEnum {
            A = 1,
            B,
            C,
        }

        fn example() {
            let x = 2;

            match FromPrimitive::from_i32(x) {
                Some(MyEnum::A) => println!("Got A"),
                Some(MyEnum::B) => println!("Got B"),
                Some(MyEnum::C) => println!("Got C"),
                None => println!("Couldn't convert {}", x),
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
