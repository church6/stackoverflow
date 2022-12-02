// @filename   : stackoverflow_0278_30511331.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/30511331
// @title      : Getting the absolute path from a PathBuf

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        // Rust 1.5.0 added std::fs::canonicalize, which sounds pretty close to what you want:
        // Returns the canonical form of a path with all intermediate components normalized and symbolic links resolved.
        // Note that, unlike the accepted answer, this removes the ./ from the returned path.
        // A simple example from my machine:

        use std::fs;
        use std::path::PathBuf;

        fn example() {
            let srcdir = PathBuf::from("./src");
            println!("1={:?}", fs::canonicalize(&srcdir));

            let solardir = PathBuf::from("./../");
            println!("1={:?}", fs::canonicalize(&solardir));
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
        use std::env;
        use std::io;
        use std::path::{Path, PathBuf};

        //use path_clean::PathClean;

        pub fn absolute_path(path: impl AsRef<Path>) -> io::Result<PathBuf> {
            let path = path.as_ref();

            let absolute_path = if path.is_absolute() {
                path.to_path_buf()
            } else {
                env::current_dir()?.join(path)
            }; //.clean()

            Ok(absolute_path)
        }

        pub fn test() {
            // add your code here
            let abs_path = absolute_path(PathBuf::from("./src")).unwrap();
            println!("2={}", abs_path.display());
            let abs_path = absolute_path(PathBuf::from("../")).unwrap();
            println!("2={}", abs_path.display());
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
        use std::path::PathBuf;

        pub fn test() {
            // add your code here
            let relative_path = PathBuf::from("/etc/hostname");
            //let mut absolute_path = try!(std::env::current_dir());
            // absolute_path.push(relative_path);
            let mut absolute_path = std::env::current_dir().unwrap();
            absolute_path.push(relative_path);
            println!("3={}", absolute_path.display());
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
pub fn test() {
    _enter!();
    answer1::test();
    answer2::test();
    answer3::test();
    _leave!();
}
