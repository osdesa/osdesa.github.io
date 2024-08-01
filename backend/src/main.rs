use std::fs;

fn main() {
    println!("Starting server");
    
    get_cv("config/config.env", "cv.pdf");
}

fn get_cv(config_path: &str, file_name: &str) {
    let token = get_access_token(config_path);
    
}

fn get_access_token(config_path: &str) -> String{
    let contents = fs::read_to_string(config_path)
        .expect("Should have been able to read the file");

    let parts = contents.split(":");

    String::from(parts.collect::<Vec<_>>()[1])
}

