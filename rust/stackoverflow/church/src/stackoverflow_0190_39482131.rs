// @filename   : stackoverflow_0190_39482131.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/39482131
// @title      : Is it possible to use `impl Trait` as a function's return type in a trait definition?

#[allow(dead_code)]
mod answer1 {
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
        trait A {
            fn new<S: Into<String>>(name: S) -> Self;
            fn get_name(&self) -> String;
        }

        struct Person {
            name: String,
        }

        impl A for Person {
            fn new<S: Into<String>>(name: S) -> Person {
                Person { name: name.into() }
            }

            fn get_name(&self) -> String {
                self.name.clone()
            }
        }

        struct Pet {
            name: String,
        }

        impl A for Pet {
            fn new<S: Into<String>>(name: S) -> Pet {
                Pet { name: name.into() }
            }

            fn get_name(&self) -> String {
                self.name.clone()
            }
        }

        fn example() {
            let person = Person::new("Simon");
            let pet = Pet::new("Buddy");

            println!("{}'s pets name is {}", get_name(&person), get_name(&pet));
        }

        fn get_name<T: A>(a: &T) -> String {
            a.get_name()
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
    //answer1::test();
    //answer2::test();
    answer3::test();
    _leave!();
}
