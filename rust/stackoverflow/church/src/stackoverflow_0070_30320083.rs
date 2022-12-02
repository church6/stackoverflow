// @filename   : stackoverflow_0070_30320083.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/30320083
// @title      : How to print a Vec?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        pub fn test() {
            // add your code here
            let v2 = vec![1; 10];
            println!("{:?}", v2);
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
        use std::fmt::{Display, Error, Formatter};

        struct NumVec(Vec<u32>);

        impl Display for NumVec {
            fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
                let mut comma_separated = String::new();

                for num in &self.0[0..self.0.len() - 1] {
                    comma_separated.push_str(&num.to_string());
                    comma_separated.push_str(", ");
                }

                comma_separated.push_str(&self.0[self.0.len() - 1].to_string());
                write!(f, "{}", comma_separated)
            }
        }

        pub fn test() {
            // add your code here
            let numbers = NumVec(vec![1; 10]);
            println!("{}", numbers);
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
