// @filename   : stackoverflow_0270_30353462.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/30353462
// @title      : How to clone a struct storing a boxed trait object?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        trait Animal: AnimalClone {
            fn speak(&self);
        }

        // Splitting AnimalClone into its own trait allows us to provide a blanket
        // implementation for all compatible types, without having to implement the
        // rest of Animal.  In this case, we implement it for all types that have
        // 'static lifetime (*i.e.* they don't contain non-'static pointers), and
        // implement both Animal and Clone.  Don't ask me how the compiler resolves
        // implementing AnimalClone for dyn Animal when Animal requires AnimalClone;
        // I have *no* idea why this works.
        trait AnimalClone {
            fn clone_box(&self) -> Box<dyn Animal>;
        }

        impl<T> AnimalClone for T
        where
            T: 'static + Animal + Clone,
        {
            fn clone_box(&self) -> Box<dyn Animal> {
                Box::new(self.clone())
            }
        }

        // We can now implement Clone manually by forwarding to clone_box.
        impl Clone for Box<dyn Animal> {
            fn clone(&self) -> Box<dyn Animal> {
                self.clone_box()
            }
        }

        #[derive(Clone)]
        struct Dog {
            name: String,
        }

        impl Dog {
            fn new(name: &str) -> Dog {
                Dog {
                    name: name.to_string(),
                }
            }
        }

        impl Animal for Dog {
            fn speak(&self) {
                println!("{}: ruff, ruff!", self.name);
            }
        }

        #[derive(Clone)]
        struct AnimalHouse {
            animal: Box<dyn Animal>,
        }

        fn example() {
            let house = AnimalHouse {
                animal: Box::new(Dog::new("Bobby")),
            };
            // #[warn(clippy::redundant_clone)]
            // let house2 = house.clone();
            let house2 = house;
            house2.animal.speak();
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
        use std::rc::Rc;

        trait Animal {
            fn speak(&self);
        }

        #[derive(Clone)]
        struct Dog {
            name: String,
        }

        impl Dog {
            fn new(name: &str) -> Dog {
                Dog {
                    name: name.to_string(),
                }
            }
        }

        impl Animal for Dog {
            fn speak(&self) {
                println!("{}: ruff, ruff!", self.name);
            }
        }

        #[derive(Clone)]
        struct AnimalHouse {
            animal: Rc<dyn Animal>,
        }

        fn example() {
            let house = AnimalHouse {
                animal: Rc::new(Dog::new("Bobby")),
            };
            // #[warn(clippy::redundant_clone)]
            // let house2 = house.clone();
            let house2 = house;
            house2.animal.speak();
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
