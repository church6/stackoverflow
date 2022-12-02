// @filename   : stackoverflow_0207_16267578.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/16267578
// @title      : How can I convert char to string?

#[allow(dead_code)]
mod answer1 {
    #[allow(clippy::print_literal)]
    mod code1 {
        fn example1() {
            let string = 'c'.to_string();
            println!("string = {string}");
            // or
            println!("string = {}", 'c');
            let string = format!("{}", 'c');
            println!("string = {string}");
        }

        fn example2() {
            let c = 'c';
            let mut tmp = [0u8; 4];
            let string = c.encode_utf8(&mut tmp);
            println!("string = {string}");
        }

        pub fn test() {
            // add your code here
            example1();
            example2();
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
    //answer1::test();
    //answer2::test();
    //answer3::test();
    _leave!();
}
