// @filename   : stackoverflow_0262_45882329.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/45882329
// @title      : Read large files line by line in Rust [duplicate]

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        use std::fs::File;
        use std::io::{self, prelude::*, BufReader};

        fn example() -> io::Result<()> {
            let file = File::open("/etc/hostname")?;
            let reader = BufReader::new(file);

            for line in reader.lines() {
                println!("{}", line?);
            }

            Ok(())
        }

        pub fn test() {
            // add your code here
            example().unwrap();
        }
    }
    mod code2 {
        fn example() -> std::io::Result<()> {
            let mut reader = my_reader::BufReader::open("/etc/hostname")?;
            let mut buffer = String::new();

            while let Some(line) = reader.read_line(&mut buffer) {
                println!("{}", line?.trim());
            }

            Ok(())
        }

        mod my_reader {
            use std::{
                fs::File,
                io::{self, prelude::*},
            };

            pub struct BufReader {
                reader: io::BufReader<File>,
            }

            impl BufReader {
                pub fn open(path: impl AsRef<std::path::Path>) -> io::Result<Self> {
                    let file = File::open(path)?;
                    let reader = io::BufReader::new(file);

                    Ok(Self { reader })
                }

                pub fn read_line<'buf>(
                    &mut self,
                    buffer: &'buf mut String,
                ) -> Option<io::Result<&'buf mut String>> {
                    buffer.clear();

                    self.reader
                        .read_line(buffer)
                        .map(|u| if u == 0 { None } else { Some(buffer) })
                        .transpose()
                }
            }
        }

        pub fn test() {
            // add your code here
            example().unwrap();
        }
    }
    mod code3 {
        fn example() -> std::io::Result<()> {
            for line in my_reader::BufReader::open("/etc/hostname")? {
                println!("{}", line?.trim());
            }

            Ok(())
        }

        mod my_reader {
            use std::{
                fs::File,
                io::{self, prelude::*},
                rc::Rc,
            };

            pub struct BufReader {
                reader: io::BufReader<File>,
                buf: Rc<String>,
            }

            fn new_buf() -> Rc<String> {
                Rc::new(String::with_capacity(1024)) // Tweakable capacity
            }

            impl BufReader {
                pub fn open(path: impl AsRef<std::path::Path>) -> io::Result<Self> {
                    let file = File::open(path)?;
                    let reader = io::BufReader::new(file);
                    let buf = new_buf();

                    Ok(Self { reader, buf })
                }
            }

            impl Iterator for BufReader {
                type Item = io::Result<Rc<String>>;

                fn next(&mut self) -> Option<Self::Item> {
                    let buf = match Rc::get_mut(&mut self.buf) {
                        Some(buf) => {
                            buf.clear();
                            buf
                        }
                        None => {
                            self.buf = new_buf();
                            Rc::make_mut(&mut self.buf)
                        }
                    };

                    self.reader
                        .read_line(buf)
                        .map(|u| {
                            if u == 0 {
                                None
                            } else {
                                Some(Rc::clone(&self.buf))
                            }
                        })
                        .transpose()
                }
            }
        }

        pub fn test() {
            // add your code here
            example().unwrap();
        }
    }
    pub fn test() {
        code1::test();
        code2::test();
        code3::test();
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
