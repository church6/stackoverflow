// @filename   : stackoverflow_0113_32552593.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/32552593
// @title      : Is it possible for one struct to extend an existing struct, keeping all the fields?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        // Structural composition

        struct Person {
            age: u8,
        }

        struct Child {
            person: Person,
            has_toy: bool,
        }
        // #[warn(clippy::redundant_field_names)]
        impl Person {
            fn new(age: u8) -> Self {
                Person { age }
            }

            fn age(&self) -> u8 {
                self.age
            }
        }

        impl Child {
            fn new(age: u8, has_toy: bool) -> Self {
                Child {
                    person: Person::new(age),
                    has_toy,
                }
            }

            fn age(&self) -> u8 {
                self.person.age()
            }
        }
        pub fn test() {
            // add your code here
            let p = Person::new(42);
            let c = Child::new(7, true);

            println!("I am {}", p.age());
            println!("My child is {}", c.age());
        }
    }
    mod code2 {
        // Traits

        trait SayHi {
            fn say_hi(&self);
        }

        struct Person {
            age: u8,
        }

        struct Child {
            age: u8,
            has_toy: bool,
        }

        impl SayHi for Person {
            fn say_hi(&self) {
                println!("Greetings. I am {}", self.age)
            }
        }

        impl SayHi for Child {
            fn say_hi(&self) {
                if self.has_toy {
                    println!("I'm only {}, but I have a toy!", self.age)
                } else {
                    println!("I'm only {}, and I don't even have a toy!", self.age)
                }
            }
        }

        fn greet<T>(thing: T)
        where
            T: SayHi,
        {
            thing.say_hi()
        }
        pub fn test() {
            // add your code here
            let p = Person { age: 42 };
            let c = Child {
                age: 7,
                has_toy: true,
            };

            greet(p);
            greet(c);
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
mod answer2 {
    mod code1 {
        struct StructA;

        impl StructA {
            fn name(&self) -> &'static str {
                "Anna"
            }
        }

        struct StructB {
            a: StructA,
            // other fields...
        }

        impl std::ops::Deref for StructB {
            type Target = StructA;
            fn deref(&self) -> &Self::Target {
                &self.a
            }
        }
        pub fn test() {
            // add your code here
            let b = StructB { a: StructA };
            println!("{}", b.name());
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
    answer2::test();
    //answer3::test();
    _leave!();
}
