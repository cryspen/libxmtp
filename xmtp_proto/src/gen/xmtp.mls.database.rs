// @generated
/// The data required to publish a message
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendMessageData {
    #[prost(oneof="send_message_data::Version", tags="1")]
    pub version: ::core::option::Option<send_message_data::Version>,
}
/// Nested message and enum types in `SendMessageData`.
pub mod send_message_data {
    /// V1 of SendMessagePublishData
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct V1 {
        #[prost(bytes="vec", tag="1")]
        pub payload_bytes: ::prost::alloc::vec::Vec<u8>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Version {
        #[prost(message, tag="1")]
        V1(V1),
    }
}
/// Wrapper around a list af repeated EVM Account Addresses
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountAddresses {
    #[prost(string, repeated, tag="1")]
    pub account_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Wrapper around a list of repeated Installation IDs
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstallationIds {
    #[prost(bytes="vec", repeated, tag="1")]
    pub installation_ids: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// One of an EVM account address or Installation ID
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressesOrInstallationIds {
    #[prost(oneof="addresses_or_installation_ids::AddressesOrInstallationIds", tags="1, 2")]
    pub addresses_or_installation_ids: ::core::option::Option<addresses_or_installation_ids::AddressesOrInstallationIds>,
}
/// Nested message and enum types in `AddressesOrInstallationIds`.
pub mod addresses_or_installation_ids {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum AddressesOrInstallationIds {
        #[prost(message, tag="1")]
        AccountAddresses(super::AccountAddresses),
        #[prost(message, tag="2")]
        InstallationIds(super::InstallationIds),
    }
}
/// The data required to add members to a group
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddMembersData {
    #[prost(oneof="add_members_data::Version", tags="1")]
    pub version: ::core::option::Option<add_members_data::Version>,
}
/// Nested message and enum types in `AddMembersData`.
pub mod add_members_data {
    /// V1 of AddMembersPublishData
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct V1 {
        #[prost(message, optional, tag="1")]
        pub addresses_or_installation_ids: ::core::option::Option<super::AddressesOrInstallationIds>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Version {
        #[prost(message, tag="1")]
        V1(V1),
    }
}
/// The data required to remove members from a group
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveMembersData {
    #[prost(oneof="remove_members_data::Version", tags="1")]
    pub version: ::core::option::Option<remove_members_data::Version>,
}
/// Nested message and enum types in `RemoveMembersData`.
pub mod remove_members_data {
    /// V1 of RemoveMembersPublishData
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct V1 {
        #[prost(message, optional, tag="1")]
        pub addresses_or_installation_ids: ::core::option::Option<super::AddressesOrInstallationIds>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Version {
        #[prost(message, tag="1")]
        V1(V1),
    }
}
/// The data required to make a commit that updates group membership
/// Handles both Add and Remove actions
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateGroupMembershipData {
    #[prost(oneof="update_group_membership_data::Version", tags="1")]
    pub version: ::core::option::Option<update_group_membership_data::Version>,
}
/// Nested message and enum types in `UpdateGroupMembershipData`.
pub mod update_group_membership_data {
    /// V1 of UpdateGroupMembershipPublishData
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct V1 {
        /// Contains delta of membership updates that need to be applied
        #[prost(map="string, uint64", tag="1")]
        pub membership_updates: ::std::collections::HashMap<::prost::alloc::string::String, u64>,
        /// Contains the list of members that will be removed
        #[prost(string, repeated, tag="2")]
        pub removed_members: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Version {
        #[prost(message, tag="1")]
        V1(V1),
    }
}
/// The data required to update group metadata
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateMetadataData {
    #[prost(oneof="update_metadata_data::Version", tags="1")]
    pub version: ::core::option::Option<update_metadata_data::Version>,
}
/// Nested message and enum types in `UpdateMetadataData`.
pub mod update_metadata_data {
    /// V1 of UpdateMetadataPublishData
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct V1 {
        #[prost(string, tag="1")]
        pub field_name: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub field_value: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Version {
        #[prost(message, tag="1")]
        V1(V1),
    }
}
/// Generic data-type for all post-commit actions
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PostCommitAction {
    #[prost(oneof="post_commit_action::Kind", tags="1")]
    pub kind: ::core::option::Option<post_commit_action::Kind>,
}
/// Nested message and enum types in `PostCommitAction`.
pub mod post_commit_action {
    /// An installation
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Installation {
        #[prost(bytes="vec", tag="1")]
        pub installation_key: ::prost::alloc::vec::Vec<u8>,
        #[prost(bytes="vec", tag="2")]
        pub hpke_public_key: ::prost::alloc::vec::Vec<u8>,
    }
    /// SendWelcome message
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SendWelcomes {
        #[prost(message, repeated, tag="1")]
        pub installations: ::prost::alloc::vec::Vec<Installation>,
        #[prost(bytes="vec", tag="2")]
        pub welcome_message: ::prost::alloc::vec::Vec<u8>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        #[prost(message, tag="1")]
        SendWelcomes(SendWelcomes),
    }
}
/// Encoded file descriptor set for the `xmtp.mls.database` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xf1, 0x25, 0x0a, 0x1a, 0x6d, 0x6c, 0x73, 0x2f, 0x64, 0x61, 0x74, 0x61, 0x62, 0x61, 0x73,
    0x65, 0x2f, 0x69, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12,
    0x11, 0x78, 0x6d, 0x74, 0x70, 0x2e, 0x6d, 0x6c, 0x73, 0x2e, 0x64, 0x61, 0x74, 0x61, 0x62, 0x61,
    0x73, 0x65, 0x22, 0x80, 0x01, 0x0a, 0x0f, 0x53, 0x65, 0x6e, 0x64, 0x4d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x44, 0x61, 0x74, 0x61, 0x12, 0x37, 0x0a, 0x02, 0x76, 0x31, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x25, 0x2e, 0x78, 0x6d, 0x74, 0x70, 0x2e, 0x6d, 0x6c, 0x73, 0x2e, 0x64, 0x61,
    0x74, 0x61, 0x62, 0x61, 0x73, 0x65, 0x2e, 0x53, 0x65, 0x6e, 0x64, 0x4d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x44, 0x61, 0x74, 0x61, 0x2e, 0x56, 0x31, 0x48, 0x00, 0x52, 0x02, 0x76, 0x31, 0x1a,
    0x29, 0x0a, 0x02, 0x56, 0x31, 0x12, 0x23, 0x0a, 0x0d, 0x70, 0x61, 0x79, 0x6c, 0x6f, 0x61, 0x64,
    0x5f, 0x62, 0x79, 0x74, 0x65, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x0c, 0x70, 0x61,
    0x79, 0x6c, 0x6f, 0x61, 0x64, 0x42, 0x79, 0x74, 0x65, 0x73, 0x42, 0x09, 0x0a, 0x07, 0x76, 0x65,
    0x72, 0x73, 0x69, 0x6f, 0x6e, 0x22, 0x3f, 0x0a, 0x10, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74,
    0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x65, 0x73, 0x12, 0x2b, 0x0a, 0x11, 0x61, 0x63, 0x63,
    0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x65, 0x73, 0x18, 0x01,
    0x20, 0x03, 0x28, 0x09, 0x52, 0x10, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x41, 0x64, 0x64,
    0x72, 0x65, 0x73, 0x73, 0x65, 0x73, 0x22, 0x3c, 0x0a, 0x0f, 0x49, 0x6e, 0x73, 0x74, 0x61, 0x6c,
    0x6c, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x49, 0x64, 0x73, 0x12, 0x29, 0x0a, 0x10, 0x69, 0x6e, 0x73,
    0x74, 0x61, 0x6c, 0x6c, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x73, 0x18, 0x01, 0x20,
    0x03, 0x28, 0x0c, 0x52, 0x0f, 0x69, 0x6e, 0x73, 0x74, 0x61, 0x6c, 0x6c, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x49, 0x64, 0x73, 0x22, 0xe2, 0x01, 0x0a, 0x1a, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73,
    0x65, 0x73, 0x4f, 0x72, 0x49, 0x6e, 0x73, 0x74, 0x61, 0x6c, 0x6c, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x49, 0x64, 0x73, 0x12, 0x52, 0x0a, 0x11, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x5f, 0x61,
    0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x65, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x23,
    0x2e, 0x78, 0x6d, 0x74, 0x70, 0x2e, 0x6d, 0x6c, 0x73, 0x2e, 0x64, 0x61, 0x74, 0x61, 0x62, 0x61,
    0x73, 0x65, 0x2e, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73,
    0x73, 0x65, 0x73, 0x48, 0x00, 0x52, 0x10, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x41, 0x64,
    0x64, 0x72, 0x65, 0x73, 0x73, 0x65, 0x73, 0x12, 0x4f, 0x0a, 0x10, 0x69, 0x6e, 0x73, 0x74, 0x61,
    0x6c, 0x6c, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x22, 0x2e, 0x78, 0x6d, 0x74, 0x70, 0x2e, 0x6d, 0x6c, 0x73, 0x2e, 0x64, 0x61, 0x74,
    0x61, 0x62, 0x61, 0x73, 0x65, 0x2e, 0x49, 0x6e, 0x73, 0x74, 0x61, 0x6c, 0x6c, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x49, 0x64, 0x73, 0x48, 0x00, 0x52, 0x0f, 0x69, 0x6e, 0x73, 0x74, 0x61, 0x6c, 0x6c,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x49, 0x64, 0x73, 0x42, 0x1f, 0x0a, 0x1d, 0x61, 0x64, 0x64, 0x72,
    0x65, 0x73, 0x73, 0x65, 0x73, 0x5f, 0x6f, 0x72, 0x5f, 0x69, 0x6e, 0x73, 0x74, 0x61, 0x6c, 0x6c,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x73, 0x22, 0xcb, 0x01, 0x0a, 0x0e, 0x41, 0x64,
    0x64, 0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x73, 0x44, 0x61, 0x74, 0x61, 0x12, 0x36, 0x0a, 0x02,
    0x76, 0x31, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x24, 0x2e, 0x78, 0x6d, 0x74, 0x70, 0x2e,
    0x6d, 0x6c, 0x73, 0x2e, 0x64, 0x61, 0x74, 0x61, 0x62, 0x61, 0x73, 0x65, 0x2e, 0x41, 0x64, 0x64,
    0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x73, 0x44, 0x61, 0x74, 0x61, 0x2e, 0x56, 0x31, 0x48, 0x00,
    0x52, 0x02, 0x76, 0x31, 0x1a, 0x76, 0x0a, 0x02, 0x56, 0x31, 0x12, 0x70, 0x0a, 0x1d, 0x61, 0x64,
    0x64, 0x72, 0x65, 0x73, 0x73, 0x65, 0x73, 0x5f, 0x6f, 0x72, 0x5f, 0x69, 0x6e, 0x73, 0x74, 0x61,
    0x6c, 0x6c, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x2d, 0x2e, 0x78, 0x6d, 0x74, 0x70, 0x2e, 0x6d, 0x6c, 0x73, 0x2e, 0x64, 0x61, 0x74,
    0x61, 0x62, 0x61, 0x73, 0x65, 0x2e, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x65, 0x73, 0x4f,
    0x72, 0x49, 0x6e, 0x73, 0x74, 0x61, 0x6c, 0x6c, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x49, 0x64, 0x73,
    0x52, 0x1a, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x65, 0x73, 0x4f, 0x72, 0x49, 0x6e, 0x73,
    0x74, 0x61, 0x6c, 0x6c, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x49, 0x64, 0x73, 0x42, 0x09, 0x0a, 0x07,
    0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x22, 0xd1, 0x01, 0x0a, 0x11, 0x52, 0x65, 0x6d, 0x6f,
    0x76, 0x65, 0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x73, 0x44, 0x61, 0x74, 0x61, 0x12, 0x39, 0x0a,
    0x02, 0x76, 0x31, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x27, 0x2e, 0x78, 0x6d, 0x74, 0x70,
    0x2e, 0x6d, 0x6c, 0x73, 0x2e, 0x64, 0x61, 0x74, 0x61, 0x62, 0x61, 0x73, 0x65, 0x2e, 0x52, 0x65,
    0x6d, 0x6f, 0x76, 0x65, 0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x73, 0x44, 0x61, 0x74, 0x61, 0x2e,
    0x56, 0x31, 0x48, 0x00, 0x52, 0x02, 0x76, 0x31, 0x1a, 0x76, 0x0a, 0x02, 0x56, 0x31, 0x12, 0x70,
    0x0a, 0x1d, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x65, 0x73, 0x5f, 0x6f, 0x72, 0x5f, 0x69,
    0x6e, 0x73, 0x74, 0x61, 0x6c, 0x6c, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x73, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x2d, 0x2e, 0x78, 0x6d, 0x74, 0x70, 0x2e, 0x6d, 0x6c, 0x73,
    0x2e, 0x64, 0x61, 0x74, 0x61, 0x62, 0x61, 0x73, 0x65, 0x2e, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73,
    0x73, 0x65, 0x73, 0x4f, 0x72, 0x49, 0x6e, 0x73, 0x74, 0x61, 0x6c, 0x6c, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x49, 0x64, 0x73, 0x52, 0x1a, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x65, 0x73, 0x4f,
    0x72, 0x49, 0x6e, 0x73, 0x74, 0x61, 0x6c, 0x6c, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x49, 0x64, 0x73,
    0x42, 0x09, 0x0a, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x22, 0xd6, 0x02, 0x0a, 0x19,
    0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x47, 0x72, 0x6f, 0x75, 0x70, 0x4d, 0x65, 0x6d, 0x62, 0x65,
    0x72, 0x73, 0x68, 0x69, 0x70, 0x44, 0x61, 0x74, 0x61, 0x12, 0x41, 0x0a, 0x02, 0x76, 0x31, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x2f, 0x2e, 0x78, 0x6d, 0x74, 0x70, 0x2e, 0x6d, 0x6c, 0x73,
    0x2e, 0x64, 0x61, 0x74, 0x61, 0x62, 0x61, 0x73, 0x65, 0x2e, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65,
    0x47, 0x72, 0x6f, 0x75, 0x70, 0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x73, 0x68, 0x69, 0x70, 0x44,
    0x61, 0x74, 0x61, 0x2e, 0x56, 0x31, 0x48, 0x00, 0x52, 0x02, 0x76, 0x31, 0x1a, 0xea, 0x01, 0x0a,
    0x02, 0x56, 0x31, 0x12, 0x75, 0x0a, 0x12, 0x6d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x73, 0x68, 0x69,
    0x70, 0x5f, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32,
    0x46, 0x2e, 0x78, 0x6d, 0x74, 0x70, 0x2e, 0x6d, 0x6c, 0x73, 0x2e, 0x64, 0x61, 0x74, 0x61, 0x62,
    0x61, 0x73, 0x65, 0x2e, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x47, 0x72, 0x6f, 0x75, 0x70, 0x4d,
    0x65, 0x6d, 0x62, 0x65, 0x72, 0x73, 0x68, 0x69, 0x70, 0x44, 0x61, 0x74, 0x61, 0x2e, 0x56, 0x31,
    0x2e, 0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x73, 0x68, 0x69, 0x70, 0x55, 0x70, 0x64, 0x61, 0x74,
    0x65, 0x73, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x52, 0x11, 0x6d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x73,
    0x68, 0x69, 0x70, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x73, 0x12, 0x27, 0x0a, 0x0f, 0x72, 0x65,
    0x6d, 0x6f, 0x76, 0x65, 0x64, 0x5f, 0x6d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x73, 0x18, 0x02, 0x20,
    0x03, 0x28, 0x09, 0x52, 0x0e, 0x72, 0x65, 0x6d, 0x6f, 0x76, 0x65, 0x64, 0x4d, 0x65, 0x6d, 0x62,
    0x65, 0x72, 0x73, 0x1a, 0x44, 0x0a, 0x16, 0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x73, 0x68, 0x69,
    0x70, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x73, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x12, 0x10, 0x0a,
    0x03, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x03, 0x6b, 0x65, 0x79, 0x12,
    0x14, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x05,
    0x76, 0x61, 0x6c, 0x75, 0x65, 0x3a, 0x02, 0x38, 0x01, 0x42, 0x09, 0x0a, 0x07, 0x76, 0x65, 0x72,
    0x73, 0x69, 0x6f, 0x6e, 0x22, 0xa1, 0x01, 0x0a, 0x12, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x4d,
    0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x44, 0x61, 0x74, 0x61, 0x12, 0x3a, 0x0a, 0x02, 0x76,
    0x31, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x28, 0x2e, 0x78, 0x6d, 0x74, 0x70, 0x2e, 0x6d,
    0x6c, 0x73, 0x2e, 0x64, 0x61, 0x74, 0x61, 0x62, 0x61, 0x73, 0x65, 0x2e, 0x55, 0x70, 0x64, 0x61,
    0x74, 0x65, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x44, 0x61, 0x74, 0x61, 0x2e, 0x56,
    0x31, 0x48, 0x00, 0x52, 0x02, 0x76, 0x31, 0x1a, 0x44, 0x0a, 0x02, 0x56, 0x31, 0x12, 0x1d, 0x0a,
    0x0a, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x09, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x1f, 0x0a, 0x0b,
    0x66, 0x69, 0x65, 0x6c, 0x64, 0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x0a, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x42, 0x09, 0x0a,
    0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x22, 0xe8, 0x02, 0x0a, 0x10, 0x50, 0x6f, 0x73,
    0x74, 0x43, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x41, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x57, 0x0a,
    0x0d, 0x73, 0x65, 0x6e, 0x64, 0x5f, 0x77, 0x65, 0x6c, 0x63, 0x6f, 0x6d, 0x65, 0x73, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x30, 0x2e, 0x78, 0x6d, 0x74, 0x70, 0x2e, 0x6d, 0x6c, 0x73, 0x2e,
    0x64, 0x61, 0x74, 0x61, 0x62, 0x61, 0x73, 0x65, 0x2e, 0x50, 0x6f, 0x73, 0x74, 0x43, 0x6f, 0x6d,
    0x6d, 0x69, 0x74, 0x41, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x53, 0x65, 0x6e, 0x64, 0x57, 0x65,
    0x6c, 0x63, 0x6f, 0x6d, 0x65, 0x73, 0x48, 0x00, 0x52, 0x0c, 0x73, 0x65, 0x6e, 0x64, 0x57, 0x65,
    0x6c, 0x63, 0x6f, 0x6d, 0x65, 0x73, 0x1a, 0x61, 0x0a, 0x0c, 0x49, 0x6e, 0x73, 0x74, 0x61, 0x6c,
    0x6c, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x29, 0x0a, 0x10, 0x69, 0x6e, 0x73, 0x74, 0x61, 0x6c,
    0x6c, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c,
    0x52, 0x0f, 0x69, 0x6e, 0x73, 0x74, 0x61, 0x6c, 0x6c, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x4b, 0x65,
    0x79, 0x12, 0x26, 0x0a, 0x0f, 0x68, 0x70, 0x6b, 0x65, 0x5f, 0x70, 0x75, 0x62, 0x6c, 0x69, 0x63,
    0x5f, 0x6b, 0x65, 0x79, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x0d, 0x68, 0x70, 0x6b, 0x65,
    0x50, 0x75, 0x62, 0x6c, 0x69, 0x63, 0x4b, 0x65, 0x79, 0x1a, 0x8f, 0x01, 0x0a, 0x0c, 0x53, 0x65,
    0x6e, 0x64, 0x57, 0x65, 0x6c, 0x63, 0x6f, 0x6d, 0x65, 0x73, 0x12, 0x56, 0x0a, 0x0d, 0x69, 0x6e,
    0x73, 0x74, 0x61, 0x6c, 0x6c, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28,
    0x0b, 0x32, 0x30, 0x2e, 0x78, 0x6d, 0x74, 0x70, 0x2e, 0x6d, 0x6c, 0x73, 0x2e, 0x64, 0x61, 0x74,
    0x61, 0x62, 0x61, 0x73, 0x65, 0x2e, 0x50, 0x6f, 0x73, 0x74, 0x43, 0x6f, 0x6d, 0x6d, 0x69, 0x74,
    0x41, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x49, 0x6e, 0x73, 0x74, 0x61, 0x6c, 0x6c, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x52, 0x0d, 0x69, 0x6e, 0x73, 0x74, 0x61, 0x6c, 0x6c, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x73, 0x12, 0x27, 0x0a, 0x0f, 0x77, 0x65, 0x6c, 0x63, 0x6f, 0x6d, 0x65, 0x5f, 0x6d, 0x65,
    0x73, 0x73, 0x61, 0x67, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x0e, 0x77, 0x65, 0x6c,
    0x63, 0x6f, 0x6d, 0x65, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x42, 0x06, 0x0a, 0x04, 0x6b,
    0x69, 0x6e, 0x64, 0x42, 0xb5, 0x01, 0x0a, 0x15, 0x63, 0x6f, 0x6d, 0x2e, 0x78, 0x6d, 0x74, 0x70,
    0x2e, 0x6d, 0x6c, 0x73, 0x2e, 0x64, 0x61, 0x74, 0x61, 0x62, 0x61, 0x73, 0x65, 0x42, 0x0c, 0x49,
    0x6e, 0x74, 0x65, 0x6e, 0x74, 0x73, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0x5a, 0x28, 0x67,
    0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x78, 0x6d, 0x74, 0x70, 0x2f, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x2f, 0x76, 0x33, 0x2f, 0x67, 0x6f, 0x2f, 0x6d, 0x6c, 0x73, 0x2f, 0x64,
    0x61, 0x74, 0x61, 0x62, 0x61, 0x73, 0x65, 0xa2, 0x02, 0x03, 0x58, 0x4d, 0x44, 0xaa, 0x02, 0x11,
    0x58, 0x6d, 0x74, 0x70, 0x2e, 0x4d, 0x6c, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x62, 0x61, 0x73,
    0x65, 0xca, 0x02, 0x11, 0x58, 0x6d, 0x74, 0x70, 0x5c, 0x4d, 0x6c, 0x73, 0x5c, 0x44, 0x61, 0x74,
    0x61, 0x62, 0x61, 0x73, 0x65, 0xe2, 0x02, 0x1d, 0x58, 0x6d, 0x74, 0x70, 0x5c, 0x4d, 0x6c, 0x73,
    0x5c, 0x44, 0x61, 0x74, 0x61, 0x62, 0x61, 0x73, 0x65, 0x5c, 0x47, 0x50, 0x42, 0x4d, 0x65, 0x74,
    0x61, 0x64, 0x61, 0x74, 0x61, 0xea, 0x02, 0x13, 0x58, 0x6d, 0x74, 0x70, 0x3a, 0x3a, 0x4d, 0x6c,
    0x73, 0x3a, 0x3a, 0x44, 0x61, 0x74, 0x61, 0x62, 0x61, 0x73, 0x65, 0x4a, 0x8e, 0x16, 0x0a, 0x06,
    0x12, 0x04, 0x02, 0x00, 0x6c, 0x01, 0x0a, 0x27, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x02, 0x00, 0x12,
    0x32, 0x1d, 0x20, 0x56, 0x33, 0x20, 0x69, 0x6e, 0x76, 0x69, 0x74, 0x65, 0x20, 0x6d, 0x65, 0x73,
    0x73, 0x61, 0x67, 0x65, 0x20, 0x73, 0x74, 0x72, 0x75, 0x63, 0x74, 0x75, 0x72, 0x65, 0x0a, 0x0a,
    0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x04, 0x00, 0x1a, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03,
    0x06, 0x00, 0x3f, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0b, 0x12, 0x03, 0x06, 0x00, 0x3f, 0x0a, 0x34,
    0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x0a, 0x00, 0x13, 0x01, 0x1a, 0x28, 0x20, 0x54, 0x68, 0x65,
    0x20, 0x64, 0x61, 0x74, 0x61, 0x20, 0x72, 0x65, 0x71, 0x75, 0x69, 0x72, 0x65, 0x64, 0x20, 0x74,
    0x6f, 0x20, 0x70, 0x75, 0x62, 0x6c, 0x69, 0x73, 0x68, 0x20, 0x61, 0x20, 0x6d, 0x65, 0x73, 0x73,
    0x61, 0x67, 0x65, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x0a, 0x08, 0x17,
    0x0a, 0x2c, 0x0a, 0x04, 0x04, 0x00, 0x03, 0x00, 0x12, 0x04, 0x0c, 0x02, 0x0e, 0x03, 0x1a, 0x1e,
    0x20, 0x56, 0x31, 0x20, 0x6f, 0x66, 0x20, 0x53, 0x65, 0x6e, 0x64, 0x4d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x50, 0x75, 0x62, 0x6c, 0x69, 0x73, 0x68, 0x44, 0x61, 0x74, 0x61, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x03, 0x00, 0x01, 0x12, 0x03, 0x0c, 0x0a, 0x0c, 0x0a, 0x0d, 0x0a, 0x06,
    0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0d, 0x04, 0x1c, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x00, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0d, 0x04, 0x09, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x00, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0d, 0x0a, 0x17, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x00, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0d, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x00, 0x08, 0x00, 0x12, 0x04, 0x10, 0x02, 0x12, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x08,
    0x00, 0x01, 0x12, 0x03, 0x10, 0x08, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x11, 0x04, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x11,
    0x04, 0x06, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x11, 0x07, 0x09,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x11, 0x0c, 0x0d, 0x0a, 0x45,
    0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x16, 0x00, 0x18, 0x01, 0x1a, 0x39, 0x20, 0x57, 0x72, 0x61,
    0x70, 0x70, 0x65, 0x72, 0x20, 0x61, 0x72, 0x6f, 0x75, 0x6e, 0x64, 0x20, 0x61, 0x20, 0x6c, 0x69,
    0x73, 0x74, 0x20, 0x61, 0x66, 0x20, 0x72, 0x65, 0x70, 0x65, 0x61, 0x74, 0x65, 0x64, 0x20, 0x45,
    0x56, 0x4d, 0x20, 0x41, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x20, 0x41, 0x64, 0x64, 0x72, 0x65,
    0x73, 0x73, 0x65, 0x73, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x16, 0x08,
    0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x17, 0x02, 0x28, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x17, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x17, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x17, 0x12, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x17, 0x26, 0x27, 0x0a, 0x40, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x1b, 0x00,
    0x1d, 0x01, 0x1a, 0x34, 0x20, 0x57, 0x72, 0x61, 0x70, 0x70, 0x65, 0x72, 0x20, 0x61, 0x72, 0x6f,
    0x75, 0x6e, 0x64, 0x20, 0x61, 0x20, 0x6c, 0x69, 0x73, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x72, 0x65,
    0x70, 0x65, 0x61, 0x74, 0x65, 0x64, 0x20, 0x49, 0x6e, 0x73, 0x74, 0x61, 0x6c, 0x6c, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x20, 0x49, 0x44, 0x73, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12,
    0x03, 0x1b, 0x08, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x1c, 0x02,
    0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x1c, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x1c, 0x0b, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1c, 0x11, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1c, 0x24, 0x25, 0x0a, 0x3e, 0x0a, 0x02, 0x04, 0x03, 0x12,
    0x04, 0x20, 0x00, 0x25, 0x01, 0x1a, 0x32, 0x20, 0x4f, 0x6e, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x61,
    0x6e, 0x20, 0x45, 0x56, 0x4d, 0x20, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x20, 0x61, 0x64,
    0x64, 0x72, 0x65, 0x73, 0x73, 0x20, 0x6f, 0x72, 0x20, 0x49, 0x6e, 0x73, 0x74, 0x61, 0x6c, 0x6c,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x49, 0x44, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01,
    0x12, 0x03, 0x20, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x03, 0x08, 0x00, 0x12, 0x04, 0x21,
    0x02, 0x24, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x08, 0x00, 0x01, 0x12, 0x03, 0x21, 0x08,
    0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x22, 0x04, 0x2b, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x06, 0x12, 0x03, 0x22, 0x04, 0x14, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x22, 0x15, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x22, 0x29, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01,
    0x12, 0x03, 0x23, 0x04, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x06, 0x12, 0x03,
    0x23, 0x04, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x23, 0x14,
    0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x23, 0x27, 0x28, 0x0a,
    0x39, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x28, 0x00, 0x31, 0x01, 0x1a, 0x2d, 0x20, 0x54, 0x68,
    0x65, 0x20, 0x64, 0x61, 0x74, 0x61, 0x20, 0x72, 0x65, 0x71, 0x75, 0x69, 0x72, 0x65, 0x64, 0x20,
    0x74, 0x6f, 0x20, 0x61, 0x64, 0x64, 0x20, 0x6d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x73, 0x20, 0x74,
    0x6f, 0x20, 0x61, 0x20, 0x67, 0x72, 0x6f, 0x75, 0x70, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04,
    0x01, 0x12, 0x03, 0x28, 0x08, 0x16, 0x0a, 0x2b, 0x0a, 0x04, 0x04, 0x04, 0x03, 0x00, 0x12, 0x04,
    0x2a, 0x02, 0x2c, 0x03, 0x1a, 0x1d, 0x20, 0x56, 0x31, 0x20, 0x6f, 0x66, 0x20, 0x41, 0x64, 0x64,
    0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x73, 0x50, 0x75, 0x62, 0x6c, 0x69, 0x73, 0x68, 0x44, 0x61,
    0x74, 0x61, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x03, 0x00, 0x01, 0x12, 0x03, 0x2a, 0x0a,
    0x0c, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x04, 0x03, 0x00, 0x02, 0x00, 0x12, 0x03, 0x2b, 0x04, 0x41,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x2b, 0x04, 0x1e,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2b, 0x1f, 0x3c,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2b, 0x3f, 0x40,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x04, 0x08, 0x00, 0x12, 0x04, 0x2e, 0x02, 0x30, 0x03, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x08, 0x00, 0x01, 0x12, 0x03, 0x2e, 0x08, 0x0f, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x2f, 0x04, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x00, 0x06, 0x12, 0x03, 0x2f, 0x04, 0x06, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x2f, 0x07, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x2f, 0x0c, 0x0d, 0x0a, 0x3e, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x34, 0x00, 0x3d, 0x01, 0x1a,
    0x32, 0x20, 0x54, 0x68, 0x65, 0x20, 0x64, 0x61, 0x74, 0x61, 0x20, 0x72, 0x65, 0x71, 0x75, 0x69,
    0x72, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x72, 0x65, 0x6d, 0x6f, 0x76, 0x65, 0x20, 0x6d, 0x65,
    0x6d, 0x62, 0x65, 0x72, 0x73, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x61, 0x20, 0x67, 0x72, 0x6f,
    0x75, 0x70, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x34, 0x08, 0x19, 0x0a,
    0x2e, 0x0a, 0x04, 0x04, 0x05, 0x03, 0x00, 0x12, 0x04, 0x36, 0x02, 0x38, 0x03, 0x1a, 0x20, 0x20,
    0x56, 0x31, 0x20, 0x6f, 0x66, 0x20, 0x52, 0x65, 0x6d, 0x6f, 0x76, 0x65, 0x4d, 0x65, 0x6d, 0x62,
    0x65, 0x72, 0x73, 0x50, 0x75, 0x62, 0x6c, 0x69, 0x73, 0x68, 0x44, 0x61, 0x74, 0x61, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x03, 0x00, 0x01, 0x12, 0x03, 0x36, 0x0a, 0x0c, 0x0a, 0x0d, 0x0a,
    0x06, 0x04, 0x05, 0x03, 0x00, 0x02, 0x00, 0x12, 0x03, 0x37, 0x04, 0x41, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x05, 0x03, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x37, 0x04, 0x1e, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x05, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x37, 0x1f, 0x3c, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x05, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x37, 0x3f, 0x40, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x05, 0x08, 0x00, 0x12, 0x04, 0x3a, 0x02, 0x3c, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x08, 0x00, 0x01, 0x12, 0x03, 0x3a, 0x08, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00,
    0x12, 0x03, 0x3b, 0x04, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x06, 0x12, 0x03,
    0x3b, 0x04, 0x06, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3b, 0x07,
    0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x3b, 0x0c, 0x0d, 0x0a,
    0x73, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x41, 0x00, 0x4d, 0x01, 0x1a, 0x67, 0x20, 0x54, 0x68,
    0x65, 0x20, 0x64, 0x61, 0x74, 0x61, 0x20, 0x72, 0x65, 0x71, 0x75, 0x69, 0x72, 0x65, 0x64, 0x20,
    0x74, 0x6f, 0x20, 0x6d, 0x61, 0x6b, 0x65, 0x20, 0x61, 0x20, 0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74,
    0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x73, 0x20, 0x67, 0x72,
    0x6f, 0x75, 0x70, 0x20, 0x6d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x73, 0x68, 0x69, 0x70, 0x0a, 0x20,
    0x48, 0x61, 0x6e, 0x64, 0x6c, 0x65, 0x73, 0x20, 0x62, 0x6f, 0x74, 0x68, 0x20, 0x41, 0x64, 0x64,
    0x20, 0x61, 0x6e, 0x64, 0x20, 0x52, 0x65, 0x6d, 0x6f, 0x76, 0x65, 0x20, 0x61, 0x63, 0x74, 0x69,
    0x6f, 0x6e, 0x73, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x41, 0x08, 0x21,
    0x0a, 0x36, 0x0a, 0x04, 0x04, 0x06, 0x03, 0x00, 0x12, 0x04, 0x43, 0x02, 0x48, 0x03, 0x1a, 0x28,
    0x20, 0x56, 0x31, 0x20, 0x6f, 0x66, 0x20, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x47, 0x72, 0x6f,
    0x75, 0x70, 0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x73, 0x68, 0x69, 0x70, 0x50, 0x75, 0x62, 0x6c,
    0x69, 0x73, 0x68, 0x44, 0x61, 0x74, 0x61, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x03, 0x00,
    0x01, 0x12, 0x03, 0x43, 0x0a, 0x0c, 0x0a, 0x4d, 0x0a, 0x06, 0x04, 0x06, 0x03, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x45, 0x04, 0x2f, 0x1a, 0x3e, 0x20, 0x43, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x73,
    0x20, 0x64, 0x65, 0x6c, 0x74, 0x61, 0x20, 0x6f, 0x66, 0x20, 0x6d, 0x65, 0x6d, 0x62, 0x65, 0x72,
    0x73, 0x68, 0x69, 0x70, 0x20, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x73, 0x20, 0x74, 0x68, 0x61,
    0x74, 0x20, 0x6e, 0x65, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x62, 0x65, 0x20, 0x61, 0x70, 0x70,
    0x6c, 0x69, 0x65, 0x64, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x06, 0x03, 0x00, 0x02, 0x00, 0x06,
    0x12, 0x03, 0x45, 0x04, 0x17, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x06, 0x03, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x45, 0x18, 0x2a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x06, 0x03, 0x00, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x45, 0x2d, 0x2e, 0x0a, 0x42, 0x0a, 0x06, 0x04, 0x06, 0x03, 0x00, 0x02, 0x01, 0x12,
    0x03, 0x47, 0x04, 0x28, 0x1a, 0x33, 0x20, 0x43, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x73, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x6c, 0x69, 0x73, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x6d, 0x65, 0x6d, 0x62,
    0x65, 0x72, 0x73, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65,
    0x20, 0x72, 0x65, 0x6d, 0x6f, 0x76, 0x65, 0x64, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x06, 0x03,
    0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x47, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x06, 0x03,
    0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x47, 0x0d, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x06, 0x03,
    0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x47, 0x14, 0x23, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x06, 0x03,
    0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x47, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x06, 0x08,
    0x00, 0x12, 0x04, 0x4a, 0x02, 0x4c, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x08, 0x00, 0x01,
    0x12, 0x03, 0x4a, 0x08, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x03, 0x4b,
    0x04, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x06, 0x12, 0x03, 0x4b, 0x04, 0x06,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03, 0x4b, 0x07, 0x09, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x03, 0x4b, 0x0c, 0x0d, 0x0a, 0x38, 0x0a, 0x02,
    0x04, 0x07, 0x12, 0x04, 0x50, 0x00, 0x5a, 0x01, 0x1a, 0x2c, 0x20, 0x54, 0x68, 0x65, 0x20, 0x64,
    0x61, 0x74, 0x61, 0x20, 0x72, 0x65, 0x71, 0x75, 0x69, 0x72, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20,
    0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x20, 0x67, 0x72, 0x6f, 0x75, 0x70, 0x20, 0x6d, 0x65, 0x74,
    0x61, 0x64, 0x61, 0x74, 0x61, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x50,
    0x08, 0x1a, 0x0a, 0x2f, 0x0a, 0x04, 0x04, 0x07, 0x03, 0x00, 0x12, 0x04, 0x52, 0x02, 0x55, 0x03,
    0x1a, 0x21, 0x20, 0x56, 0x31, 0x20, 0x6f, 0x66, 0x20, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x4d,
    0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x50, 0x75, 0x62, 0x6c, 0x69, 0x73, 0x68, 0x44, 0x61,
    0x74, 0x61, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x03, 0x00, 0x01, 0x12, 0x03, 0x52, 0x0a,
    0x0c, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x07, 0x03, 0x00, 0x02, 0x00, 0x12, 0x03, 0x53, 0x04, 0x1a,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x53, 0x04, 0x0a,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x53, 0x0b, 0x15,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x53, 0x18, 0x19,
    0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x07, 0x03, 0x00, 0x02, 0x01, 0x12, 0x03, 0x54, 0x04, 0x1b, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x07, 0x03, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x54, 0x04, 0x0a, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x07, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x54, 0x0b, 0x16, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x07, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x54, 0x19, 0x1a, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x07, 0x08, 0x00, 0x12, 0x04, 0x57, 0x02, 0x59, 0x03, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x08, 0x00, 0x01, 0x12, 0x03, 0x57, 0x08, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x07, 0x02, 0x00, 0x12, 0x03, 0x58, 0x04, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00,
    0x06, 0x12, 0x03, 0x58, 0x04, 0x06, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x58, 0x07, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x03, 0x58,
    0x0c, 0x0d, 0x0a, 0x3b, 0x0a, 0x02, 0x04, 0x08, 0x12, 0x04, 0x5d, 0x00, 0x6c, 0x01, 0x1a, 0x2f,
    0x20, 0x47, 0x65, 0x6e, 0x65, 0x72, 0x69, 0x63, 0x20, 0x64, 0x61, 0x74, 0x61, 0x2d, 0x74, 0x79,
    0x70, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x61, 0x6c, 0x6c, 0x20, 0x70, 0x6f, 0x73, 0x74, 0x2d,
    0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x20, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x0a, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x08, 0x01, 0x12, 0x03, 0x5d, 0x08, 0x18, 0x0a, 0x1f, 0x0a, 0x04, 0x04,
    0x08, 0x03, 0x00, 0x12, 0x04, 0x5f, 0x02, 0x62, 0x03, 0x1a, 0x11, 0x20, 0x41, 0x6e, 0x20, 0x69,
    0x6e, 0x73, 0x74, 0x61, 0x6c, 0x6c, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x08, 0x03, 0x00, 0x01, 0x12, 0x03, 0x5f, 0x0a, 0x16, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x08,
    0x03, 0x00, 0x02, 0x00, 0x12, 0x03, 0x60, 0x04, 0x1f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x08, 0x03,
    0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x60, 0x04, 0x09, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x08, 0x03,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x60, 0x0a, 0x1a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x08, 0x03,
    0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x60, 0x1d, 0x1e, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x08, 0x03,
    0x00, 0x02, 0x01, 0x12, 0x03, 0x61, 0x04, 0x1e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x08, 0x03, 0x00,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x61, 0x04, 0x09, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x08, 0x03, 0x00,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x61, 0x0a, 0x19, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x08, 0x03, 0x00,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x61, 0x1c, 0x1d, 0x0a, 0x23, 0x0a, 0x04, 0x04, 0x08, 0x03, 0x01,
    0x12, 0x04, 0x64, 0x02, 0x67, 0x03, 0x1a, 0x15, 0x20, 0x53, 0x65, 0x6e, 0x64, 0x57, 0x65, 0x6c,
    0x63, 0x6f, 0x6d, 0x65, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x08, 0x03, 0x01, 0x01, 0x12, 0x03, 0x64, 0x0a, 0x16, 0x0a, 0x0d, 0x0a, 0x06, 0x04,
    0x08, 0x03, 0x01, 0x02, 0x00, 0x12, 0x03, 0x65, 0x04, 0x2c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x08,
    0x03, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x65, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x08,
    0x03, 0x01, 0x02, 0x00, 0x06, 0x12, 0x03, 0x65, 0x0d, 0x19, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x08,
    0x03, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x65, 0x1a, 0x27, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x08,
    0x03, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x65, 0x2a, 0x2b, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x08,
    0x03, 0x01, 0x02, 0x01, 0x12, 0x03, 0x66, 0x04, 0x1e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x08, 0x03,
    0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x66, 0x04, 0x09, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x08, 0x03,
    0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x66, 0x0a, 0x19, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x08, 0x03,
    0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x66, 0x1c, 0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x08, 0x08,
    0x00, 0x12, 0x04, 0x69, 0x02, 0x6b, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x08, 0x00, 0x01,
    0x12, 0x03, 0x69, 0x08, 0x0c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x00, 0x12, 0x03, 0x6a,
    0x04, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x06, 0x12, 0x03, 0x6a, 0x04, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x01, 0x12, 0x03, 0x6a, 0x11, 0x1e, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x03, 0x12, 0x03, 0x6a, 0x21, 0x22, 0x62, 0x06, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x33,
];
include!("xmtp.mls.database.serde.rs");
// @@protoc_insertion_point(module)