// @filename   : stackoverflow_0256_27305585.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/27305585
// @title      : What is the difference between passing a value to a function by reference and passing it by Box?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        fn example() {
            let mut heap_a = Box::new(3);
            #[allow(clippy::explicit_auto_deref)]
            foo(&mut *heap_a);
            println!("{}", heap_a);

            let heap_b = Box::new(3);
            bar(heap_b);
            // can't use `heap_b`. `heap_b` has been deallocated at the end of `bar`
            // println!("{}", heap_b);
        } // `heap_a` is destroyed here

        fn foo(x: &mut i32) {
            *x = 5;
        }

        #[allow(clippy::boxed_local)]
        fn bar(mut x: Box<i32>) {
            *x = 5;
        } // heap_b (now `x`) is deallocated here

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
