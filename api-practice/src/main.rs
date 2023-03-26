use std::env;
use std::process;

fn main() -> std::io::Result<()> {
    // let env_variables = read_file("src/.env")?;
    // assert_eq!(&env_variables["KEY"], "YOUR_CLIENT_ID");


    let api_key_name = "KEY";
    let api_key = match env::var(api_key_name){
        Ok(val) => val,
        Err(err) => {
            println!("{}: KEY",err);
            process::exit(1);
        },
    };
    let api_url = "URL";
    println!("{}",api_key);
    Ok(())
}
