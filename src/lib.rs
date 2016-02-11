#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_must_use)]

//! Dropbox SDK for Rust
//!
//! This crate implements a simple client for the [Dropbox API](https://dropbox.com/developers).
//! It uses [hyper](https://hyperium.github.io) for HTTP, though that part is swappable if you would
//! like to use a different HTTP implementation.
//!
//! If you want to use hyper, just `use dbox::client::Client`, as in the example below. If,
//! however, you want to use something else, you will need to implement the `DropboxClient` trait
//! for your data structure, build this crate with `--no-default-features`, and pass a reference to
//! your client as the first parameter to the API functions.
//!
//! # Example
//!
//! ```ignore
//! extern crate dbox;
//!
//! use std::env;
//! use dbox::client::Client;
//! use dbox::files;
//!
//! let access_token = env::var("DROPBOX_TOKEN");
//! let client = Client::new(access_token);
//! let folder_list = files::copy_(&client, "/path/to/existing/file", "/path/to/new/file");
//! ```
//!

#[cfg(feature = "hyper-client")] extern crate hyper;
extern crate chrono;
extern crate rustc_serialize;

#[cfg(test)] extern crate rand;

use std::convert::From;
use std::fmt;
use std::collections::BTreeMap;

#[doc(hidden)]
pub enum Endpoint {
    Api,
    Content,
}

impl fmt::Display for Endpoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Endpoint::Api => write!(f, "api"),
            Endpoint::Content => write!(f, "content"),
        }
    }
}

pub trait DropboxClient {
    fn access_token(&self) -> &str;
    fn request<T>(&self, endpoint: Endpoint, url: &str, headers: &mut BTreeMap<String, String>, body: Option<T>) -> Result<Response>
            where T: rustc_serialize::Encodable + Clone;

    fn api<T>(&self, url: &str, headers: &mut BTreeMap<String, String>, body: Option<T>) -> Result<Response>
            where T: rustc_serialize::Encodable + Clone
    {
        self.request(Endpoint::Api, url, headers, body)
    }

    fn content<T>(&self, url: &str, headers: &mut BTreeMap<String, String>, body: Option<T>) -> Result<Response>
            where T: rustc_serialize::Encodable + Clone
    {
        self.request(Endpoint::Content, url, headers, body)
    }
}

/// Collection of possible errors
#[derive(Debug, PartialEq, Clone)]
pub enum ApiError {
    AddFolderMemberError,
    ClientError,
    CreateFolderError,
    CreateSharedLinkError,
    DeleteError,
    DownloadError,
    GetAccountError,
    GetAccountBatchError,
    GetMetadataError,
    GetSharedLinksError,
    ListFolderError,
    ListFolderContinueError,
    ListFolderLongpollError,
    ListFolderMembersContinueError,
    ListFoldersContinueError,
    ListRevisionsError,
    MountFolderError,
    PollError,
    PreviewError,
    RelinquishFolderMembershipError,
    RelocationError,
    RemoveFolderMemberError,
    RestoreError,
    RevokeSharedLinkError,
    SearchError,
    ShareFolderError,
    SharedFolderAccessError,
    ThumbnailError,
    TokenError,
    TransferFolderError,
    UnmountFolderError,
    UnshareFolderError,
    UpdateFolderMemberError,
    UpdateFolderPolicyError,
    UploadError,
    UploadSessionLookupError,
    UploadSessionFinishError,
}

impl From<rustc_serialize::json::DecoderError> for ApiError {
    fn from(e: rustc_serialize::json::DecoderError) -> ApiError {
        ApiError::ClientError
    }
}

/// Simple abstraction of a HTTP response, to allow the HTTP client to be pluggable
///
/// TODO: overhaul this
#[derive(Debug, PartialEq, Clone)]
pub struct Response {
    pub status: u16,
    pub api_result: Option<String>,
    pub body: String,
}

pub type Result<T> = ::std::result::Result<T, ApiError>;

#[cfg(feature = "hyper-client")]
/// Default implementation of the Dropbox Client
pub mod client;
/// Module for performing operations on files in a dropbox account
pub mod files;
/// Module that holds definitions for dropbox data structures
pub mod structs;
/// TODO
pub mod sharing;
/// TODO
pub mod users;

#[cfg(test)]
mod tests {
    use super::client::Client;
    use super::files;
    use super::DropboxClient;
    use chrono::{DateTime, Local};
    use rand;
    use std::str;
    use std::env;
    use std::default::Default;
    use rustc_serialize::json;

    const ACCESS_TOKEN: &'static str = "DROPBOX_TOKEN";
    const MALFORMED_TOKEN: &'static str = "asdf";

    fn random_ascii_letters(len: usize) -> String {
        let mut rng = rand::thread_rng();
        let ascii_letters = (65..91).chain((97..123)).collect::<Vec<u8>>();
        let random_bytes = rand::sample(&mut rng, ascii_letters.into_iter(), len);
        let random_string = str::from_utf8(&random_bytes).unwrap();
        random_string.to_owned()
    }

    #[test]
    fn test_bad_auth() {
        if let Ok(_) = Client::new(MALFORMED_TOKEN) {
            panic!("Malformed token should not be accepted");
        }

        let valid_looking_token = vec!["z"; 62].join("");
        let client = Client::new(&valid_looking_token).unwrap();
        if let Ok(_) = files::list_folder(&client, "") {
            panic!("Valid-looking but invalid token should not be accepted");
        }
    }

    #[test]
    fn test_list_rpc() {
        let access_token = match env::var(ACCESS_TOKEN) {
            Ok(t) => t,
            Err(_) => panic!("No {} found", ACCESS_TOKEN),
        };
        let client = Client::new(&access_token).unwrap();
        files::create_folder(&client, "");
        assert!(files::list_folder(&client, "").is_ok());
    }

    #[test]
    fn test_upload_download() {
        let access_token = match env::var(ACCESS_TOKEN) {
            Ok(t) => t,
            Err(_) => panic!("No {} found", ACCESS_TOKEN),
        };
        let random_filename = random_ascii_letters(15);
        let now: DateTime<Local> = Local::now();
        let now = format!("{}", now.timestamp());
        let random_path = format!("/Test/{}/{}", now, random_filename);
        let random_contents = random_ascii_letters(20);

        let client = Client::new(&access_token).unwrap();
        assert!(files::upload(&client, &random_contents, &random_path).is_ok());

        assert!(files::copy_(&client, &random_path, &format!("/Test/{}/{}copy", now, random_filename)).is_ok());

        println!("About to download file");
        let (metadata, resp) = files::download(&client, &random_path).unwrap();
        let body: String = json::decode(&resp.body).unwrap();
        assert_eq!(&body, &random_contents);

        assert!(files::delete(&client, &format!("/Test/{}", now)).is_ok());
    }
}
