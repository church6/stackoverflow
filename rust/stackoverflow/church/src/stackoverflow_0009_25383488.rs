// @filename   : stackoverflow_0009_25383488.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/25383488
// @title      : How to match a String against string literals?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        pub fn test() {
            // add your code here
            let stringthing = String::from("c");
            match stringthing.as_str() {
                "a" => println!("0"),
                "b" => println!("1"),
                "c" => println!("2"),
                _ => println!("something else!"),
            }
            match stringthing.as_ref() {
                "a" => println!("0"),
                "b" => println!("1"),
                "c" => println!("2"),
                _ => println!("something else!"),
            }
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
            let stringthing = String::from("c");
            match &stringthing[..] {
                "a" => println!("0"),
                "b" => println!("1"),
                "c" => println!("2"),
                _ => println!("something else!"),
            }
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
            let stringthing = String::from("c");
            match &stringthing as &str {
                "a" => println!("0"),
                "b" => println!("1"),
                "c" => println!("2"),
                _ => println!("something else!"),
            }
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
mod answer4 {
    pub fn test() {
        /*
                    let stringthing = String::from("c");
                    match stringthing.as_slice() {
                        "a" => println!("0"),
                        "b" => println!("1"),
                        "c" => println!("2"),
                        _ => println!("something else!"),
                    }
        */
    }
    // better to use .as_ref() or .as_str(), both did not take ownership.
}
pub fn test() {
    _enter!();
    answer1::test();
    answer2::test();
    answer3::test();
    answer4::test();
    _leave!();
}
