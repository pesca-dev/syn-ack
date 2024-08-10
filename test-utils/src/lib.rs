mod framework;

pub use self::framework::*;

#[derive(Clone, Copy, Default)]
pub struct Setup;

impl Setup {
    pub fn set_env(self, vars: Vec<(&str, &str)>) -> Self {
        for (key, value) in vars.iter() {
            unsafe {
                std::env::set_var(key, value);
            }
        }

        self
    }

    pub fn env(self) -> Env {
        Env
    }
}

pub fn setup() -> Setup {
    dotenv::dotenv().ok();

    Setup
}

#[derive(Clone, Copy, Default)]
pub struct Env;

impl Env {
    pub fn setup(self) -> Env {
        dotenv::dotenv().ok();
        Env
    }

    pub fn set(self, key: impl ToString, value: impl ToString) -> Self {
        unsafe {
            std::env::set_var(key.to_string(), value.to_string());
        }
        self
    }
}

pub fn env() -> Env {
    Env
}

pub async fn client() -> Result<syn_ack::rocket::local::asynchronous::Client, syn_ack::rocket::Error>
{
    syn_ack::rocket::local::asynchronous::Client::tracked(syn_ack::start().await).await
}
