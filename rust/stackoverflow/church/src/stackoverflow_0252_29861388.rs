// @filename   : stackoverflow_0252_29861388.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/29861388
// @title      : When is it useful to define multiple lifetimes in a struct?

#[allow(dead_code)]
mod answer1 {
    #[allow(clippy::needless_borrow)]
    mod code1 {
        static ZERO: i32 = 0;

        struct Foo<'a, 'b> {
            x: &'a i32,
            y: &'b i32,
        }

        fn get_x_or_zero_ref<'a, 'b>(x: &'a i32, y: &'b i32) -> &'a i32 {
            if *x > *y {
                /*return*/
                x
            } else {
                /*return*/
                &ZERO
            }
        }

        fn example() {
            let x = 1;
            let v;
            {
                let y = 2;
                let f = Foo { x: &x, y: &y };
                v = get_x_or_zero_ref(&f.x, &f.y);
            }
            println!("{}", *v);
        }

        pub fn test() {
            // add your code here
            example();
        }
    }
    mod code2 {
        static ZERO: i32 = 0;

        struct Foo {
            x: i32,
            y: i32,
        }

        fn get_x_or_zero_ref(x: i32, y: i32) -> i32 {
            if x > y {
                /*return*/
                x
            } else {
                /*return*/
                ZERO
            }
        }

        fn example() {
            let x = 1;
            let v;
            {
                let y = 2;
                let f = Foo { x, y };
                v = get_x_or_zero_ref(f.x, f.y);
            }
            println!("{}", v);
        }

        pub fn test() {
            // add your code here
            example()
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
