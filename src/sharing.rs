use super::{Result, ApiError, Response, DropboxClient};
use std::default::Default;

use structs::{FolderList, Metadata, SharingInfo, Tag};

#[derive(Debug, PartialEq, Clone)]
pub enum JobError {
    AccessError(String),
    MemberError(String),
    Other(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum SharePathError {
    AlreadyShared,
    ContainsSharedFolder,
    InsideAppFolder,
    InsideSharedFolder,
    InvalidPath,
    IsAppFolder,
    IsFile,
    Other(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum ShareFolderError {
    EmailUnverified,
    BadPath(SharePathError),
    TeamPolicyDisallowsMemberPolicy,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AddFolderMemberOptions {
    quiet: bool,
    custom_message: Option<String>,
}

impl Default for AddFolderMemberOptions {
    fn default() -> AddFolderMemberOptions {
        AddFolderMemberOptions {
            quiet: false,
            custom_message: None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum JobStatus {
    Complete,
    Failed(JobError),
}

#[derive(Debug, PartialEq, Clone)]
pub enum ShareFolderJobStatus {
    Complete(Metadata),
    Failed(ShareFolderError),
}

#[derive(Debug, PartialEq, Clone)]
pub struct CreateSharedLinkOptions {
    short_url: bool,
    pending_upload: Option<PendingUploadMode>,
}

impl Default for CreateSharedLinkOptions {
    fn default() -> CreateSharedLinkOptions {
        CreateSharedLinkOptions {
            short_url: false,
            pending_upload: None
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum PendingUploadMode {
    File,
    Folder,
}

#[derive(Debug, PartialEq, Clone)]
pub struct SharedFolderMembers {
    users: Vec<UserMembershipInfo>,
    groups: Vec<GroupMembershipInfo>,
    invitees: Vec<InviteeMembershipInfo>,
    cursor: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct UserMembershipInfo {
    access_type: AccessLevel,
    user: UserInfo,
}

#[derive(Debug, PartialEq, Clone)]
pub struct GroupMembershipInfo {
    access_type: AccessLevel,
    group: GroupInfo,
}

#[derive(Debug, PartialEq, Clone)]
pub struct InviteeMembershipInfo {
    access_type: AccessLevel,
    invitee: InviteeInfo,
}

#[derive(Debug, PartialEq, Clone)]
pub enum AccessLevel {
    Owner,
    Editor,
    Viewer,
    Other(String),
}

#[derive(Debug, PartialEq, Clone)]
pub struct UserInfo {
    account_id: String,
    same_team: bool,
    team_member_id: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct GroupInfo {
    group_name: String,
    group_id: String,
    member_count: usize,
    same_team: bool,
    group_external_id: String,
}

#[derive(Debug, PartialEq, Clone)]
pub enum InviteeInfo {
    Email(String),
    Other(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum MemberSelector {
    DropboxID(String),
    Email(String),
    Other(String),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ShareFolderOptions {
    member_policy: MemberPolicy,
    acl_update_policy: AclUpdatePolicy,
    shared_link_policy: SharedLinkPolicy,
    force_async: bool,
}

impl Default for ShareFolderOptions {
    fn default() -> ShareFolderOptions {
        ShareFolderOptions {
            member_policy: MemberPolicy::Anyone,
            acl_update_policy: AclUpdatePolicy::Owner,
            shared_link_policy: SharedLinkPolicy::Anyone,
            force_async: false,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum MemberPolicy {
    Team,
    Anyone,
    Other(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum AclUpdatePolicy {
    Owner,
    Editors,
    Other(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum SharedLinkPolicy {
    Anyone,
    Members,
    Other(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum ShareFolderLaunch {
    Complete(Metadata),
}

#[derive(Debug, PartialEq, Clone)]
pub struct UpdateFolderPolicyOptions {
    member_policy: Option<MemberPolicy>,
    acl_update_policy: Option<AclUpdatePolicy>,
    shared_link_policy: Option<SharedLinkPolicy>,
}

impl Default for UpdateFolderPolicyOptions {
    fn default() -> UpdateFolderPolicyOptions {
        UpdateFolderPolicyOptions {
            member_policy: None,
            acl_update_policy: None,
            shared_link_policy: None,
        }
    }
}

// Functions
pub fn add_folder_member<T, U>(client: &T, shared_folder_id: &str, members: &[U]) -> Result<()>
                where T: DropboxClient,
                      U: AsRef<str>
{
    add_folder_member_with_options(client, shared_folder_id, members, Default::default())
}

pub fn add_folder_member_with_options<T, U>(client: &T, shared_folder_id: &str, members: &[U], options: AddFolderMemberOptions) -> Result<()>
                where T: DropboxClient,
                      U: AsRef<str>
{
    Ok(())
}

pub fn check_job_status<T>(client: &T, async_job_id: &str) -> Result<JobStatus>
                where T: DropboxClient
{
    Ok(JobStatus::Complete)
}

pub fn check_share_job_status<T>(client: &T, async_job_id: &str) -> Result<ShareFolderJobStatus>
                where T: DropboxClient
{
    Ok(ShareFolderJobStatus::Complete(Default::default()))
}

pub fn create_shared_link<T>(client: &T, path: &str) -> Result<Metadata>
                where T: DropboxClient
{
    create_shared_link_with_options(client, path, Default::default())
}

pub fn create_shared_link_with_options<T>(client: &T, path: &str, options: CreateSharedLinkOptions) -> Result<Metadata>
                where T: DropboxClient
{
    Ok(Default::default())
}

pub fn get_folder_metadata<T>(client: &T, shared_folder_id: &str) -> Result<Metadata>
                where T: DropboxClient
{
    Ok(Default::default())
}

pub fn get_shared_links<T>(client: &T, path: Option<&str>) -> Result<Vec<Metadata>>
                where T: DropboxClient
{
    Ok(vec![])
}

pub fn list_folder_members<T>(client: &T, shared_folder_id: &str) -> Result<SharedFolderMembers>
                where T: DropboxClient
{
    Ok(SharedFolderMembers {
        users: vec![],
        groups: vec![],
        invitees: vec![],
        cursor: "".to_string(),
    })
}

pub fn list_folder_members_continue<T>(client: &T, cursor: &str) -> Result<SharedFolderMembers>
                where T: DropboxClient
{
    Ok(SharedFolderMembers {
        users: vec![],
        groups: vec![],
        invitees: vec![],
        cursor: "".to_string(),
    })
}

pub fn list_folders<T>(client: &T) -> Result<FolderList>
                where T: DropboxClient
{
    Ok(Default::default())
}

pub fn list_folders_continue<T>(client: &T, cursor: &str) -> Result<FolderList>
                where T: DropboxClient
{
    Ok(Default::default())
}

pub fn mount_folder<T>(client: &T, shared_folder_id: &str) -> Result<Metadata>
                where T: DropboxClient
{
    Ok(Default::default())
}

pub fn relinquish_folder_membership<T>(client: &T, shared_folder_id: &str) -> Result<()>
                where T: DropboxClient
{
    Ok(())
}

pub fn remove_folder_member<T>(client: &T,
                               shared_folder_id: &str,
                               member: MemberSelector,
                               leave_a_copy: bool) -> Result<()>
                where T: DropboxClient
{
    Ok(())
}

pub fn revoke_shared_link<T>(client: &T, url: &str) -> Result<()>
                where T: DropboxClient
{
    Ok(())
}

pub fn share_folder<T>(client: &T, path: &str) -> Result<ShareFolderLaunch>
                where T: DropboxClient
{
    share_folder_with_options(client, path, Default::default())
}

pub fn share_folder_with_options<T>(client: &T, path: &str, options: ShareFolderOptions) -> Result<ShareFolderLaunch>
                where T: DropboxClient
{
    Ok(ShareFolderLaunch::Complete(Default::default()))
}

pub fn transfer_folder<T>(client: &T, shared_folder_id: &str, to_dropbox_id: &str) -> Result<()>
                where T: DropboxClient
{
    Ok(())
}

pub fn unmount_folder<T>(client: &T, shared_folder_id: &str) -> Result<()>
                where T: DropboxClient
{
    Ok(())
}

pub fn unshare_folder<T>(client: &T, shared_folder_id: &str, leave_a_copy: bool) -> Result<()>
                where T: DropboxClient
{
    Ok(())
}

pub fn update_folder_member<T>(client: &T,
                               shared_folder_id: &str,
                               member: MemberSelector,
                               access_level: AccessLevel) -> Result<()>
                where T: DropboxClient
{
    Ok(())
}

pub fn update_folder_policy<T>(client: &T, shared_folder_id: &str) -> Result<Metadata>
                where T: DropboxClient
{
    update_folder_policy_with_options(client, shared_folder_id, Default::default())
}

pub fn update_folder_policy_with_options<T>(client: &T, shared_folder_id: &str, options: UpdateFolderPolicyOptions) -> Result<Metadata>
                where T: DropboxClient
{
    Ok(Default::default())
}
