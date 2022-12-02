// @filename   : stackoverflow_0073_31233938.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/31233938
// @title      : Converting from Option<String> to Option<&str>

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        pub fn test() {
            // add your code here
            // As of Rust 1.40, the standard library has Option::as_deref to do this:
            let opt: Option<String> = Some("some value".to_owned());
            let value = opt.as_deref().unwrap_or("default string");
            println!("{}", value);
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
            let opt: Option<String> = Some("some value".to_owned());
            //let value = opt.as_ref().map(|x| &**x).unwrap_or("default string");
            let value = opt.as_deref().unwrap_or("default string");
            println!("{}", value);
        }
    }
    mod code2 {
        pub fn test() {
            // add your code here
            let opt: Option<String> = Some("some value".to_owned());
            let value = opt.as_ref().map_or("default string", |x| &**x);
            println!("{}", value);
        }
    }
    mod code3 {
        pub fn test() {
            // add your code here
            let opt: Option<String> = Some("some value".to_owned());
            let value = opt.as_ref().map_or("default string", String::as_str);
            println!("{}", value);
        }
    }
    mod code4 {
        pub fn test() {
            let opt: Option<String> = Some("some value".to_owned());
            let value = opt.as_ref().map_or("default string", String::as_ref);
            println!("{}", value);
        }
    }
    mod code5 {
        use std::ops::Deref;
        pub fn test() {
            let opt: Option<String> = Some("some value".to_owned());
            let value = opt.as_ref().map_or("default string", String::deref);
            println!("{}", value);
        }
    }
    mod code6 {
        use std::borrow::Cow::{Borrowed, Owned};
        pub fn test() {
            {
                let opt: Option<String> = Some("some value".to_owned());
                #[allow(clippy::redundant_closure)]
                let value = opt.map_or(Borrowed("default string"), |x| Owned(x));
                println!("{}", value);
            }
            {
                let opt: Option<String> = Some("some value".to_owned());
                let value = opt.map_or(Borrowed("default string"), Owned);
                println!("{}", value);
            }
        }
    }
    pub fn test() {
        code1::test();
        code2::test();
        code3::test();
        code4::test();
        code5::test();
        code6::test();
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
    answer2::test();
    //answer3::test();
    _leave!();
}
