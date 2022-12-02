// @filename   : stackoverflow_0086_32384594.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/32384594
// @title      : How to check whether a path exists?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        // Rust 1.5+
        use std::path::Path;

        fn test1() {
            println!("{}", Path::new("/etc/hosts").exists());
        }
        /*
                pub fn exists(&self) -> bool {
                    fs::metadata(self).is_ok()
                }
        */
        // Rust 1.0+
        use std::fs;

        pub fn path_exists(path: &str) -> bool {
            fs::metadata(path).is_ok()
        }

        fn test2() {
            println!("{}", path_exists("/etc/hosts"));
        }

        pub fn test() {
            // add your code here
            test1();
            test2();
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
        use std::path::Path;
        pub fn test() {
            // add your code here
            let b = Path::new("file.txt").is_file();
            println!("{}", b);
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
