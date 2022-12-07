// @filename   : stackoverflow_0312_48034119.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/48034119
// @title      : How can I pattern match against an Option<String>?

#[allow(dead_code)]
mod answer1 {
    // It's a known limitation of Rust's patterns.

    /*
    Method calls (including internal methods for operators like ==) automatically call .deref() as needed, so String gets automagically turned into &str for comparisons with literals.

    On the other hand, the patterns are quite literal in their comparisons, and find that String and &str are different.
    */

    // There are two solutions:

    // Change Option<String> to Option<&str> before matching on it: Some(a).as_deref(). The as_deref() is a combo of as_ref() that makes Option<&String> (preventing move), and deref()/as_str() then unambiguously references it as a &str.

    // Use match guard: match Some(a) { Some(ref s) if s == "hello" => … }. Some(ref s) matches any String, and captures it as s: &String, which you can then compare in the if guard which does the usual flexible coercions to make it work.

    mod code1 {
        pub fn test() {
            // add your code here

            let a = "hello".to_string();

            match &a[..] {
                "hello" => {
                    println!("Matches hello");
                }
                _ => panic!(),
            }
        }
    }
    mod code2 {
        pub fn test() {
            // add your code here
            /*
                        let a = "hello".to_string();
                        match Some(a) {
                            Some("hello") => {
                                println!("Matches some hello");
                            }
                            _ => panic!(),
                        }
            */

            let a = "hello".to_string();
            match Some(a) {
                Some(b) => match &b[..] {
                    "hello" => {
                        println!("Matches some, some hello");
                    }
                    _ => panic!(),
                },
                _ => panic!(),
            }
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
        fn example() {
            // As of Rust 1.40, you can now call as_deref on Option<String> to convert it to Option<&str> and then match on it:

            let v: Option<String> = Some("help".to_string());

            match v.as_deref() {
                Some("help") => {
                    println!("match help")
                }
                Some(s) => {
                    println!("match {s}")
                }
                None => {
                    println!("None")
                }
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
