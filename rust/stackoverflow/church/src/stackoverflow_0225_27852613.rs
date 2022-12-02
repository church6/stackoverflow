// @filename   : stackoverflow_0225_27852613.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/27852613
// @title      : Why does printing a pointer print the same thing as printing the dereferenced pointer?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        fn example1() {
            let x = 1;
            let ptr_y = &x;
            println!("x: {}, ptr_y: {}", x, *ptr_y);
        }
        // This gives me the same results (x=1; y=1) even without an explicit dereference:

        fn example2() {
            let x = 1;
            let ptr_y = &x;
            println!("x: {}, ptr_y: {}", x, ptr_y);
        }
        pub fn test() {
            // add your code here
            example1();
            example2();
        }
    }
    mod code2 {
        /*
                use std::error::Error;
                impl<'a> std::fmt::Display for &'a String {
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), dyn Error> {
                        std::fmt::Display::fmt(&**self, f)
                    }
                }
        */
        fn example() {
            let x = 1;
            let ptr_y = &x;
            println!("x: {}, ptr_y: {}, address: {:p}", x, ptr_y, ptr_y);
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
