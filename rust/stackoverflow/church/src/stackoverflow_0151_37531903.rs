// @filename   : stackoverflow_0151_37531903.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/37531903
// @title      : How do I print output without a trailing newline in Rust?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        use std::io;
        use std::io::Write; // <--- bring flush() into scope

        fn example() {
            println!("I'm picking a number between 1 and 100...");

            print!("Enter a number: ");
            io::stdout().flush().unwrap();
            let mut val = String::new();

            io::stdin()
                .read_line(&mut val)
                .expect("Error getting guess");

            println!("You entered {}", val);
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
        use std::io;
        pub fn test() {
            // add your code here
            let mut num = String::new();
            print!("Enter the number : ");
            io::stdin().read_line(&mut num).unwrap();
            println!("You entered {}", num);
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
    //answer1::test();
    //answer2::test();
    //answer3::test();
    _leave!();
}
