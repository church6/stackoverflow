// @filename   : stackoverflow_0103_14154753.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/14154753
// @title      : How do I make an HTTP request from Rust?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        use std::error::Error;

        // This requires the optional blocking feature to be enabled.
        fn http() -> Result<(), Box<dyn Error>> {
            let resp = reqwest::blocking::get("https://www.baidu.com")?.text()?;
            println!("{:#?}", resp);
            Ok(())
        }
        pub fn test() {
            // add your code here
            http().unwrap();
        }
    }
    #[allow(unused_must_use)]
    mod code2 {
        use std::error::Error;

        //#[tokio::main]
        async fn http() -> Result<(), Box<dyn Error>> {
            let resp = reqwest::get("https://www.baidu.com").await?.text().await?;
            println!("{:#?}", resp);
            Ok(())
        }
        pub fn test() {
            // add your code here
            http();
        }
    }
    #[allow(unused_imports)]
    #[allow(unused_must_use)]
    mod code3 {
        use hyper::{body::HttpBody as _, Client, Uri};
        use std::error::Error;

        //#[tokio::main]
        async fn http() -> Result<(), Box<dyn Error>> {
            let client = Client::new();

            let res = client
                .get(Uri::from_static("https://www.baidu.com"))
                .await?;

            println!("status: {}", res.status());

            let buf = hyper::body::to_bytes(res).await?;

            println!("body: {:?}", buf);
            Ok(())
        }

        pub fn test() {
            // add your code here
            http();
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
