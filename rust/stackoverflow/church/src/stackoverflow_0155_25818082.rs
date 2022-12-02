// @filename   : stackoverflow_0155_25818082.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/25818082
// @title      : Vector of objects belonging to a trait

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        trait Animal {
            fn make_sound(&self) -> String;
        }

        struct Cat;
        impl Animal for Cat {
            fn make_sound(&self) -> String {
                "meow".to_string()
            }
        }

        struct Dog;
        impl Animal for Dog {
            fn make_sound(&self) -> String {
                "woof".to_string()
            }
        }

        fn example() {
            let dog: Dog = Dog;
            let cat: Cat = Cat;
            let mut v: Vec<Box<dyn Animal>> = Vec::new();
            v.push(Box::new(cat));
            v.push(Box::new(dog));
            for animal in v.iter() {
                println!("{}", animal.make_sound());
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