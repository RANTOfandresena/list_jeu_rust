
use lazy_static::lazy_static;

lazy_static!{
    pub static ref ADDRESS: String = set_adress();
    pub static ref PORT: u16 = set_port();
    pub static ref DATABASE_URL: String = set_database_url();
    pub static ref SECRET: String = set_secret();
}

fn set_adress() -> String {
    dotenv::dotenv().ok();
    std::env::var("ADDRESS").unwrap_or("localhost".to_string())
}

fn set_port() -> u16 {
    dotenv::dotenv().ok();
    std::env::var("PORT")
        .unwrap_or("5050".to_owned()).parse::<u16>()
        .expect("erreur Port")
}
fn set_database_url() -> String {
    dotenv::dotenv().ok();
    std::env::var("DATABASE_URL").unwrap()

}
fn set_secret() -> String {
    dotenv::dotenv().ok();
    std::env::var("SECRET").unwrap_or("SECRET".to_string())

}