// @filename   : stackoverflow_0106_16504643.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/16504643
// @title      : What is the overhead of Rust's Option type?

#[allow(dead_code)]
mod answer1 {
    // Yes, there is some compiler magic that optimises Option<ptr> to a single pointer (most of the time).
    mod code1 {
        use std::mem::size_of;

        macro_rules! show_size {
            (header) => {
                println!("{:<22} {:>4}    {}", "Type", "T", "Option<T>");
            };
            ($t:ty) => {
                println!(
                    "{:<22} {:4} {:4}",
                    stringify!($t),
                    size_of::<$t>(),
                    size_of::<Option<$t>>()
                )
            };
        }
        pub fn test() {
            // add your code here
            show_size!(header);
            show_size!(i32);
            show_size!(&i32);
            show_size!(Box<i32>);
            show_size!(&[i32]);
            show_size!(Vec<i32>);
            show_size!(Result<(), Box<i32>>);
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
