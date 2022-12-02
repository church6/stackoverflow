// @filename   : stackoverflow_0008_27043268.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/27043268
// @title      : Convert a String to int?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        pub fn test() {
            // add your code here
            let my_string = "27".to_string(); // `parse()` works with `&str` and `String`!
            let my_int = my_string.parse::<i32>().unwrap();
            println!("{}", my_int);
            let my_int: i32 = my_string.parse().unwrap();
            println!("{}", my_int);
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
            let my_u8: u8 = "42".parse().unwrap();
            println!("{}", my_u8);
            let my_u32: u32 = "42".parse().unwrap();
            println!("{}", my_u32);

            // or, to be safe, match the `Err`
            match "foobar".parse::<i32>() {
                Ok(n) => println!("n={}", n),
                Err(e) => println!("Err={}", e),
            }

            match "1024".parse::<i32>() {
                Ok(n) => println!("n={}", n),
                Err(e) => println!("Err={}", e),
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
        use std::str::FromStr;
        pub fn test() {
            // add your code here
            let my_num = i32::from_str("9").unwrap_or(0);
            println!("{}", my_num);
            // bad
            // let my_num = i32::from_str_radix("9", 10).unwrap_or(0);
            // println!("{}", my_num);
            // better
            let my_num = "9".parse::<i32>().unwrap_or(0);
            println!("{}", my_num);

            let char = "23";
            let char: i32 = char.trim().parse().unwrap();
            println!("{}", char + 1);
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
