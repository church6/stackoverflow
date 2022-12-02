// @filename   : stackoverflow_0219_28024373.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/28024373
// @title      : Is there a way to print enum values?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        #[derive(Debug)]
        enum Suit {
            Heart,
            Diamond,
            Spade,
            Club,
        }

        fn example() {
            let s = Suit::Heart;
            println!("{:?}", s);
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
mod answer2 {
    mod code1 {
        use std::fmt;

        enum Suit {
            Heart,
            Diamond,
            Spade,
            Club,
        }

        impl fmt::Display for Suit {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match *self {
                    Suit::Heart => write!(f, "♥"),
                    Suit::Diamond => write!(f, "♦"),
                    Suit::Spade => write!(f, "♠"),
                    Suit::Club => write!(f, "♣"),
                }
            }
        }

        fn example() {
            let heart = Suit::Heart;
            println!("{}", heart);
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
