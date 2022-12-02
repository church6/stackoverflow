// @filename   : stackoverflow_0229_27650312.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/27650312
// @title      : Show u8 slice in hex representation

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        // Rust 1.26.0 and up
        // The :x? "debug with hexadecimal integers" formatter can be used:

        fn example() {
            let data = b"hello";
            // lower case
            println!("{:x?}", data);
            // upper case
            println!("{:X?}", data);

            let data = [0x0, 0x1, 0xe, 0xf, 0xff];
            // print the leading zero
            println!("{:02X?}", data);
            // It can be combined with the pretty modifier as well
            println!("{:#04X?}", data);
        }
        pub fn test() {
            // add your code here
            example();
        }
    }
    mod code2 {
        // Rust 1.0 and up
        use std::fmt::Write;

        fn example() {
            let mut s = String::new();
            for &byte in "Hello".as_bytes() {
                write!(&mut s, "{:X} ", byte).expect("Unable to write");
            }

            println!("{}", s);
        }

        pub fn test() {
            // add your code here
            example();
        }
    }
    mod code3 {
        use std::fmt;

        struct HexSlice<'a>(&'a [u8]);

        impl<'a> HexSlice<'a> {
            fn new<T>(data: &'a T) -> HexSlice<'a>
            where
                T: ?Sized + AsRef<[u8]> + 'a,
            {
                HexSlice(data.as_ref())
            }
        }

        // You can choose to implement multiple traits, like Lower and UpperHex
        impl fmt::Display for HexSlice<'_> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                for byte in self.0 {
                    // Decide if you want to pad the value or have spaces inbetween, etc.
                    write!(f, "{:X} ", byte)?;
                }
                Ok(())
            }
        }

        fn example() {
            // To get a `String`
            let _s = format!("{}", HexSlice::new("Hello"));

            // Or print it directly
            println!("{}", HexSlice::new("world"));

            // Works with
            HexSlice::new("Hello"); // string slices (&str)
            HexSlice::new(b"Hello"); // byte slices (&[u8])
            HexSlice::new(&"World".to_string()); // References to String
            HexSlice::new(&vec![0x00, 0x01]); // References to Vec<u8>
        }
        pub fn test() {
            // add your code here
            example();
        }
    }
    mod code4 {
        #[derive(Debug)]
        struct HexSlice<'a>(&'a [u8]);

        impl<'a> HexSlice<'a> {
            fn new<T>(data: &'a T) -> HexSlice<'a>
            where
                T: ?Sized + AsRef<[u8]> + 'a,
            {
                HexSlice(data.as_ref())
            }
        }

        trait HexDisplayExt {
            fn hex_display(&self) -> HexSlice<'_>;
        }

        impl<T> HexDisplayExt for T
        where
            T: ?Sized + AsRef<[u8]>,
        {
            fn hex_display(&self) -> HexSlice<'_> {
                HexSlice::new(self)
            }
        }

        fn example() {
            println!("{:?}", "world".hex_display());
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
        code4::test();
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
