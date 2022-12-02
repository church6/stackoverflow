// @filename   : stackoverflow_0244_27588416.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/27588416
// @title      : How to send output to stderr?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        // After Rust 1.19
        // As of Rust 1.19, you can use the eprint and eprintln macros:
        // This was originally proposed in RFC 1896.
        fn example1() {
            eprintln!("This is going to standard error!, awesome");
        }

        // Before Rust 1.19
        // You can format stuff to stderr using similar macros though:

        use std::io::Write;
        fn example2() {
            let name = "world";
            writeln!(&mut std::io::stderr(), "Hello {}!", name).unwrap();
        }

        // use std::io::Write;
        fn example3() {
            let name = "world";
            let r = writeln!(&mut std::io::stderr(), "Hello {}!", name);
            r.expect("failed printing to stderr");
        }

        // This is a bit much, so let's wrap it back in a macro:
        // use std::io::Write;
        macro_rules! println_stderr(
    ($($arg:tt)*) => { {
        let r = writeln!(&mut ::std::io::stderr(), $($arg)*);
        r.expect("failed printing to stderr");
    } }
);

        fn example4() {
            let name = "world";
            println_stderr!("Hello {}!", name)
        }

        pub fn test() {
            // add your code here
            example1();
            example2();
            example3();
            example4();
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
