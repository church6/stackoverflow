// @filename   : stackoverflow_0282_30073684.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/30073684
// @title      : How to get mutable references to two array elements at the same time?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        fn change(a: &mut i32, b: &mut i32) {
            /*let c = *a;
             *a = *b;
             *b = c;
             */
            std::mem::swap(&mut (*a), &mut (*b));
        }
        fn example() {
            let mut v = vec![1, 2, 3];
            let (a, b) = v.split_at_mut(1); // Returns (&mut [1], &mut [2, 3])
            change(&mut a[0], &mut b[0]);
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
        fn change(a: &mut i32, b: &mut i32) {
            /*let c = *a;
             *a = *b;
             *b = c;
             */
            std::mem::swap(&mut (*a), &mut (*b));
        }

        fn example() {
            let mut arr = [5, 6, 7, 8];
            {
                let [ref mut a, _, ref mut b, ..] = arr;
                change(a, b);
            }
            assert_eq!(arr, [7, 6, 5, 8]);
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
mod answer3 {
    mod code1 {
        fn example() {
            let mut v = vec![1, 2, 3];
            v.swap(0, 1);
            println!("{:?}", v);
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
pub fn test() {
    _enter!();
    answer1::test();
    answer2::test();
    answer3::test();
    _leave!();
}
