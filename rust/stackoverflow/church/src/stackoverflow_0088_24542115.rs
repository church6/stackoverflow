// @filename   : stackoverflow_0088_24542115.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/24542115
// @title      : How to index a String in Rust

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        pub fn test() {
            // add your code here
            let num = 14;
            let num_string = num.to_string();
            let i: usize = 0;
            let b: u8 = num_string.as_bytes()[i];
            println!("{b}");
            let c: char = b as char;
            println!("{c}");
        }
    }
    mod code2 {
        pub fn test() {
            // add your code here
            let num = 14;
            let num_string = num.to_string();
            let i: usize = 0;
            let nth = num_string.chars().nth(i).unwrap();
            println!("{nth}");
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
        fn is_palindrome(num: u64) -> bool {
            let num_string = num.to_string();
            let half = num_string.len() / 2;

            num_string
                .bytes()
                .take(half)
                .eq(num_string.bytes().rev().take(half))
        }
        pub fn test() {
            // add your code here
            assert!(is_palindrome(232i32 as u64));
            assert!(is_palindrome(1001i32 as u64));
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
            let s: String = String::from("abc");
            let x: usize = 0;
            //If you are sure
            println!("{}", s.chars().nth(x).unwrap());
            //or if not
            println!("{}", s.chars().nth(x).expect("message"));
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
