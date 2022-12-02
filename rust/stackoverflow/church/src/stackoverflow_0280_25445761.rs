// @filename   : stackoverflow_0280_25445761.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/25445761
// @title      : Returning a closure from a function

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        // As of Rust 1.26, you can use impl trait:

        fn make_adder(a: i32) -> impl Fn(i32) -> i32 {
            move |b| a + b
        }

        fn example() {
            println!("{}", make_adder(1)(2));
            assert_eq!(5i32, make_adder(2i32)(3i32));
        }
        pub fn test() {
            // add your code here
            example();
        }
    }
    mod code2 {
        fn make_adder(a: i32) -> impl Fn(i32) -> i32 {
            move |b| a + b
            // each closure has a unique, un-namable type
            /*
            if a > 0 {
                move |b| a + b
            } else {
                move |b| a - b
            }
            */
        }

        fn example() {
            println!("{}", make_adder(1)(2));
            println!("{}", make_adder(0)(2));
        }

        pub fn test() {
            // add your code here
            example();
        }
    }
    mod code3 {
        struct Example<F>(F);

        fn make_it() -> Example<impl Fn()> {
            Example(|| println!("Hello"))
        }

        fn example() {
            let unnamed_type_ok = make_it();
            let Example(fp) = unnamed_type_ok;
            fp();
            //let named_type_bad:
            /* No valid type here */
            // = make_it();
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
        fn make_adder(a: i32) -> Box<dyn Fn(i32) -> i32> {
            Box::new(move |b| a + b)
        }

        fn example() {
            println!("{}", make_adder(1)(2));
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
        fn counter() -> impl FnMut() -> i32 {
            let mut value = 0;

            move || -> i32 {
                value += 1;
                //return value;
                value
            }
        }

        fn example() {
            let mut incre = counter();
            println!("Count 1: {}", incre());
            println!("Count 2: {}", incre());
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
