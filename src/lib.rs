use serde_json::{Result, Value};
use std::time::Instant;
use z_api_def::{Api, Get, Post};

pub mod model;

use model::*;

/// The api definitions.
#[Api]
pub trait ZApi {
    /*
     * User Post Api
     */
    #[Post("/eapi/user/login")]
    fn login(&self, email: String, password: String) -> Result<Value>;

    /*
     * User Get Api
     */
    #[Get("/eapi/user/profile")]
    fn get_profile(&self) -> Result<Value>;

    #[Get("/eapi/user/book/recommended")]
    fn get_user_book_recommended(&self) -> Result<Vec<Book>>;

    /*
     * Book Get Api
     */
    #[Get("/eapi/book/recently")]
    fn get_book_recently(&self) -> Result<Vec<Book>>;

    #[Get("/eapi/book/most-popular")]
    fn get_book_most_popular(&self, switch_language: String) -> Result<Vec<Book>>;

    #[Get("/eapi/book/{id}/{hash}/send-to-{device}")]
    fn send_book_to_device(&self, id: i32, hash: i32, device: String) -> Result<()>;

    #[Get("/eapi/book/{id}/{hash}")]
    fn get_book(&self, id: i32, hash: i32, switch_language: String) -> Result<Book>;

    #[Get("/eapi/book/{id}/{hash}/similar")]
    fn get_similar_books(&self, id: i32, hash: i32) -> Result<Vec<Book>>;

    #[Get("/eapi/book/{id}/{hash}/formats")]
    fn get_book_formats(&self, id: i32, hash: i32) -> Result<Vec<Format>>;

    /*
     * Book Post Api
     */
    #[Post("/eapi/book/search")]
    fn search_books(
        &self,
        message: String,
        yearFrom: String,
        yearTo: String,
        language: String,
        extensions: String,
        e: String,
        page: i32,
        limit: i32,
    ) -> Result<Vec<Book>>;
}
