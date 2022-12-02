// @filename   : stackoverflow_0005_34733811.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/34733811
// @title      : What is the difference between iter and into_iter?

#[allow(dead_code)]
mod answer1 {
    /*
    TL;DR:
    1)The iterator returned by into_iter may yield any of T, &T or &mut T, depending on the context.
    2)The iterator returned by iter will yield &T, by convention.
    3)The iterator returned by iter_mut will yield &mut T, by convention.
    */

    /*
    into_iter comes from the IntoIterator trait:

    pub trait IntoIterator
    where
        <Self::IntoIter as Iterator>::Item == Self::Item,
    {
        type Item;
        type IntoIter: Iterator;
        fn into_iter(self) -> Self::IntoIter;
    }

    */
    mod code1 {
        pub fn test() {
            // add your code here
            // Vec example
            let vec1 = vec![1, 2, 3];
            let vec2 = vec![4, 5, 6];

            // `iter()` for vecs yields `&i32`. Destructure to `i32`.
            println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));
            // `into_iter()` for vecs yields `i32`. No destructuring required.
            println!("2 in vec2: {}", vec2.into_iter().any(|x| x == 2));

            // Array example
            let array1 = [1, 2, 3];
            let array2 = [4, 5, 6];

            // `iter()` for arrays yields `&i32`.
            println!("2 in array1: {}", array1.iter().any(|&x| x == 2));
            // `into_iter()` for arrays unusually yields `&i32`.
            println!("2 in array2: {}", array2.into_iter().any(|x| x == 2));
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
            let v = vec![1, 2];

            // Below is equivalent to: `for item in v.iter() {`
            for item in &v {
                println!("{}", item);
            }
        }
    }
    mod code2 {
        pub fn test() {
            // add your code here
            let v = vec![1, 2];

            let a: Vec<i32> = v.iter().map(|x| x * x).collect();
            // Although above and below are equivalent, above is a lot clearer than below.
            // `.into_iter()` call is equivalent to `.iter()` and will not consume the `Vec`
            // let b: Vec<i32> = (&v).into_iter().map(|x| x * x).collect();
            // this expression borrows a value the compiler would automatically borrow
            // let b: Vec<i32> = (&v).iter().map(|x| x * x).collect();
            let b: Vec<i32> = v.iter().map(|x| x * x).collect();
            println!("a={:?}", a);
            println!("b={:?}", b);
        }
    }
    mod code3 {
        pub fn test() {
            // add your code here
            // Similarly, in for loops, v.iter_mut() can be replaced with &mut v:
            let mut v = vec![1, 2];

            // Below is equivalent to: `for item in v.iter_mut() {`
            for item in &mut v {
                *item *= 2;
                println!("{}", *item);
            }
        }
    }
    pub fn test() {
        code1::test();
        code2::test();
        code3::test();
    }
}
pub fn test() {
    _enter!();
    answer1::test();
    //answer2::test();
    answer3::test();
    _leave!();
}
