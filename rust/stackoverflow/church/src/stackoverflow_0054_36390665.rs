// @filename   : stackoverflow_0054_36390665.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/36390665
// @title      : How do you pass a Rust function as a parameter?

#[allow(dead_code)]
mod answer1 {
    /*
    TL;DR; Basically there are 3 types of closures (callable objects):

    Fn: It cannot modify the objects it captures.
    FnMut: It can modify the objects it captures.
    FnOnce: The most restricted. Can only be called once because when it is called it consumes itself and its captures.
    */
    mod code1 {
        fn fun_test(value: i32, f: &dyn Fn(i32) -> i32) -> i32 {
            println!("{}", f(value));
            value
        }

        fn times2(value: i32) -> i32 {
            2 * value
        }
        pub fn test() {
            // add your code here
            fun_test(5, &times2);
        }
    }
    mod code2 {
        fn fun_test_impl(value: i32, f: impl Fn(i32) -> i32) -> i32 {
            println!("{}", f(value));
            value
        }
        fn fun_test_dyn(value: i32, f: &dyn Fn(i32) -> i32) -> i32 {
            println!("{}", f(value));
            value
        }
        fn fun_test_ptr(value: i32, f: fn(i32) -> i32) -> i32 {
            println!("{}", f(value));
            value
        }

        fn times2(value: i32) -> i32 {
            2 * value
        }

        pub fn test() {
            // add your code here

            let y = 2;
            //static dispatch
            fun_test_impl(5, times2);
            fun_test_impl(5, |x| 2 * x);
            fun_test_impl(5, |x| y * x);
            //dynamic dispatch
            fun_test_dyn(5, &times2);
            fun_test_dyn(5, &|x| 2 * x);
            fun_test_dyn(5, &|x| y * x);
            //C-like pointer to function
            fun_test_ptr(5, times2);
            fun_test_ptr(5, |x| 2 * x); //ok: empty capture set
                                        // error[E0308]: mismatched types
                                        // fun_test_ptr(5, |x| y * x); //error: expected fn pointer, found closure
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
    //answer2::test();
    //answer3::test();
    _leave!();
}
