// @filename   : stackoverflow_0250_27996430.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/27996430
// @title      : Reversing a string in Rust

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        fn example() {
            let reserved = "palimpsest";
            println!("{}", reserved.chars().rev().collect::<String>());
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
        use unicode_segmentation::UnicodeSegmentation;
        pub fn test() {
            // add your code here

            let word: &str = "loẅks";
            let drow: String = word
                // Split the string into an Iterator of &strs, where each element is an
                // extended grapheme cluster.
                .graphemes(true)
                // Reverse the order of the grapheme iterator.
                .rev()
                // Collect all the chars into a new owned String.
                .collect();

            assert_eq!(drow, "skẅol");

            // Print it out to be sure.
            println!("drow = `{}`", drow);
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
