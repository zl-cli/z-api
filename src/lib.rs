extern crate serde_json;

use z_api_def::{Api, Post, Get};
use serde_json::{Result, Value};


/// The api definitions.
#[Api]
pub trait  ZApi{
    #[Post("/eapi/user/login")]
    fn login(&self, email: String, password: String) -> Result<Value>;

    #[Get("/eapi/user/profile")]
    fn get_profile(&self) -> Result<Value>;

}