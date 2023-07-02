use std::{collections::HashMap, hash::Hash};

use serde::{Deserialize, Serialize};
use serde_json::Value;

// example:
// "user": {
//     "id": 13416985,
//     "email": "9211080n0225@njust.edu.cn",
//     "name": "LRD",
//     "kindle_email": "",
//     "remix_userkey": "7470fa8c59efc4d3fe0a8b439de0f90d",
//     "donations_active": null,
//     "donations_expire": null,
//     "downloads_today": 0,
//     "downloads_limit": 10,
//     "confirmed": 1,
//     "isPremium": 0,
//     "personalDomains": [
//         "lib-vyfxb4urfpft3qi5b43ewpgh.fit.wf",
//         "lib-bxtnqoxeg7dxe37xluhzbydh.yeah.pw"
//     ]
// }
#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: i32,
    email: String,
    name: String,
    kindle_email: String,
    remix_userkey: String,
    donations_activate: Option<String>,
    donations_expire: Option<String>,
    downloads_today: i32,
    downloads_limit: i32,
    confirmed: i32,
    isPremium: i32,
    personalDomains: Vec<String>,
}

// example:
// {
//     "id": 438227,
//     "title": "Teach Yourself Electricity and Electronics",
//     "author": "Stan Gibilisco",
//     "cover": "https://static.singlelogin.re/covers299/books/eb/16/85/eb1685e7ec252888c2855a9646884f9b.jpg",
//     "hash": "c44dd4"
// }
#[derive(Debug, Serialize, Deserialize)]
pub struct Book {
    id: i32,
    title: String,
    author: String,
    cover: String, // url
    hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Info {
    books: i32,
    articles: i32,
    plans: Value,
    languages: HashMap<String, String>,
    extensions: Vec<String>,
    banner: String,
    service: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Format {}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Domain {
    domain: String,
    contentAvailable: bool,
    premium: bool,
    isRedirector: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DoaminList {
    success: i32,
    domains: Vec<Domain>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Extensions {
    success: i32,
    extensions: Vec<String>
}

// I don't know if is nesseary
//
// #[derive(Debug, Serialize, Deserialize)]
// pub struct ZWapper<T> {
//     success: i32,
//     body: T
// }
