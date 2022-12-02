// @filename   : stackoverflow_0058_28273169.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/28273169
// @title      : How do I convert between numeric types safely and idiomatically?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        // From a type that fits completely within another
        // There's no problem here. Use the From trait to be explicit that there's no loss occurring:

        fn example1(v: i8) -> i32 {
            i32::from(v) // or v.into()
        }
        fn example2(v: i8) -> i32 {
            v as i32
        }
        pub fn test() {
            // add your code here
            assert_eq!(example1(100), 100i32);
            assert_eq!(example2(100), 100i32);
        }
    }
    mod code2 {
        // From a type that doesn't fit completely in another

        use std::convert::TryFrom;
        // Since Rust 1.34, you can use TryFrom:
        fn example1(v: i32) -> Option<i8> {
            i8::try_from(v).ok()
        }

        // Before that, you'd have to write similar code yourself:
        fn example2(v: i32) -> Option<i8> {
            if v > std::i8::MAX as i32 {
                None
            } else {
                Some(v as i8)
            }
        }
        pub fn test() {
            // add your code here
            assert_eq!(example1(100i32), Some(100i8));
            assert_eq!(example2(100i32), Some(100i8));
        }
    }
    mod code3 {
        pub fn test() {
            // add your code here

            let a: u16 = 0x1234;
            let b: u8 = a as u8;
            println!("0x{:04x}, 0x{:02x}", a, b); // 0x1234, 0x34

            let a: i16 = -257;
            let b: u8 = a as u8;
            println!("0x{:02x}, 0x{:02x}", a, b); // 0xfeff, 0xff
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
