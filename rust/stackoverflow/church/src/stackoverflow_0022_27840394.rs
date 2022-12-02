// @filename   : stackoverflow_0022_27840394.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/27840394
// @title      : How can a Rust program access metadata from its Cargo package?

#[allow(dead_code)]
mod answer1 {
    // CARGO_MANIFEST_DIR
    // CARGO_PKG_AUTHORS
    // CARGO_PKG_DESCRIPTION
    // CARGO_PKG_HOMEPAGE
    // CARGO_PKG_NAME
    // CARGO_PKG_REPOSITORY
    // CARGO_PKG_VERSION
    // CARGO_PKG_VERSION_MAJOR
    // CARGO_PKG_VERSION_MINOR
    // CARGO_PKG_VERSION_PATCH
    // CARGO_PKG_VERSION_PRE
    mod code1 {
        pub fn test() {
            // add your code here
            const VERSION1: &str = env!("CARGO_PKG_VERSION");
            println!("MyProgram v{}", VERSION1);

            const VERSION2: Option<&str> = option_env!("CARGO_PKG_VERSION");
            let version: &str = VERSION2.unwrap_or("unknown");
            println!("MyProgram v{}", version);
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
