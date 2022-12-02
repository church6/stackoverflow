// @filename   : stackoverflow_0041_27893223.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/27893223
// @title      : How do I iterate over a range with a custom step?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        pub fn test() {
            // add your code here
            for x in (1..10).step_by(2) {
                println!("{}", x);
            }
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
        struct SimpleStepRange(isize, isize, isize); // start, end, and step

        impl Iterator for SimpleStepRange {
            type Item = isize;

            #[inline]
            fn next(&mut self) -> Option<isize> {
                if self.0 < self.1 {
                    let v = self.0;
                    self.0 = v + self.2;
                    Some(v)
                } else {
                    None
                }
            }
        }
        pub fn test() {
            // add your code here
            for i in SimpleStepRange(0, 10, 2) {
                println!("{}", i);
            }
        }
    }
    mod code2 {
        use std::ops::Add;

        struct StepRange<T>(T, T, T)
        where
            for<'a> &'a T: Add<&'a T, Output = T>,
            T: PartialOrd,
            T: Clone;

        impl<T> Iterator for StepRange<T>
        where
            for<'a> &'a T: Add<&'a T, Output = T>,
            T: PartialOrd,
            T: Clone,
        {
            type Item = T;

            #[inline]
            fn next(&mut self) -> Option<T> {
                if self.0 < self.1 {
                    let v = self.0.clone();
                    self.0 = &v + &self.2;
                    Some(v)
                } else {
                    None
                }
            }
        }
        pub fn test() {
            // add your code here
            for i in StepRange(0u64, 10u64, 2u64) {
                println!("{}", i);
            }
        }
    }
    mod code3 {
        pub fn test() {
            // add your code here
        }
    }
    pub fn test() {
        code1::test();
        code2::test();
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
