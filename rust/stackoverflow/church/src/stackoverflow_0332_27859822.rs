// @filename   : stackoverflow_0332_27859822.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/27859822
// @title      : Is it possible to have stack allocated arrays with the size determined at runtime in Rust?

#[allow(dead_code)]
mod answer1 {
    mod code1 {

        enum SmallVector<T, const N: usize> {
            Inline(usize, [T; N]),
            Dynamic(Vec<T>),
        }

        impl<T: Copy + Clone, const N: usize> SmallVector<T, N> {
            fn new(v: T, n: usize) -> Self {
                if n <= N {
                    Self::Inline(n, [v; N])
                } else {
                    Self::Dynamic(vec![v; n])
                }
            }
        }

        impl<T, const N: usize> SmallVector<T, N> {
            fn as_slice(&self) -> &[T] {
                match self {
                    Self::Inline(n, array) => &array[0..*n],
                    Self::Dynamic(vec) => vec,
                }
            }

            fn as_mut_slice(&mut self) -> &mut [T] {
                match self {
                    Self::Inline(n, array) => &mut array[0..*n],
                    Self::Dynamic(vec) => vec,
                }
            }
        }

        use std::ops::{Deref, DerefMut};

        impl<T, const N: usize> Deref for SmallVector<T, N> {
            type Target = [T];

            fn deref(&self) -> &Self::Target {
                self.as_slice()
            }
        }

        impl<T, const N: usize> DerefMut for SmallVector<T, N> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                self.as_mut_slice()
            }
        }

        fn example() {
            let mut v: SmallVector<u32, 4> = SmallVector::new(1u32, 4);
            v[2] = 3;
            println!("{}: {}", v.len(), v[2])
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
