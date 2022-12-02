// @filename   : stackoverflow_0137_45786955.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/45786955
// @title      : How to compose functions in Rust?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        fn compose<A, B, C, G, F>(f: F, g: G) -> impl Fn(A) -> C
        where
            F: Fn(A) -> B,
            G: Fn(B) -> C,
        {
            move |x| g(f(x))
        }

        fn example() {
            let multiply_and_add = compose(|x| x * 2, |x| x + 2);
            let divide_and_subtract = compose(|x| x / 2, |x| x - 2);

            let finally = compose(multiply_and_add, divide_and_subtract);
            println!("Result is {}", finally(10));
        }

        pub fn test() {
            // add your code here
            example();
        }
    }
    mod code2 {
        macro_rules! compose {
    ( $last:expr ) => { $last };
    ( $head:expr, $($tail:expr), +) => {
        compose_two($head, compose!($($tail),+))
    };
}

        fn compose_two<A, B, C, G, F>(f: F, g: G) -> impl Fn(A) -> C
        where
            F: Fn(A) -> B,
            G: Fn(B) -> C,
        {
            move |x| g(f(x))
        }

        fn example() {
            let add = |x| x + 2;
            let multiply = |x| x * 2;
            let divide = |x| x / 2;
            let intermediate = compose!(add, multiply, divide);

            let subtract = |x| x - 2;
            let finally = compose!(intermediate, subtract);

            println!("Result is {}", finally(10));
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
