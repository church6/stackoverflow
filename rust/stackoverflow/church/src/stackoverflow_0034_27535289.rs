// @filename   : stackoverflow_0034_27535289.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/27535289
// @title      : What is the correct way to return an Iterator (or any other trait)?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        // Impl trait
        // As of Rust 1.26, you can use impl trait:
        #[allow(clippy::needless_lifetimes)]
        fn to_words<'a>(text: &'a str) -> impl Iterator<Item = &'a str> {
            text.split(' ')
        }

        pub fn test() {
            // add your code here
            let text = "word1 word2 word3";
            println!("{}", to_words(text).take(2).count());
        }
    }
    mod code2 {
        // Boxed
        // If you don't mind losing a little bit of efficiency, you can return a Box<dyn Iterator>:

        fn to_words<'a>(text: &'a str) -> Box<dyn Iterator<Item = &'a str> + 'a> {
            Box::new(text.split(' '))
        }
        pub fn test() {
            // add your code here
            let text = "word1 word2 word3";
            println!("{}", to_words(text).take(2).count());
        }
    }
    mod code3 {
        // Newtype
        use std::str;

        struct Wrapper<'a>(str::Split<'a, char>);

        impl<'a> Iterator for Wrapper<'a> {
            type Item = &'a str;

            fn next(&mut self) -> Option<&'a str> {
                self.0.next()
            }

            fn size_hint(&self) -> (usize, Option<usize>) {
                self.0.size_hint()
            }
        }

        fn to_words(text: &str) -> Wrapper<'_> {
            Wrapper(text.split(' '))
        }
        pub fn test() {
            // add your code here
            let text = "word1 word2 word3";
            println!("{}", to_words(text).take(2).count());
        }
    }
    mod code4 {
        // Type alias
        use std::str;

        type MyIter<'a> = str::Split<'a, char>;

        fn to_words(text: &str) -> MyIter<'_> {
            text.split(' ')
        }

        pub fn test() {
            let text = "word1 word2 word3";
            println!("{}", to_words(text).take(2).count());
        }
    }
    mod code5 {
        // Dealing with closures
        use std::{iter::Filter, ops::Range};

        type Odds = Filter<Range<i32>, fn(&i32) -> bool>;

        fn odd_numbers() -> Odds {
            fn f(&v: &i32) -> bool {
                v % 2 != 0
            }
            (0..100).filter(f as fn(v: &i32) -> bool)
        }
        pub fn test() {
            let odds = odd_numbers();
            // Filter { iter: 0..100 }
            println!("{:?}", odds);
            /*
            for each in odds {
                println!("{}", each);
            }
            */
        }
    }
    pub fn test() {
        code1::test();
        code2::test();
        code3::test();
        code4::test();
        code5::test();
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
    answer1::test();
    //answer2::test();
    //answer3::test();
    _leave!();
}
