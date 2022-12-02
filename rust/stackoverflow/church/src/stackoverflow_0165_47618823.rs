// @filename   : stackoverflow_0165_47618823.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/47618823
// @title      : Cannot borrow as mutable because it is also borrowed as immutable

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        // Minimal, Reproducible Example

        // This applies to the version of Rust this question
        // was asked about; see below for updated examples.
        fn example1() {
            let mut items = vec![1];
            let _item = items.last();
            items.push(2);
            println!("example1={:?}", items);
        }

        fn example2() {
            let mut items = vec![1];
            {
                let _item = items.last();
            }
            items.push(2);
            println!("example2={:?}", items);
        }

        fn example3() {
            let mut items = vec![1];
            let _item = items.last();
            items.push(2);
            println!("example3={:?}", items);
        }

        fn example4() {
            let mut items = vec![1];
            let _item = items.last().cloned();
            items.push(2);
            println!("example4={:?}", items);
        }

        #[derive(Debug)]
        struct NonClone;

        use std::rc::Rc;

        fn example5() {
            let mut items = vec![Rc::new(NonClone)];
            let _item = items.last().cloned();
            items.push(Rc::new(NonClone));
            println!("example5={:?}", items);
        }

        pub fn test() {
            // add your code here
            example1();
            example2();
            example3();
            example4();
            example5();
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
