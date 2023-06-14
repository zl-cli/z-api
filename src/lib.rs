use serde_json::{Result, Value};
use z_api_def::{Api, Get, Post};

pub mod model;

use model::*;


/// The api definitions.
#[Api]
pub trait ZApi {
    #[Post("/eapi/user/login")]
    pub fn login(&self, email: String, password: String) -> Result<Value>;

    #[Get("/eapi/user/profile")]
    pub fn get_profile(&self) -> Result<Value>;

    #[Get("/eapi/book/recently")]
    pub fn get_book_recently(&self) -> Result<Vec<Book>>;

    #[Get("/eapi/book/recommended")]
    pub fn get_book_recommended(&self) -> Result<Vec<Book>>;

    #[Get("/eapi/book/most-popular")]
    pub fn get_book_most_popular(&self, switch_language: String) -> Result<Vec<Book>>;

    #[Get("/eapi/book/{id}/{hash}/send-to-{type}")]
    pub fn send_book_to_device(&self, id: i32, hash: i32, device:String) -> Result<()>;

    
}
