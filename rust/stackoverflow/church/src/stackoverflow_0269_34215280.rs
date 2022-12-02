// @filename   : stackoverflow_0269_34215280.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/34215280
// @title      : How can I randomly select one element from a vector or array?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        use rand::seq::SliceRandom; // 0.7.2

        fn example() {
            let vs = vec![0, 1, 2, 3, 4];
            println!("{:?}", vs.choose(&mut rand::thread_rng()));
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
        //Using choose_multiple:
        use rand::seq::SliceRandom; // 0.7.2

        fn example() {
            let samples = vec!["hi", "this", "is", "a", "test!"];
            let sample: Vec<_> = samples
                .choose_multiple(&mut rand::thread_rng(), 1)
                .collect();
            println!("{:?}", sample);
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
        use rand::distributions::WeightedIndex;
        use rand::prelude::*;
        fn example() {
            let choices = ['a', 'b', 'c'];
            let weights = [2, 1, 1];
            let dist = WeightedIndex::new(&weights).unwrap();
            let mut rng = thread_rng();
            for _ in 0..100 {
                // 50% chance to print 'a', 25% chance to print 'b', 25% chance to print 'c'
                println!("{}", choices[dist.sample(&mut rng)]);
            }

            let items = [('a', 0), ('b', 3), ('c', 7)];
            let dist2 = WeightedIndex::new(items.iter().map(|item| item.1)).unwrap();
            for _ in 0..100 {
                // 0% chance to print 'a', 30% chance to print 'b', 70% chance to print 'c'
                println!("{}", items[dist2.sample(&mut rng)].0);
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
mod answer4 {
    use random_choice::random_choice; // v0.3.2

    fn example() {
        let /*mut*/ samples = vec!["hi", "this", "is", "a", "test!"];
        let weights: Vec<f64> = vec![5.6, 7.8, 9.7, 1.1, 2.0];

        let number_choices = 100;
        let choices = random_choice().random_choice_f64(&samples, &weights, number_choices);

        for choice in choices {
            print!("{}, ", choice);
        }
    }

    pub fn test() {
        example();
    }
}
mod answer5 {
    fn example() {
        let mut vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

        let index = (rand::random::<f32>() * vec.len() as f32).floor() as usize;
        let value = vec.remove(index);

        println!("index: {} value: {}", index, value);
        println!("{:?}", vec);
    }

    pub fn test() {
        example();
    }
}
mod answer6 {
    use rand::Rng;
    fn example() {
        let mut rng = rand::thread_rng();

        let my_strings: Vec<&str> = vec!["a", "b", "c"];
        let random_string_index: usize = rng.gen_range(0..my_strings.len());
        let string = my_strings[random_string_index];
        println!("{:?}", string);
    }
    pub fn test() {
        example();
    }
}
pub fn test() {
    _enter!();
    answer1::test();
    answer2::test();
    answer3::test();
    answer4::test();
    answer5::test();
    answer6::test();
    _leave!();
}
