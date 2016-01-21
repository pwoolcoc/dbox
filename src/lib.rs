#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_must_use)]

//! Dropbox SDK for Rust
//!
//! # Example
//!
//! ```ignore
//! extern crate dropbox;
//!
//! use dropbox::client::Client;
//! use dropbox::files;
//!
//! let client = Client::new(ACCESS_TOKEN);
//! let folder_list = files::list_folder(&client, "/path/to/folder");
//! ```
//!

#[cfg(feature = "hyper-client")] extern crate hyper;
extern crate serde;
extern crate serde_json;
extern crate chrono;

#[cfg(test)] extern crate rand;

use std::convert::From;
use std::fmt;
use std::collections::BTreeMap;
use serde::ser;

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
    fn request<T>(&self, endpoint: Endpoint, url: &str, headers: &mut BTreeMap<String, String>, body: &T) -> Result<Response> where T: ser::Serialize;

    fn api<T>(&self, url: &str, headers: &mut BTreeMap<String, String>, body: &T) -> Result<Response>
            where T: ser::Serialize
    {
        self.request(Endpoint::Api, url, headers, body)
    }

    fn content<T>(&self, url: &str, headers: &mut BTreeMap<String, String>, body: &T) -> Result<Response>
            where T: ser::Serialize
    {
        self.request(Endpoint::Content, url, headers, body)
    }
}

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

impl From<serde_json::error::Error> for ApiError {
    fn from(err: serde_json::error::Error) -> ApiError {
        ApiError::ClientError
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Response {
    _status: u16,
    _body: String,
}

impl Response {
    pub fn status(&self) -> u16 {
        self._status
    }
    pub fn body(&self) -> String {
        self._body.clone()
    }
}

pub type Result<T> = ::std::result::Result<T, ApiError>;

#[cfg(feature = "hyper-client")] pub mod client;
pub mod files;
pub mod structs;
pub mod sharing;
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
        let access_token = env::var(ACCESS_TOKEN).unwrap();
        let client = Client::new(&access_token).unwrap();
        files::create_folder(&client, "");
        assert!(files::list_folder(&client, "").is_ok());
    }

    #[test]
    fn test_upload_download() {
        let access_token = env::var(ACCESS_TOKEN).unwrap();
        let random_filename = random_ascii_letters(15);
        let now: DateTime<Local> = Local::now();
        let now = format!("{}", now.timestamp());
        let random_path = format!("/Test/{}/{}", now, random_filename);
        let random_contents = random_ascii_letters(20);

        let client = Client::new(&access_token).unwrap();
        assert!(files::upload(&client, &random_contents, &random_path).is_ok());

        assert!(files::copy_(&client, &random_path, &format!("/Test/{}/{}copy", now, random_filename)).is_ok());

        let (metadata, resp) = files::download(&client, &random_path).unwrap();
        assert_eq!(&resp.body(), &random_contents);

        assert!(files::delete(&client, &format!("/Test/{}", now)).is_ok());
    }
}
