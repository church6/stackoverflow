// @filename   : stackoverflow_0001_24158114.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/24158114
// @title      : What are the differences between Rust's `String` and `str`?

#[allow(dead_code)]
mod answer1 {
    /*
    String is the dynamic heap string type, like Vec: use it when you need to own or modify your string data.
    */

    /*
    str is an immutable1 sequence of UTF-8 bytes of dynamic length somewhere in memory.
    Since the size is unknown, one can only handle it behind a pointer.
    */
    /* This means that str most commonly2 appears as &str: a reference to some UTF-8 data, normally called a "string slice" or just a "slice".
     */
    mod code1 {
        pub fn test() {
            // add your code here
            use std::str;
            let x: &[u8] = &[b'a', b'b', b'c'];
            println!("{:?}", x);
            let stack_str: &str = str::from_utf8(x).unwrap();
            println!("{}", stack_str);
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
    /*
    A Rust String is like a std::string; it owns the memory and does the dirty job of managing memory.
    */
    /*
    A Rust &str is like a char* (but a little more sophisticated); it points us to the beginning of a chunk in the same way you can get a pointer to the contents of std::string.
    */
    mod code1 {
        pub fn test() {
            // add your code here
            let a: String = "hello rust".into();
            println!("{}", a);
            let a: &str = "hello rust";
            println!("{}", a);
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
    #[allow(unused_unsafe)]
    mod code1 {
        pub fn test() {
            // add your code here
            // From a string literal
            let a = "Hello World";
            let mut s = String::from(a);
            println!("{}", s);

            // From raw parts
            let ptr = s.as_mut_ptr();
            let len = s.len();
            let capacity = s.capacity();
            println!("{:p}:{},{}", ptr, len, capacity);
            let s = "bad";
            unsafe {
                // free(): double free detected in tcache 2
                // String::from_raw_parts(ptr, len, capacity)
            }
            println!("{}", s);

            // From a character
            let ch = 'c';
            let s = ch.to_string();
            println!("{}", s);

            // From vector of bytes
            let hello_world = vec![72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100];
            // We know it is valid sequence, so we can use unwrap
            let hello_world = String::from_utf8(hello_world).unwrap();
            println!("{}", hello_world); // Hello World

            {
                // From input buffer
                use std::io::{self, Read};

                fn main() -> io::Result<()> {
                    let mut buffer = String::new();
                    let stdin = io::stdin();
                    let mut handle = stdin.lock();

                    handle.read_to_string(&mut buffer)?;
                    Ok(())
                }
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
mod answer4 {
    pub fn test() {
        let string: String = "a string".to_string();
        let substring1: &str = &string[1..3];
        let substring2: &str = &string[2..4];
        println!("{}", string);
        println!("{}", substring1);
        println!("{}", substring2);
    }
}
mod church {
    pub fn test() {
        use std::mem;

        unsafe {
            let s = String::from("hello");

            // Prevent automatically dropping the String's data
            let mut s = mem::ManuallyDrop::new(s);

            let ptr = s.as_mut_ptr();
            let len = s.len();
            let capacity = s.capacity();

            let s = String::from_raw_parts(ptr, len, capacity);

            assert_eq!(String::from("hello"), s);
        }
    }
}
pub fn test() {
    _enter!();
    answer1::test();
    answer2::test();
    answer3::test();
    answer4::test();
    church::test();
    _leave!();
}
