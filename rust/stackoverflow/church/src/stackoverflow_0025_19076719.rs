// @filename   : stackoverflow_0025_19076719.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/19076719
// @title      : How do I convert a Vector of bytes (u8) to a string?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        use std::str;
        pub fn test() {
            // add your code here
            let buf = &[0x41u8, 0x41u8, 0x42u8];

            let s = match str::from_utf8(buf) {
                Ok(v) => v,
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            };

            println!("result: {}", s);
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
            let buf = &[0x41u8, 0x41u8, 0x42u8];
            let s = String::from_utf8_lossy(buf);
            println!("result: {}", s);
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
            let bytes = vec![0x41, 0x42, 0x43];
            let s = String::from_utf8(bytes).expect("Found invalid UTF-8");
            println!("{}", s);
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
        let bytes = vec![0x41, 0x42, 0x43];
        let s = format!("{:?}", &bytes);
        println!("{}", s);
    }
}
pub fn test() {
    _enter!();
    answer1::test();
    answer2::test();
    answer3::test();
    answer4::test();
    _leave!();
}
