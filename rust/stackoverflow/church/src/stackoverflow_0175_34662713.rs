// @filename   : stackoverflow_0175_34662713.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/34662713
// @title      : How can I create parameterized tests in Rust?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        fn fibonacci(n: u32) -> u32 {
            match n {
                0 => 0,
                1 => 1,
                _ => fibonacci(n - 1) + fibonacci(n - 2),
            }
        }

        macro_rules! fib_tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (input, expected) = $value;
            assert_eq!(expected, fibonacci(input));
        }
    )*
    }
}

        fib_tests! {
            fib_0: (0, 0),
            fib_1: (1, 1),
            fib_2: (2, 1),
            fib_3: (3, 2),
            fib_4: (4, 3),
            fib_5: (5, 5),
            fib_6: (6, 8),
        }

        pub fn test() {
            // add your code here
        }
    }
    mod code2 {
        pub fn fibonacci(n: u32) -> u32 {
            /*
            if n < 0 {
                panic!("{} is negative!", n);
            }*/
            match n {
                0 => 0, //panic!("zero is not a right argument to fibonacci_reccursive()!"),
                1 | 2 => 1,
                3 => 2,
                /*
                50    => 12586269025,
                */
                _ => fibonacci(n - 1) + fibonacci(n - 2),
            }
        }

        macro_rules! fib_tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (input, expected) = $value;
            assert_eq!(expected, fibonacci(input));
        }
    )*
    }
}

        fib_tests! {
            fib_0: (0, 0),
            fib_1: (1, 1),
            fib_2: (2, 1),
            fib_3: (3, 2),
            fib_4: (4, 3),
            fib_5: (5, 5),
            fib_6: (6, 8),
        }
        pub fn test() {
            // add your code here
        }
    }
    mod code3 {
        /// Non reccursive function.
        ///
        /// `n` the rank used to compute the member of the sequence.
        pub fn fibonacci(n: u32) -> u32 {
            /*
            if n < 0 {
                panic!("{} is negative!", n);
            } else */
            if n == 0 {
                return 0; //panic!("zero is not a right argument to fibonacci()!");
            } else if n == 1 {
                return 1;
            }

            let mut sum = 0;
            let mut last = 0;
            let mut curr = 1;
            for _i in 1..n {
                sum = last + curr;
                last = curr;
                curr = sum;
            }
            sum
        }

        macro_rules! fib_tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (input, expected) = $value;
            assert_eq!(expected, fibonacci(input));
        }
    )*
    }
}

        fib_tests! {
            fib_0: (0, 0),
            fib_1: (1, 1),
            fib_2: (2, 1),
            fib_3: (3, 2),
            fib_4: (4, 3),
            fib_5: (5, 5),
            fib_6: (6, 8),
        }
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