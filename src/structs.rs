use std::default::Default;
use rustc_serialize::{Encoder, Encodable, Decoder, Decodable};

#[derive(Debug, PartialEq, Clone)]
pub enum Tag {
    File,
    Folder,
}

impl Encodable for Tag {
    fn encode<S: Encoder>(&self, encoder: &mut S) -> Result<(), S::Error> {
        encoder.emit_enum("Tag", |encoder| {
            match *self {
                Tag::File => encoder.emit_enum_variant("File", 0, 4, |encoder| "file".encode(encoder)),
                Tag::Folder => encoder.emit_enum_variant("Folder", 0, 6, |encoder| "folder".encode(encoder)),
            }
        })
    }
}

impl Decodable for Tag {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Tag, D::Error> {
        decoder.read_enum("Tag", |decoder| {
                decoder.read_enum_variant(&["file", "folder"], |decoder, num| {
                    Ok(match num {
                        0 => Tag::File,
                        1 => Tag::Folder,
                        _ => unreachable!(),
                    })
                })
        })
    }
}

#[derive(Debug, PartialEq, Clone, RustcEncodable)]
pub struct Metadata {
    tag: Tag,
    name: String,
    path_lower: String,
    id: String,
    size: Option<usize>,
    rev: Option<String>,
    client_modified: Option<String>,
    server_modified: Option<String>,
    sharing_info: Option<SharingInfo>,
}

impl Decodable for Metadata {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Metadata, D::Error> {
        decoder.read_struct("", 0, |decoder| {
            Ok(Metadata {
                tag: try!(decoder.read_struct_field(".tag", 0, |decoder| Decodable::decode(decoder))),
                name: try!(decoder.read_struct_field("name", 0, |decoder| Decodable::decode(decoder))),
                path_lower: try!(decoder.read_struct_field("path_lower", 0, |decoder| Decodable::decode(decoder))),
                id: try!(decoder.read_struct_field("id", 0, |decoder| Decodable::decode(decoder))),
                client_modified: try!(decoder.read_struct_field("client_modified", 0, |decoder| Decodable::decode(decoder))),
                server_modified: try!(decoder.read_struct_field("server_modified", 0, |decoder| Decodable::decode(decoder))),
                rev: try!(decoder.read_struct_field("rev", 0, |decoder| Decodable::decode(decoder))),
                size: try!(decoder.read_struct_field("size", 0, |decoder| Decodable::decode(decoder))),
                sharing_info: try!(decoder.read_struct_field("sharing_info", 0, |decoder| Decodable::decode(decoder))),
            })
        })
    }
}

impl Default for Metadata {
    fn default() -> Metadata {
        Metadata {
            tag: Tag::File,
            name: "".to_string(),
            path_lower: "".to_string(),
            id: "".to_string(),
            size: Some(0),
            rev: Some("".to_string()),
            client_modified: Some("".to_string()),
            server_modified: Some("".to_string()),
            sharing_info: Some(SharingInfo {
                read_only: false,
                parent_shared_folder_id: "".to_string()
            }),
        }
    }
}

#[derive(PartialEq, Clone, Debug, RustcEncodable, RustcDecodable)]
pub struct FileMetadata {
    name: String,
    path_lower: String,
    id: String,
    client_modified: String,
    server_modified: String,
    rev: String,
    size: usize,
    sharing_info: Option<SharingInfo>,
    media_info: Option<()>,
}

impl Default for FileMetadata {
    fn default() -> FileMetadata {
        FileMetadata {
            name: "".to_string(),
            path_lower: "".to_string(),
            client_modified: "".to_string(),
            server_modified: "".to_string(),
            rev: "".to_string(),
            size: 0,
            id: "".to_string(),
            sharing_info: None,
            media_info: None,
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

#[derive(PartialEq, Debug, Clone, RustcEncodable, RustcDecodable)]
pub struct NewFolder {
    name: String,
    path_lower: String,
    id: String,
}

#[derive(RustcEncodable, RustcDecodable, Debug, PartialEq, Clone)]
pub struct SharingInfo {
    read_only: bool,
    parent_shared_folder_id: String,
}
