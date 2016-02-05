use std::default::Default;

#[derive(Debug, PartialEq, Clone, RustcEncodable, RustcDecodable)]
pub enum Tag {
    File,
    Folder,
}


#[derive(Debug, PartialEq, Clone, RustcEncodable, RustcDecodable)]
pub struct Metadata {
    tag: Tag,
    name: String,
    path_lower: String,
    id: String,
    client_modified: String,
    server_modified: String,
    rev: String,
    size: usize,
    sharing_info: Option<SharingInfo>,
}

impl Default for Metadata {
    fn default() -> Metadata {
        Metadata {
            tag: Tag::File,
            name: "".to_string(),
            path_lower: "".to_string(),
            client_modified: "".to_string(),
            server_modified: "".to_string(),
            rev: "".to_string(),
            size: 0,
            id: "".to_string(),
            sharing_info: Some(SharingInfo {
                read_only: false,
                parent_shared_folder_id: "".to_string()
            }),
        }
    }
}

/// Struct that is returned from the `list_folder` API call
#[derive(RustcEncodable, RustcDecodable, Debug, PartialEq, Clone)]
pub struct FolderList {
    entries: Vec<Metadata>,
    cursor: String,
    has_more: bool,
}

impl Default for FolderList {
    fn default() -> FolderList {
        FolderList {
            entries: vec![],
            cursor: "".to_string(),
            has_more: false,
        }
    }
}

#[derive(RustcEncodable, RustcDecodable, Debug, PartialEq, Clone)]
pub struct SharingInfo {
    read_only: bool,
    parent_shared_folder_id: String,
}

