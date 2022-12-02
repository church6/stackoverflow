// @filename   : stackoverflow_0006_21747136.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/21747136
// @title      : How do I print in Rust the type of a variable?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        fn print_type_of<T>(_: &T) {
            println!("{}", std::any::type_name::<T>())
        }
        pub fn test() {
            // add your code here
            let s = "Hello";
            let i = 42;

            print_type_of(&s); // &str
            print_type_of(&i); // i32
            print_type_of(&test); // playground::main
            print_type_of(&print_type_of::<i32>); // playground::print_type_of<i32>
            print_type_of(&{ || "Hi!" }); // playground::main::{{closure}}
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
        fn print_type_of<T>(_: &T) {
            println!("{}", std::any::type_name::<T>());
        }
        pub fn test() {
            // add your code here
            print_type_of(&32.90); // prints "f64"
            print_type_of(&vec![1, 2, 4]); // prints "std::vec::Vec<i32>"
            print_type_of(&"foo"); // prints "&str"
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
        /*
                trait TypeInfo {
                    fn type_of(&self) -> &'static str;
                }

                impl TypeInfo for i32 {
                    fn type_of(&self) -> &'static str {
                        "i32"
                    }
                }

                impl TypeInfo for i64 {
                    fn type_of(&self) -> &'static str {
                        "i64"
                    }
                }
        */
        trait TypeInfo {
            fn type_name() -> String;
            fn type_of(&self) -> String;
        }

        macro_rules! impl_type_info {
    ($($name:ident$(<$($T:ident),+>)*),*) => {
        $(impl_type_info_single!($name$(<$($T),*>)*);)*
    };
}

        macro_rules! mut_if {
            ($name:ident = $value:expr, $($any:expr)+) => {
                let mut $name = $value;
            };
            ($name:ident = $value:expr,) => {
                let $name = $value;
            };
        }

        macro_rules! impl_type_info_single {
    ($name:ident$(<$($T:ident),+>)*) => {
        impl$(<$($T: TypeInfo),*>)* TypeInfo for $name$(<$($T),*>)* {
            fn type_name() -> String {
                mut_if!(res = String::from(stringify!($name)), $($($T)*)*);
                $(
                    res.push('<');
                    $(
                        res.push_str(&$T::type_name());
                        res.push(',');
                    )*
                    res.pop();
                    res.push('>');
                )*
                res
            }
            fn type_of(&self) -> String {
                $name$(::<$($T),*>)*::type_name()
            }
        }
    }
}

        impl<'a, T: TypeInfo + ?Sized> TypeInfo for &'a T {
            fn type_name() -> String {
                let mut res = String::from("&");
                res.push_str(&T::type_name());
                res
            }
            fn type_of(&self) -> String {
                <&T>::type_name()
            }
        }

        impl<'a, T: TypeInfo + ?Sized> TypeInfo for &'a mut T {
            fn type_name() -> String {
                let mut res = String::from("&mut ");
                res.push_str(&T::type_name());
                res
            }
            fn type_of(&self) -> String {
                <&mut T>::type_name()
            }
        }

        macro_rules! type_of {
            ($x:expr) => {
                (&$x).type_of()
            };
        }
        impl_type_info!(i32, i64, f32, f64, str, String, Vec<T>, Result<T,S>);
        pub fn test() {
            // add your code here
            println!("{}", type_of!(1));
            println!("{}", type_of!(&1));
            println!("{}", type_of!(&&1));
            println!("{}", type_of!(&mut 1));
            println!("{}", type_of!(&&mut 1));
            println!("{}", type_of!(&mut &1));
            println!("{}", type_of!(1.0));
            println!("{}", type_of!("abc"));
            println!("{}", type_of!(&"abc"));
            println!("{}", type_of!(String::from("abc")));
            println!("{}", type_of!(vec![1, 2, 3]));

            println!("{}", <Result<String, i64>>::type_name());
            println!("{}", <&i32>::type_name());
            println!("{}", <&str>::type_name());
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
pub fn test() {
    _enter!();
    answer1::test();
    answer2::test();
    answer3::test();
    _leave!();
}
