use serde_json::{Result, Value};
use z_api_def::{Api, Get, Post};

pub mod model;

use model::*;

/// The api definitions.
#[allow(non_snake_case, dead_code)]
#[Api]
pub trait ZApi {
    /*
     * User Post Api
     */
    #[Post("/eapi/user/login")]
    fn login(&self, email: String, password: String) -> Result<Value>;

    #[Post("/eapi/user/update")]
    fn user_update(
        &self,
        email: String,
        password: String,
        name: String,
        kindle_email: String,
    ) -> Result<()>;

    #[Post("/eapi/user/token-sign-in")]
    fn token_sign_in(&self, id: i32, remix_userkey: String) -> Result<()>;

    #[Post("/eapi/user/email/confirmation/resend")]
    fn resend_confirmation_email(&self) -> Result<()>;

    #[Post("/eapi/user/registeration")]
    fn registeration(&self, email: String, password: String, name: String) -> Result<()>;

    #[Post("/eapi/user/password-recovery")]
    fn password_recovery(&self, email: String) -> Result<()>;

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

    // language: zh us
    #[Get("/eapi/book/most-popular?switchLanguage={switch_language}")]
    fn get_book_most_popular(&self, switch_language: String) -> Result<Vec<Book>>;

    #[Get("/eapi/book/{id}/{hash}/send-to-{device}")]
    fn send_book_to_device(&self, id: i32, hash: i32, device: String) -> Result<()>;

    #[Get("/eapi/book/{id}/{hash}?switchLanguage={switch_language}")]
    fn get_book(&self, id: i32, hash: i32, switch_language: String) -> Result<Book>;

    #[Get("/eapi/book/{id}/{hash}/similar")]
    fn get_similar_books(&self, id: i32, hash: i32) -> Result<Vec<Book>>;

    #[Get("/eapi/book/{id}/{hash}/formats")]
    fn get_book_formats(&self, id: i32, hash: i32) -> Result<Vec<Format>>;

    /*
     * Book Post Api
     */
    #[allow(non_snake_case)]
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

    /*
     * Info Get Apis
     */
    #[Get("/eapi/info/domains")]
    fn get_domains(&self) -> Result<DoaminList>;

    #[Get("/eapi/info/extensions")]
    fn get_extensions(&self) -> Result<Extensions>;

}
