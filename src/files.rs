use super::{Result, Response, DropboxClient, ApiError};
use std::default::Default;
use std::io;
use std::fmt;
use std::collections::BTreeMap;
use serde_json;
use serde_json::Value;

use structs::{FolderList, Metadata};

/// Instructs dropbox what to do when a conflict happens during upload
#[derive(Debug, PartialEq, Clone)]
pub enum WriteMode {
    Add,
    Overwrite,
    Update,
}

impl fmt::Display for WriteMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WriteMode::Add => write!(f, "add"),
            WriteMode::Overwrite => write!(f, "overwrite"),
            WriteMode::Update => write!(f, "update"),
        }
    }
}

/// Optional arguments to the `upload` API call
#[derive(Debug, PartialEq, Clone)]
pub struct UploadOptions {
    pub mode: WriteMode,
    pub autorename: bool,
    pub client_modified: Option<String>,
    pub mute: bool,
}

impl Default for UploadOptions {
    fn default() -> UploadOptions {
        UploadOptions {
            mode: WriteMode::Add,
            autorename: false,
            client_modified: None,
            mute: false,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ThumbnailFormat {
    Jpeg,
    Png,
    Gif,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ThumbnailSize {
    W16H16,
    W32H32,
    W64H64,
    Other(usize, usize),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ThumbnailOptions {
    format: ThumbnailFormat,
    size: ThumbnailSize,
}

impl Default for ThumbnailOptions {
    fn default() -> ThumbnailOptions {
        ThumbnailOptions {
            format: ThumbnailFormat::Jpeg,
            size: ThumbnailSize::W64H64,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct GetCursorOptions {
    recursive: bool,
    include_media_info: bool,
    include_deleted: bool,
}

impl Default for GetCursorOptions {
    fn default() -> GetCursorOptions {
        GetCursorOptions {
            recursive: false,
            include_media_info: false,
            include_deleted: false,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct FolderListLongpoll {
    changes: bool,
    backoff: Option<bool>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ListRevisions {
    is_deleted: bool,
    entries: Vec<Metadata>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum SearchMode {
    Filename,
    FilenameAndContent,
    DeletedFilename,
}

#[derive(Debug, PartialEq, Clone)]
pub struct SearchOptions {
    start: usize,
    max_results: usize,
    mode: SearchMode,
}

impl Default for SearchOptions {
    fn default() -> SearchOptions {
        SearchOptions {
            start: 0,
            max_results: 100,
            mode: SearchMode::Filename,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum SearchMatchType {
    Filename(String),
    Content(String),
    Both(String),
}

#[derive(Debug, PartialEq, Clone)]
pub struct SearchMatch {
    match_type: SearchMatchType,
    metadata: Metadata,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Search {
    matches: Vec<SearchMatch>,
    more: bool,
    start: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CommitInfo {
    path: String,
    mode: WriteMode,
    autorename: bool,
    client_modified: String,
    mute: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct UploadSessionCursor {
    session_id: String,
    offset: usize,
}

// Functions

pub fn copy_<T>(client: &T, from: &str, to: &str) -> Result<Metadata>
                where T: DropboxClient
{
    let mut map = BTreeMap::new();
    map.insert("from_path".to_string(), Value::String(from.to_string()));
    map.insert("to_path".to_string(), Value::String(to.to_string()));
    let mut headers = BTreeMap::new();
    headers.insert("Content-Type".to_string(), "application/json".to_string());
    let resp = client.api("files/copy", &mut headers, &map);
    Ok(Default::default())
}

pub fn create_folder<T>(client: &T, path: &str) -> Result<Metadata>
                where T: DropboxClient
{
    let mut map = BTreeMap::new();
    map.insert("path".to_string(), Value::String(path.to_string()));
    let mut headers = BTreeMap::new();
    headers.insert("Content-Type".to_string(), "application/json".to_string());
    let resp = client.api("files/create_folder", &mut headers, &map);
    Ok(Default::default())
}

pub fn delete<T: DropboxClient>(client: &T, path: &str) -> Result<Metadata> {
    Ok(Default::default())
}

pub fn download<T: DropboxClient>(client: &T, path: &str) -> Result<(Metadata, Response)> {
    Ok((
        Default::default(),
        Response {
            _status: 200,
            _body: "".to_string(),
        },
    ))
}

pub fn download_to_file<T>(client: &T, dest_path: &str, path: &str) -> Result<(Metadata, Response)>
                where T: DropboxClient
{
    Ok((
        Default::default(),
        Response {
            _status: 200,
            _body: "".to_string(),
        },
    ))
}

pub fn get_metadata<T>(client: &T, path: &str, include_media_info: bool) -> Result<Metadata>
                where T: DropboxClient
{
    Ok(Default::default())
}

pub fn get_preview<T>(client: &T, path: &str) -> Result<(Metadata, Response)>
                where T: DropboxClient
{
    Ok((
        Default::default(),
        Response {
            _status: 200,
            _body: "".to_string(),
        },
    ))
}

pub fn get_preview_to_file<T>(client: &T, dest_path: &str, path: &str) -> Result<(Metadata, Response)>
                where T: DropboxClient
{
    Ok((
        Default::default(),
        Response {
            _status: 200,
            _body: "".to_string(),
        },
    ))
}

pub fn get_thumbnail<T>(client: &T, path: &str) -> Result<(Metadata, Response)>
                where T: DropboxClient
{
    get_thumbnail_with_options(client, path, Default::default())
}

pub fn get_thumbnail_with_options<T>(client: &T, path: &str, options: ThumbnailOptions) -> Result<(Metadata, Response)>
                where T: DropboxClient
{
    Ok((
        Default::default(),
        Response {
            _status: 200,
            _body: "".to_string(),
        },
    ))
}

pub fn get_thumbnail_to_file<T>(client: &T, dest_path: &str, path: &str) -> Result<(Metadata, Response)>
                where T: DropboxClient
{
    get_thumbnail_to_file_with_options(client, dest_path, path, Default::default())
}

pub fn get_thumbnail_to_file_with_options<T>(client: &T, dest_path: &str, path: &str, options: ThumbnailOptions) -> Result<(Metadata, Response)>
                where T: DropboxClient
{
    Ok((
        Default::default(),
        Response {
            _status: 200,
            _body: "".to_string(),
        },
    ))
}

pub fn list_folder<T: DropboxClient>(client: &T, path: &str) -> Result<FolderList> {
    let mut map = BTreeMap::new();
    map.insert("path".to_string(), Value::String("/OpenBudget".to_string()));
    map.insert("recursive".to_string(), Value::Bool(false));
    map.insert("include_media_info".to_string(), Value::Bool(false));
    map.insert("include_deleted".to_string(), Value::Bool(false));
    let mut headers = BTreeMap::new();
    headers.insert("Content-Type".to_string(), "application/json".to_string());
    match client.api("files/list_folder", &mut headers, &map) {
        Ok(_) => Ok(Default::default()),
        Err(e) => Err(ApiError::ClientError),
    }
}

pub fn list_folder_continue<T>(client: &T, cursor: &str) -> Result<FolderList>
                where T: DropboxClient
{
    Ok(Default::default())
}

pub fn list_folder_get_latest_cursor<T>(client: &T, path: &str) -> Result<String>
                where T: DropboxClient
{
    list_folder_get_latest_cursor_with_options(client, path, Default::default())
}

pub fn list_folder_get_latest_cursor_with_options<T>(client: &T, path: &str, options: GetCursorOptions) -> Result<String>
                where T: DropboxClient
{
    Ok("".to_string())
}

pub fn list_folder_longpoll<T>(client: &T, cursor: &str, timeout: usize) -> Result<FolderListLongpoll>
                where T: DropboxClient
{
    Ok(FolderListLongpoll {
        changes: false,
        backoff: None,
    })
}

pub fn list_revisions<T>(client: &T, path: &str, limit: usize) -> Result<ListRevisions>
                where T: DropboxClient
{
    Ok(ListRevisions {
        is_deleted: false,
        entries: vec![],
    })
}

pub fn move_<T>(client: &T, from: &str, to: &str) -> Result<Metadata>
                where T: DropboxClient
{
    Ok(Default::default())
}

pub fn permanently_delete<T>(client: &T, path: &str) -> Result<()>
                where T: DropboxClient
{
    Ok(())
}

pub fn restore<T>(client: &T, path: &str, rev: &str) -> Result<Metadata>
                where T: DropboxClient
{
    Ok(Default::default())
}

pub fn search<T>(client: &T, path: &str, query: &str) -> Result<Search>
                where T: DropboxClient
{
    search_with_options(client, path, query, Default::default())
}

pub fn search_with_options<T>(client: &T, path: &str, query: &str, options: SearchOptions) -> Result<Search>
                where T: DropboxClient
{
    Ok(Search {
        matches: vec![],
        more: false,
        start: 0,
    })
}

pub fn upload<T>(client: &T, contents: &str, path: &str) -> Result<Metadata>
                where T: DropboxClient
{
    upload_with_options(client, contents, path, Default::default())
}

pub fn upload_with_options<T>(client: &T, contents: &str, path: &str, options: UploadOptions) -> Result<Metadata>
                where T: DropboxClient
{
    let mut map = BTreeMap::new();
    map.insert("path", Value::String(path.to_string()));
    map.insert("mode", Value::String(format!("{}", options.mode)));
    map.insert("autorename", Value::Bool(options.autorename));
    map.insert("mute", Value::Bool(options.mute));
    let mut headers = BTreeMap::new();
    headers.insert("Dropbox-API-Arg".to_string(), serde_json::to_string(&map).unwrap());
    headers.insert("Content-Type".to_string(), "application/octet-stream".to_string());
    let resp = client.content("files/upload", &mut headers, &contents);
    Ok(Default::default())
}

pub fn upload_session_append<T, U>(client: &T, f: U, session_id: &str, offset: usize) -> Result<()>
                where T: DropboxClient, U: io::Read
{
    Ok(())
}

pub fn upload_session_finish<T, U>(client: &T, f: U, cursor: &UploadSessionCursor, commit: &CommitInfo) -> Result<Metadata>
                where T: DropboxClient,
                      U: io::Read
{
    Ok(Default::default())
}

pub fn upload_session_start<T, U>(client: &T, f: U) -> Result<String>
                where T: DropboxClient,
                      U: io::Read
{
    Ok("".to_string())
}
