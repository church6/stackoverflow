// @filename   : stackoverflow_0164_34606043.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/34606043
// @title      : How do I replace specific characters idiomatically in Rust?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        pub fn test() {
            // add your code here

            {
                let result = str::replace("Hello World!", "!", "?");
                println!("{}", result); // => "Hello World?"
                                        // Equivalently:
                let result = "Hello World!".replace('!', "?");
                println!("{}", result); // => "Hello World?"
            }

            {
                use regex::Regex;
                let re = Regex::new(r"[A-Za-z]").unwrap();
                let result = re.replace_all("Hello World!", "x");
                println!("{}", result); // => "xxxxx xxxxx!"
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

            {
                let s: String = "Hello, world!"
                    .chars()
                    .map(|x| match x {
                        '!' => '?',
                        'A'..='Z' => 'X',
                        'a'..='z' => 'x',
                        _ => x,
                    })
                    .collect();
                println!("{}", s); // Xxxxx, xxxxx?
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