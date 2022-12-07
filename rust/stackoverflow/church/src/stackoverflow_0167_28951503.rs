// @filename   : stackoverflow_0167_28951503.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/28951503
// @title      : How can I create a function with a variable number of arguments?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        fn foo(args: &[&str]) {
            for arg in args {
                println!("{}", arg);
            }
        }

        fn example() {
            foo(&["hello", "world", "I", "am", "arguments"]);
        }

        pub fn test() {
            // add your code here
            example();
        }
    }
    mod code2 {
        fn foo(name: &str, age: Option<u8>) {
            match age {
                Some(age) => println!("{} is {}.", name, age),
                None => println!("Who knows how old {} is?", name),
            }
        }

        fn example() {
            foo("Sally", Some(27));
            foo("Bill", None);
        }

        pub fn test() {
            // add your code here
            example();
        }
    }
    #[allow(clippy::redundant_field_names)]
    mod code3 {
        struct Arguments<'a> {
            name: &'a str,
            age: Option<u8>,
        }

        impl<'a> Arguments<'a> {
            fn new(name: &'a str) -> Arguments<'a> {
                Arguments {
                    name: name,
                    age: None,
                }
            }

            fn age(self, age: u8) -> Self {
                Arguments {
                    age: Some(age),
                    ..self
                }
            }
        }

        fn foo(arg: Arguments) {
            match arg.age {
                Some(age) => println!("{} is {}.", arg.name, age),
                None => println!("Who knows how old {} is?", arg.name),
            }
        }

        fn example() {
            foo(Arguments::new("Sally").age(27));
            foo(Arguments::new("Bill"));
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
        macro_rules! print_all {
    ($($args:expr),*) => {{
        $(
            println!("{}", $args);
        )*
    }}
}

        fn example() {
            print_all!(1, 2, "Hello");
        }

        pub fn test() {
            // add your code here
            example();
        }
    }
    mod code2 {
        macro_rules! sum {
    ($($args:expr),*) => {{
        let result = 0;
        $(
            let result = result + $args;
        )*
        result
    }}
}

        fn example() {
            assert_eq!(sum!(1, 2, 3), 6);
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
        // fn variable_func<T>(_vargs: &[T]) {}
        fn variable_func<T: std::fmt::Debug>(_vargs: &[T]) {
            for v in _vargs {
                println!("{:?}", v);
            }
        }

        fn example() {
            variable_func(&[1]);
            variable_func(&[1, 2]);
            variable_func(&["A", "B", "C"]);
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
pub fn test() {
    _enter!();
    answer1::test();
    answer2::test();
    answer3::test();
    _leave!();
}
