// @filename   : stackoverflow_0021_31192956.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/31192956
// @title      : What's the de-facto way of reading and writing files in Rust 1.x?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        // Read a file to a String
        use std::fs;
        pub fn test() {
            // add your code here
            let data = fs::read_to_string("/etc/hosts").expect("Unable to read file");
            println!("{}", data);
        }
    }
    mod code2 {
        // Read a file as a Vec<u8>
        use std::fs;
        pub fn test() {
            // add your code here
            let data = fs::read("/etc/hosts").expect("Unable to read file");
            println!("{}", data.len());
        }
    }
    mod code3 {
        // Write a file
        use std::fs;
        pub fn test() {
            // add your code here
            let data = "Some data!";
            fs::write("/tmp/foo", data).expect("Unable to write file");
        }
    }
    mod code4 {
        //use std::io::BufRead;
        // Buffered I/O
        use std::fs::File;
        use std::io::BufReader;
        //use std::io::BufWriter;
        use std::io::Read;
        //use std::io::Write;
        pub fn test() {
            let mut data = String::new();
            let f = File::open("/etc/hosts").expect("Unable to open file");
            let mut br = BufReader::new(f);
            br.read_to_string(&mut data).expect("Unable to read string");
            println!("{}", data);
        }
    }
    mod code5 {
        //use std::io::BufRead;
        // Buffered I/O
        use std::fs::File;
        //use std::io::BufReader;
        use std::io::BufWriter;
        //use std::io::Read;
        use std::io::Write;
        pub fn test() {
            let data = "Some data!";
            let f = File::create("/tmp/foo").expect("Unable to create file");
            let mut f = BufWriter::new(f);
            f.write_all(data.as_bytes()).expect("Unable to write data");
        }
    }
    mod code6 {
        use std::io::BufRead;
        // Buffered I/O
        use std::fs::File;
        use std::io::BufReader;
        //use std::io::BufWriter;
        //use std::io::Read;
        //use std::io::Write;
        pub fn test() {
            let f = File::open("/etc/hosts").expect("Unable to open file");
            let f = BufReader::new(f);
            let lines = f.lines();

            for line in lines {
                let line = line.expect("Unable to read line");
                println!("Line: {}", line);
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
        use std::fs::OpenOptions;
        use std::io::Write;
        pub fn test() {
            // add your code here
            let data = "Some data!\n";
            let mut f = OpenOptions::new()
                .append(true)
                .create(true) // Optionally create the file if it doesn't already exist
                .open("/tmp/foo")
                .expect("Unable to open file");
            f.write_all(data.as_bytes()).expect("Unable to write data");
        }
    }
    mod code2 {
        use std::fs::OpenOptions;
        use std::io::{BufWriter, Write};

        pub fn test() {
            // add your code here
            let data = "Some data!\n";
            let f = OpenOptions::new()
                .append(true)
                .open("/tmp/foo")
                .expect("Unable to open file");
            let mut f = BufWriter::new(f);
            f.write_all(data.as_bytes()).expect("Unable to write data");
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
