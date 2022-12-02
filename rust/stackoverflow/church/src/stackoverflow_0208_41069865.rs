// @filename   : stackoverflow_0208_41069865.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/41069865
// @title      : How to create an in-memory object that can be used as a Reader, Writer, or Seek in Rust?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        use std::io::{Cursor, Read, Seek, SeekFrom, Write};

        fn example() {
            // Create fake "file"
            let mut c = Cursor::new(Vec::new());

            // Write into the "file" and seek to the beginning
            c.write_all(&[1, 2, 3, 4, 5]).unwrap();
            c.seek(SeekFrom::Start(0)).unwrap();

            // Read the "file's" contents into a vector
            let mut out = Vec::new();
            c.read_to_end(&mut out).unwrap();

            println!("{:?}", out);
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
        //code1::test();
        //code2::test();
        //code3::test();
    }
}
#[allow(dead_code)]
mod answer2 {
    mod code1 {
        use std::io::{Read, Write};

        fn example() {
            // Create fake "file"
            let mut file = Vec::new();

            // Write into the "file"
            file.write_all(&[1, 2, 3, 4, 5]).unwrap();

            // Read the "file's" contents into a new vector
            let mut out = Vec::new();
            let mut c = file.as_slice();
            c.read_to_end(&mut out).unwrap();

            println!("{:?}", out);
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
