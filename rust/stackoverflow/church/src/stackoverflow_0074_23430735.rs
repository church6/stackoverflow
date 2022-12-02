// @filename   : stackoverflow_0074_23430735.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/23430735
// @title      : How to convert Vec<char> to a string

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        pub fn test() {
            // add your code here
            let v = vec!['a', 'b', 'c', 'd'];
            let s: String = v.into_iter().collect();
            println!("{}", s);
        }
    }
    mod code2 {
        pub fn test() {
            // add your code here
            let v = vec!['a', 'b', 'c', 'd'];
            let s: String = v.iter().collect();
            println!("{}", s);
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
        use std::iter::FromIterator;
        pub fn test() {
            // add your code here
            {
                let v = vec!['a', 'b', 'c', 'd'];
                let s = String::from_iter(v);
                println!("{}", s);
            }
            // vs
            {
                let v = vec!['a', 'b', 'c', 'd'];
                let s: String = v.into_iter().collect();
                println!("{}", s);
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
