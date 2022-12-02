// @filename   : stackoverflow_0147_30292752.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/30292752
// @title      : How do I parse a JSON File?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        fn example() {
            let the_file = r#"{
        "FirstName": "John",
        "LastName": "Doe",
        "Age": 43,
        "Address": {
            "Street": "Downing Street 10",
            "City": "London",
            "Country": "Great Britain"
        },
        "PhoneNumbers": [
            "+44 1234567",
            "+44 2345678"
        ]
    }"#;

            let _json: serde_json::Value =
                serde_json::from_str(the_file).expect("JSON was not well-formatted");
            //println!("{:?}", json);
        }
        pub fn test() {
            // add your code here
            example();
        }
    }
    mod code2 {
        use serde::Deserialize;

        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "PascalCase")]
        struct Person {
            first_name: String,
            last_name: String,
            age: u8,
            address: Address,
            phone_numbers: Vec<String>,
        }

        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "PascalCase")]
        struct Address {
            street: String,
            city: String,
            country: String,
        }

        fn example() {
            let the_file = "../example.json";

            let person: Person =
                serde_json::from_str(the_file).expect("JSON was not well-formatted");
            println!("{:?}", person)
        }

        pub fn test() {
            // add your code here
            example();
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
