use super::{Result, ApiError, Response, DropboxClient};
use std::default::Default;

use structs::{FolderList, Metadata, SharingInfo, Tag};

#[derive(Debug, PartialEq, Clone)]
pub struct BasicAccount {
    account_id: Option<String>,
    name: Option<String>,
    is_teammate: Option<bool>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FullAccount {
account_id: String,
    name: String,
    email: String,
    locale: String,
    referral_link: String,
    is_paired: bool,
    account_type: String,
    country: String,
    team: Option<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct SpaceUsage {
    used: usize,
    allocation: usize,
}

// Functions

pub fn get_account<T>(client: &T, account_id: &str) -> Result<BasicAccount>
                where T: DropboxClient
{
    Ok(BasicAccount {
        account_id: None,
        name: None,
        is_teammate: None,
    })
}

pub fn get_account_batch<T, U>(client: &T, account_ids: &[U]) -> Result<Vec<BasicAccount>>
                where T: DropboxClient,
                      U: AsRef<str>
{
    if account_ids.len() > 300 {
        return Err(ApiError::GetAccountBatchError);
    }
    Ok(vec![])
}

pub fn get_current_account<T>(client: &T) -> Result<FullAccount>
                where T: DropboxClient
{
    Ok(FullAccount {
        account_id: "".to_string(),
        name: "".to_string(),
        email: "".to_string(),
        locale: "".to_string(),
        referral_link: "".to_string(),
        is_paired: false,
        account_type: "".to_string(),
        country: "".to_string(),
        team: None,
    })
}

pub fn get_space_usage<T>(client: &T) -> Result<SpaceUsage>
                where T: DropboxClient
{
    Ok(SpaceUsage {
        used: 0,
        allocation: 0,
    })
}
