// use std::env;
// use std::process;
use dotenv;
use std::cmp::Ordering;
use reqwest::Client;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn get_api_key_from_env() -> String {

    let mut api_key:String = "hoge".to_string();

    fn type_of<T>(_: T) -> String{
        let a = std::any::type_name::<T>();
        return a.to_string();
    }

    dotenv::from_filename("api.env").ok();
    let result: Vec<(String, String)> = dotenv::vars().collect();
    for item in result{
        let comparison = item.0.cmp(&"KEY".to_string());
        if comparison ==  Ordering::Equal {
            api_key = item.1.to_string();
        }

    }

    println!("key: {}", api_key);
    return api_key.to_string();
}



#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new(); // 1
    let url = "https://webservice.recruit.co.jp/hotpepper/gourmet/v1/?";
    let response = client
        .get(url)
        .query(&[("large_area", "Z011"), ("key",&get_api_key_from_env())])
        .send()
        .await?; // 2
    let body = response.text().await?; // 3
    println!("{}", body);
    Ok(())
}
