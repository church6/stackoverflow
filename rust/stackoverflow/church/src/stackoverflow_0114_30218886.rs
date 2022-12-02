// @filename   : stackoverflow_0114_30218886.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/30218886
// @title      : How to implement Iterator and IntoIterator for a simple struct?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        struct Pixel {
            r: i8,
            g: i8,
            b: i8,
        }

        impl IntoIterator for Pixel {
            type Item = i8;
            type IntoIter = PixelIntoIterator;

            fn into_iter(self) -> Self::IntoIter {
                PixelIntoIterator {
                    pixel: self,
                    index: 0,
                }
            }
        }

        pub struct PixelIntoIterator {
            pixel: Pixel,
            index: usize,
        }

        impl Iterator for PixelIntoIterator {
            type Item = i8;
            fn next(&mut self) -> Option<i8> {
                let result = match self.index {
                    0 => self.pixel.r,
                    1 => self.pixel.g,
                    2 => self.pixel.b,
                    _ => return None,
                };
                self.index += 1;
                Some(result)
            }
        }

        fn example() {
            let p = Pixel {
                r: 54,
                g: 23,
                b: 74,
            };
            for component in p {
                println!("{}", component);
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
