// @filename   : stackoverflow_0044_40792801.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/40792801
// @title      : Best way to concatenate vectors in Rust

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        pub fn test() {
            // add your code here
            let mut a = vec![1, 2, 3];
            let mut b = vec![4, 5, 6];

            a.append(&mut b);

            assert_eq!(a, [1, 2, 3, 4, 5, 6]);
            assert_eq!(b, [] as [i32; 0]);
        }
    }
    mod code2 {
        pub fn test() {
            // add your code here
            let mut a = vec![1, 2, 3];
            let b = vec![4, 5, 6];

            a.extend(b);
            assert_eq!(a, [1, 2, 3, 4, 5, 6]);
            // b is moved and can't be used anymore
        }
    }
    mod code3 {
        pub fn test() {
            // add your code here
            let mut a = vec![1, 2, 3];
            let b = vec![4, 5, 6];

            a.extend(&b);
            assert_eq!(a, [1, 2, 3, 4, 5, 6]);
            assert_eq!(b, [4, 5, 6]);
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
            let a = vec![1, 2, 3];
            let b = vec![4, 5, 6];
            let c: Vec<i32> = a.into_iter().chain(b.into_iter()).collect(); // Consumed
            println!("{:?}", c);

            let a = vec![1, 2, 3];
            let b = vec![4, 5, 6];
            let c: Vec<&i32> = a.iter().chain(b.iter()).collect(); // Referenced
            println!("{:?}", c);

            let a = vec![1, 2, 3];
            let b = vec![4, 5, 6];
            let c: Vec<i32> = a.iter().cloned().chain(b.iter().cloned()).collect(); // Cloned
            println!("{:?}", c);

            let a = vec![1, 2, 3];
            let b = vec![4, 5, 6];
            let c: Vec<i32> = a.iter().copied().chain(b.iter().copied()).collect(); // Copied
            println!("{:?}", c);
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
    #[allow(soft_unstable)]
    mod code1 {
        /*
                //#![feature(test)]
                #[bench]
                fn bench_concat___init__(b: &mut Bencher) {
                    b.iter(|| {
                        let mut x = vec![1i32; 100000];
                        let mut y = vec![2i32; 100000];
                    });
                }

                #[bench]
                fn bench_concat_append(b: &mut Bencher) {
                    b.iter(|| {
                        let mut x = vec![1i32; 100000];
                        let mut y = vec![2i32; 100000];
                        x.append(&mut y)
                    });
                }

                #[bench]
                fn bench_concat_extend(b: &mut Bencher) {
                    b.iter(|| {
                        let mut x = vec![1i32; 100000];
                        let mut y = vec![2i32; 100000];
                        x.extend(y)
                    });
                }

                #[bench]
                fn bench_concat_concat(b: &mut Bencher) {
                    b.iter(|| {
                        let mut x = vec![1i32; 100000];
                        let mut y = vec![2i32; 100000];
                        [x, y].concat()
                    });
                }

                #[bench]
                fn bench_concat_iter_chain(b: &mut Bencher) {
                    b.iter(|| {
                        let mut x = vec![1i32; 100000];
                        let mut y = vec![2i32; 100000];
                        x.into_iter().chain(y.into_iter())
                    });
                }

                #[bench]
                fn bench_concat_iter_chain_collect(b: &mut Bencher) {
                    b.iter(|| {
                        let mut x = vec![1i32; 100000];
                        let mut y = vec![2i32; 100000];
                        x.into_iter().chain(y.into_iter()).collect::<Vec<i32>>()
                    });
                }
        */

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
mod answer4 {
    pub fn test() {
        let first_number: Vec<usize> = Vec::from([0]);
        let final_number: Vec<usize> = Vec::from([3]);
        let middle_numbers: Vec<usize> = Vec::from([1, 2]);

        let numbers = [first_number, middle_numbers, final_number].concat();
        println!("{:?}", numbers);
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
