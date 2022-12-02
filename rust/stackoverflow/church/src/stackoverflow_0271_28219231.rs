// @filename   : stackoverflow_0271_28219231.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/28219231
// @title      : How to idiomatically copy a slice?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        fn example() {
            let mut x = vec![0; 8];
            let y = [1, 2, 3];
            x[..3].clone_from_slice(&y);
            println!("{:?}", x);
            // Output:
            // [1, 2, 3, 0, 0, 0, 0, 0]
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
        fn copy_slice(dst: &mut [u8], src: &[u8]) -> usize {
            let mut c = 0;
            for (d, s) in dst.iter_mut().zip(src.iter()) {
                *d = *s;
                c += 1;
            }
            c
        }

        fn example() {
            let x: [u8; 4] = [1u8, 2u8, 3u8, 4u8];
            //x[0] = 1u8;
            //x[1] = 2u8;
            //x[2] = 3u8;
            //x[3] = 4u8;

            let mut y: [u8; 8] = [0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8];
            copy_slice(&mut y, &x);
            //println!("{:?}", x);
            //println!("{:?}", y);
            assert_eq!(vec![1u8, 2u8, 3u8, 4u8], x);
            assert_eq!(vec![1u8, 2u8, 3u8, 4u8, 0u8, 0u8, 0u8, 0u8], y);
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
