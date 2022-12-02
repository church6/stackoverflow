// @filename   : stackoverflow_0126_37388107.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/37388107
// @title      : How to convert the PathBuf to String

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        use std::env;
        fn example() {
            {
                let cwd = env::current_dir().unwrap();
                // `cwd` moved due to this method call
                let my_str: String = cwd.as_os_str().to_str().unwrap().to_string();
                println!("{:?}", my_str);
            }
            {
                let cwd = env::current_dir().unwrap();
                let my_str: String = cwd.into_os_string().into_string().unwrap();
                println!("{:?}", my_str);
            }
            {
                let cwd = env::current_dir().unwrap();
                let my_str = cwd.into_os_string().into_string().unwrap();
                println!("{:?}", my_str);
            }
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
