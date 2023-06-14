use serde::{Deserialize,Serialize};

use z_api_def::{Api, Post, Get};
use serde_json::{Result, Value};

#[derive(Debug,Serialize, Deserialize)]
pub struct Book {

}

/// The api definitions.
#[Api]
pub trait  ZApi{
    // #[Post("/eapi/user/login")]
    // fn login(&self, email: String, password: String) -> Result<Value>;

    #[Get("/eapi/user/profile")]
    fn get_profile(&self) -> Result<Value>;

    #[Get("/eapi/book/recently")]
    fn get_book_recently(&self) -> Result<Vec<Book>>;
}