use rustify::core::auth::get_token;

use dotenv::from_path;

use std::env::var;

#[test]
#[should_panic]
fn err() {
    let path = r#"C:\Users\conno\rustify\tests\.env"#;
    from_path(path).unwrap();

    let id: String = var("USER_ID").unwrap();
    let secret: String = var("USER_SECRET").unwrap();

    let x = get_token(id.trim_matches('1').to_string(), secret);
    println!("{:?}", x);
}

#[test] 
fn work() {
    let path = r#"C:\Users\conno\rustify\tests\.env"#;
    from_path(path).unwrap();

    let id: String = var("USER_ID").unwrap();
    let secret: String = var("USER_SECRET").unwrap();

    let x = get_token(id, secret);
    println!("{:?}", x);
}