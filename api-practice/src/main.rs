// use std::env;
// use std::process;
use dotenv;
use std::cmp::Ordering;


fn main() -> std::io::Result<()> {
    // let env_variables = read_file("src/.env")?;
    // assert_eq!(&env_variables["KEY"], "YOUR_CLIENT_ID");


    // let key = "PATH";
    // match env::var_os(key) {
    //     Some(paths) => {
    //         for path in env::split_paths(&paths) {
    //             println!("'{}'", path.display());
    //         }
    //     }
    //     None => println!("{key} is not defined in the environment.")
    // }

    // dotenv().ok();

    let mut api_key = "hoge";

    dotenv::from_filename("api.env").ok();
    let result: Vec<(String, String)> = dotenv::vars().collect();
    for item in result{
        let mut comparison = item.0.cmp(&"KEY".to_string());
        if comparison ==  Ordering::Equal {
            let mut api_key = item.1.to_string();
        }
        println!("{}: {}", item.0, item.1);
    }
    println!("key: {}", api_key);
    Ok(())
}
