// @filename   : stackoverflow_0222_30555477.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/30555477
// @title      : Why do try!() and ? not compile when used in a function that doesn't return Option or Result?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        // As of Rust 1.26.0
        use std::{fs, io, path::Path};

        fn example() -> Result<(), io::Error> {
            let dir = Path::new("../FileSystem");

            if !dir.is_dir() {
                println!("Is not a directory");
                return Ok(());
            }

            for item in fs::read_dir(dir)? {
                let _file = match item {
                    Err(e) => {
                        println!("Error: {}", e);
                        return Ok(());
                    }
                    Ok(f) => f,
                };

                //println!("");
                println!();
            }

            println!("Done");
            Ok(())
        }

        pub fn test() {
            // add your code here
            match example() {
                Ok(()) => println!("Ok()"),
                Err(e) => println!("Error: {}", e /*.to_string()*/),
            }
        }
    }
    mod code2 {
        use std::{error::Error, fs, path::Path};

        fn print_dir_contents() -> Result<String, Box<dyn Error>> {
            let dir = Path::new("../FileSystem");

            if !dir.is_dir() {
                return Err(Box::from("Is not a directory!"));
            }

            for entry in fs::read_dir(dir)? {
                let path = entry?.path();
                let file_name = path.file_name().unwrap();
                println!("{}", file_name.to_string_lossy());
            }

            Ok("Done".into())
        }

        fn example() {
            match print_dir_contents() {
                Ok(s) => println!("{}", s),
                Err(e) => println!("Error: {}", e /*.to_string()*/),
            }
        }

        pub fn test() {
            // add your code here
            example();
        }
    }
    mod code3 {
        use std::{fs, io, path::Path};

        fn print_dir_contents() -> Result<String, io::Error> {
            let dir = Path::new("../FileSystem");

            for entry in fs::read_dir(dir)? {
                let path = entry?.path();
                let file_name = path.file_name().unwrap();
                println!("{}", file_name.to_string_lossy());
            }

            Ok("Done".into())
        }

        fn example() {
            match print_dir_contents() {
                Ok(s) => println!("{}", s),
                Err(e) => println!("Error: {}", e /*.to_string()*/),
            }
        }

        pub fn test() {
            // add your code here
            example();
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
