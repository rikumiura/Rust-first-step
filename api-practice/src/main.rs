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

    // let mut api_key = "hoge".to_string();
    let mut api_key:String = "hoge".to_string();
    // println!("{}: {}", api_key, type_of(&api_key));

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
        // println!("{}: {}", item.0, type_of(&item.0));

    }
    // println!("key: {}", type_of(&"KEY".to_string()));
    println!("key: {}", api_key);
    Ok(())
}
