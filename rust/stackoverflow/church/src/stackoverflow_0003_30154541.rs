// @filename   : stackoverflow_0003_30154541.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/30154541
// @title      : How do I concatenate strings?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        pub fn test() {
            // add your code here
            let mut owned_string: String = "hello ".to_owned();
            let borrowed_string: &str = "world";

            owned_string.push_str(borrowed_string);
            println!("{}", owned_string);
        }
    }
    mod code2 {
        pub fn test() {
            // add your code here
            let mut owned_string: String = "hello ".to_owned();
            let another_owned_string: String = "world".to_owned();

            owned_string.push_str(&another_owned_string);
            println!("{}", owned_string);
        }
    }
    mod code3 {
        pub fn test() {
            // add your code here
            let owned_string: String = "hello ".to_owned();
            let borrowed_string: &str = "world";

            let new_owned_string = owned_string + borrowed_string;
            println!("{}", new_owned_string);
        }
    }
    mod code4 {
        pub fn test() {
            // add your code here
            let borrowed_string: &str = "hello ";
            let another_borrowed_string: &str = "world";

            let together = format!("{}{}", borrowed_string, another_borrowed_string);

            // After https://rust-lang.github.io/rfcs/2795-format-args-implicit-identifiers.html
            // let together = format!("{borrowed_string}{another_borrowed_string}");

            println!("{}", together);
        }
    }
    mod code5 {
        pub fn test() {
            // add your code here
            let owned_string: String = "hello ".to_owned();
            let another_owned_string: String = "world".to_owned();

            let together = format!("{}{}", owned_string, another_owned_string);

            // After https://rust-lang.github.io/rfcs/2795-format-args-implicit-identifiers.html
            // let together = format!("{owned_string}{another_owned_string}");
            println!("{}", together);
        }
    }
    mod code6 {
        pub fn test() {
            // add your code here
            {
                let owned_string: String = "hello ".to_owned();
                let borrowed_string: &str = "world";
                // owned_string: this value is dropped without further use
                // #[allow(clippy::redundant_clone)]
                let together = owned_string.clone() + borrowed_string;
                println!("{}", together);
                println!("{}", owned_string);
            }
            {
                let owned_string: String = "hello ".to_owned();
                let borrowed_string: &str = "world";
                let together = owned_string + borrowed_string;
                println!("{}", together);
            }
        }
    }
    pub fn test() {
        code1::test();
        code2::test();
        code3::test();
        code4::test();
        code5::test();
        code6::test();
    }
}
#[allow(dead_code)]
mod answer2 {
    mod code1 {
        pub fn test() {
            // add your code here
            let a = "Hello";
            let b = "world";
            let result = [a, b].join("\n");

            print!("{}", result);
        }
    }
    mod code2 {
        pub fn test() {
            // add your code here
            let a = "Hello";
            let b = "world";
            let result = format!("{}\n{}", a, b);

            print!("{}", result);
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
mod answer3 {
    mod code1 {
        pub fn test() {
            // add your code here
            // First method (Using concat!() ):
            println!("{}", concat!("a", "b"))
        }
    }
    mod code2 {
        pub fn test() {
            // add your code here
            // Second method (using push_str() and + operator):
            let mut _a = "a".to_string();
            let _b = "b".to_string();
            let _c = "c".to_string();

            _a.push_str(&_b);

            println!("{}", _a);

            println!("{}", _a + &_c);
        }
    }
    mod code3 {
        pub fn test() {
            // add your code here
            // third method (Using format!()):
            let mut _a = "a".to_string();
            let _b = "b".to_string();
            let _c = format!("{}{}", _a, _b);

            println!("{}", _c);
        }
    }
    pub fn test() {
        code1::test();
        code2::test();
        code3::test();
    }
}
pub fn test() {
    _enter!();
    answer1::test();
    answer2::test();
    answer3::test();
    _leave!();
}
