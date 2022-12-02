// @filename   : stackoverflow_0254_53177980.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/53177980
// @title      : How do I conditionally execute code only when an Option is None?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        fn ffunc() -> Option<i32> {
            let x = 0;
            if x == 0 {
                None
            } else {
                Some(0i32)
            }
        }

        fn example() {
            let x = ffunc();

            #[allow(clippy::redundant_pattern_matching)]
            if let None = x {
                println!("x is None")
            }
            // Or using the Option::is_none function:

            let x = ffunc();

            if x.is_none() {
                println!("x is None")
            }

            #[allow(clippy::partialeq_to_none)]
            if x == None {
                println!("x is None")
            }

            let v = ffunc();
            if let Some(x) = v {
                println!("x is {x}");
            } else {
                println!("x is None")
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
