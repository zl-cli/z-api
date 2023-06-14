use serde_json::{Result, Value};
use z_api_def::{Api, Get, Post};

pub mod model;

use model::*;


/// The api definitions.
#[Api]
pub trait ZApi {
    #[Post("/eapi/user/login")]
    fn login(&self, email: String, password: String) -> Result<Value>;

    #[Get("/eapi/user/profile")]
    fn get_profile(&self) -> Result<Value>;


    /*
     * Book Get Api
     */

    #[Get("/eapi/book/recently")]
    fn get_book_recently(&self) -> Result<Vec<Book>>;

    #[Get("/eapi/book/recommended")]
    fn get_book_recommended(&self) -> Result<Vec<Book>>;

    #[Get("/eapi/book/most-popular")]
    fn get_book_most_popular(&self, switch_language: String) -> Result<Vec<Book>>;

    #[Get("/eapi/book/{id}/{hash}/send-to-{device}")]
    fn send_book_to_device(&self, id: i32, hash: i32, device:String) -> Result<()>;

    #[Get("/eapi/book/{id}/{hash}")]
    fn get_book(&self, id: i32, hash: i32, switch_language: String) -> Result<Book>;

    #[Get("/eapi/book/{id}/{hash}/similar")]
    fn get_similar_books(&self, id: i32, hash: i32) -> Result<Vec<Book>>;
    
}
