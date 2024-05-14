use std::sync::{Arc, Mutex, MutexGuard};

use diesel::sql_types::Binary;
use diesel::{deserialize::QueryableByName, sql_query, RunQueryDsl};
use log::error;
use openmls_traits::storage::*;
use serde::Serialize;
use serde_json::{from_slice, from_value, Value};

use super::encrypted_store::db_connection::DbConnection;

#[derive(QueryableByName, Debug)]
#[table_name = "openmls_key_value"]
struct StorageData {
    #[column_name = "value_bytes"]
    #[sql_type = "Binary"]
    value_bytes: Vec<u8>,
}

#[derive(Debug)]
pub struct SqlKeyStore<'a> {
    // Directly wrap the DbConnection which is a SqliteConnection in this case
    conn: Arc<Mutex<&'a DbConnection<'a>>>,
}

impl<'a> SqlKeyStore<'a> {
    pub fn new(conn: &'a DbConnection<'a>) -> Self {
        Self {
            conn: Arc::new(Mutex::new(conn)),
        }
    }

    pub fn conn(&self) -> Arc<Mutex<&'a DbConnection<'a>>> {
        Arc::clone(&self.conn)
    }

    pub fn write<const VERSION: u16>(
        &self,
        label: &[u8],
        key: &[u8],
        value: &[u8],
    ) -> Result<(), <Self as StorageProvider<CURRENT_VERSION>>::Error> {
        let storage_key = build_key_from_vec::<VERSION>(label, key.to_vec());
        let query =
            "REPLACE INTO openmls_key_value (key_bytes, version, value_bytes) VALUES (?, ?, ?)";

        let conn = self.conn.lock().unwrap();
        let _ = conn.raw_query(|conn| {
            sql_query(query)
                .bind::<diesel::sql_types::Binary, _>(&storage_key)
                .bind::<diesel::sql_types::Integer, _>(VERSION as i32)
                .bind::<diesel::sql_types::Binary, _>(&value)
                .execute(conn)
        });

        Ok(())
    }

    pub fn append<const VERSION: u16>(
        &self,
        label: &[u8],
        key: &[u8],
        value: &[u8],
    ) -> Result<(), <Self as StorageProvider<CURRENT_VERSION>>::Error> {
        let storage_key = build_key_from_vec::<VERSION>(label, key.to_vec());
        let select_query =
            "SELECT value_bytes FROM openmls_key_value WHERE key_bytes = ? AND version = ?";
        let update_query =
            "UPDATE openmls_key_value SET value_bytes = ? WHERE key_bytes = ? AND version = ?";

        let conn: MutexGuard<_> = self.conn.lock().unwrap();

        let current_data: Result<Vec<StorageData>, diesel::result::Error> =
            conn.raw_query(|conn| {
                sql_query(select_query)
                    .bind::<diesel::sql_types::Binary, _>(&storage_key)
                    .bind::<diesel::sql_types::Integer, _>(VERSION as i32)
                    .load(conn)
            });

        match current_data {
            Ok(data) => {
                if let Some(entry) = data.into_iter().next() {
                    // The value in the storage is an array of array of bytes, encoded as json.
                    match from_slice::<Value>(&entry.value_bytes) {
                        Ok(mut deserialized) => {
                            // Assuming value is JSON and needs to be added to an array
                            if let Value::Array(ref mut arr) = deserialized {
                                arr.push(Value::from(value));
                            }
                            // eprintln!(
                            //     "  got the value: {}",
                            //     serde_json::to_string(&deserialized).unwrap()
                            // );
                            // arr.push(value);
                            let modified_data = serde_json::to_vec(&deserialized)
                                .map_err(|_| MemoryStorageError::SerializationError)?;
                            // eprintln!("  modified_data: {modified_data}");

                            let _ = conn.raw_query(|conn| {
                                sql_query(update_query)
                                    .bind::<diesel::sql_types::Binary, _>(&modified_data)
                                    .bind::<diesel::sql_types::Binary, _>(&storage_key)
                                    .bind::<diesel::sql_types::Integer, _>(VERSION as i32)
                                    .execute(conn)
                            });
                            Ok(())
                        }
                        Err(_e) => Err(MemoryStorageError::SerializationError),
                    }
                } else {
                    eprintln!("  first entry ...");
                    // Add a first entry
                    let query = "REPLACE INTO openmls_key_value (key_bytes, version, value_bytes) VALUES (?, ?, ?)";
                    let _ = conn.raw_query(|conn| {
                        sql_query(query)
                            .bind::<diesel::sql_types::Binary, _>(&storage_key)
                            .bind::<diesel::sql_types::Integer, _>(VERSION as i32)
                            .bind::<diesel::sql_types::Binary, _>(
                                &serde_json::to_vec(&[value]).unwrap(),
                            )
                            .execute(conn)
                    });

                    Ok(())
                }
            }
            Err(_) => Err(MemoryStorageError::None),
        }
    }

    pub fn remove_item<const VERSION: u16>(
        &self,
        label: &[u8],
        key: &[u8],
        value: &[u8],
    ) -> Result<(), <Self as StorageProvider<CURRENT_VERSION>>::Error> {
        let storage_key = build_key_from_vec::<VERSION>(label, key.to_vec());
        let select_query =
            "SELECT value_bytes FROM openmls_key_value WHERE key_bytes = ? AND version = ?";
        let update_query =
            "UPDATE openmls_key_value SET value_bytes = ? WHERE key_bytes = ? AND version = ?";
        let conn: MutexGuard<&DbConnection<'a>> = self.conn.lock().unwrap();

        let current_data: Result<Vec<StorageData>, diesel::result::Error> =
            conn.raw_query(|conn| {
                sql_query(select_query)
                    .bind::<diesel::sql_types::Binary, _>(&storage_key)
                    .bind::<diesel::sql_types::Integer, _>(VERSION as i32)
                    .load(conn)
            });

        match current_data {
            Ok(data) => {
                if let Some(entry) = data.into_iter().next() {
                    // The value in the storage is an array of array of bytes, encoded as json.
                    match from_slice::<Value>(&entry.value_bytes) {
                        Ok(mut deserialized) => {
                            if let Value::Array(ref mut arr) = deserialized {
                                // Find and remove the valu.
                                let vpos = arr.iter().position(|v| {
                                    from_value::<Vec<u8>>(v.clone()).unwrap() == value
                                });
                                if let Some(pos) = vpos {
                                    arr.remove(pos);
                                }
                            }
                            let modified_data = serde_json::to_vec(&deserialized)
                                .map_err(|_| MemoryStorageError::SerializationError)?;

                            let _ = conn.raw_query(|conn| {
                                sql_query(update_query)
                                    .bind::<diesel::sql_types::Binary, _>(&modified_data)
                                    .bind::<diesel::sql_types::Binary, _>(&storage_key)
                                    .bind::<diesel::sql_types::Integer, _>(VERSION as i32)
                                    .execute(conn)
                            });
                            Ok(())
                        }
                        Err(_e) => Err(MemoryStorageError::SerializationError),
                    }
                } else {
                    eprintln!("  first entry ...");
                    // Add a first entry
                    let query = "REPLACE INTO openmls_key_value (key_bytes, version, value_bytes) VALUES (?, ?, ?)";
                    let _ = conn.raw_query(|conn| {
                        sql_query(query)
                            .bind::<diesel::sql_types::Binary, _>(&storage_key)
                            .bind::<diesel::sql_types::Integer, _>(VERSION as i32)
                            .bind::<diesel::sql_types::Binary, _>(
                                &serde_json::to_vec(&[value]).unwrap(),
                            )
                            .execute(conn)
                    });

                    Ok(())
                }
            }
            Err(_) => Err(MemoryStorageError::None),
        }
    }

    pub fn read<const VERSION: u16, V: Entity<VERSION>>(
        &self,
        label: &[u8],
        key: &[u8],
    ) -> Result<Option<V>, <Self as StorageProvider<CURRENT_VERSION>>::Error> {
        let storage_key = build_key_from_vec::<VERSION>(label, key.to_vec());
        let query = "SELECT value_bytes FROM openmls_key_value WHERE key_bytes = ? AND version = ?";
        let conn: MutexGuard<&DbConnection<'a>> = self.conn.lock().unwrap();

        let results: Result<Vec<StorageData>, diesel::result::Error> = conn.raw_query(|conn| {
            sql_query(query)
                .bind::<diesel::sql_types::Binary, _>(&storage_key)
                .bind::<diesel::sql_types::Integer, _>(VERSION as i32)
                .load(conn)
        });

        match results {
            Ok(data) => {
                if let Some(entry) = data.into_iter().next() {
                    match serde_json::from_slice::<V>(&entry.value_bytes) {
                        Ok(deserialized) => Ok(Some(deserialized)),
                        Err(e) => {
                            eprintln!("Error occurred: {}", e);
                            Err(MemoryStorageError::SerializationError)
                        }
                    }
                } else {
                    Ok(None)
                }
            }
            Err(_e) => Err(MemoryStorageError::None),
        }
    }

    pub fn read_list<const VERSION: u16, V: Entity<VERSION>>(
        &self,
        label: &[u8],
        key: &[u8],
    ) -> Result<Vec<V>, <Self as StorageProvider<CURRENT_VERSION>>::Error> {
        let storage_key = build_key_from_vec::<VERSION>(label, key.to_vec());
        let query = "SELECT value_bytes FROM openmls_key_value WHERE key_bytes = ? AND version = ?";
        let conn: MutexGuard<&DbConnection<'a>> = self.conn.lock().unwrap();
        match conn.raw_query(|conn| {
            sql_query(query)
                .bind::<diesel::sql_types::Binary, _>(&storage_key)
                .bind::<diesel::sql_types::Integer, _>(VERSION as i32)
                .load::<StorageData>(conn)
        }) {
            Ok(results) => {
                if let Some(entry) = results.into_iter().next() {
                    // eprintln!(
                    //     "  got raw value: {:?}",
                    //     String::from_utf8(entry.value_bytes.clone()).unwrap()
                    // );
                    // The value has to be an array.
                    // This is always an array of bytes right now.
                    let list = from_slice::<Vec<Vec<u8>>>(&entry.value_bytes).unwrap();

                    // Read the values from the bytes in the list
                    return Ok(list
                        .into_iter()
                        .map(|v| serde_json::from_slice(&v).unwrap())
                        .collect::<Vec<V>>());
                }
                Err(MemoryStorageError::None)
            }
            Err(_e) => Err(MemoryStorageError::None),
        }
    }

    pub fn delete<const VERSION: u16>(
        &self,
        label: &[u8],
        key: &[u8],
    ) -> Result<(), <Self as StorageProvider<CURRENT_VERSION>>::Error> {
        let storage_key = build_key_from_vec::<VERSION>(label, key.to_vec());
        let query = "DELETE FROM openmls_key_value WHERE key_bytes = ? AND version = ?";
        let conn: MutexGuard<&DbConnection<'a>> = self.conn.lock().unwrap();
        let _ = conn.raw_query(|conn| {
            sql_query(query)
                .bind::<diesel::sql_types::Binary, _>(&storage_key)
                .bind::<diesel::sql_types::Integer, _>(VERSION as i32)
                .execute(conn)
        });
        Ok(())
    }
}

/// Errors thrown by the key store.
#[derive(thiserror::Error, Debug, Copy, Clone, PartialEq, Eq)]
pub enum MemoryStorageError {
    #[error("The key store does not allow storing serialized values.")]
    UnsupportedValueTypeBytes,
    #[error("Updating is not supported by this key store.")]
    UnsupportedMethod,
    #[error("Error serializing value.")]
    SerializationError,
    #[error("Value does not exist.")]
    None,
}

const KEY_PACKAGE_LABEL: &[u8] = b"KeyPackage";
const ENCRYPTION_KEY_PAIR_LABEL: &[u8] = b"EncryptionKeyPair";
const SIGNATURE_KEY_PAIR_LABEL: &[u8] = b"SignatureKeyPair";
const EPOCH_KEY_PAIRS_LABEL: &[u8] = b"EpochKeyPairs";

// related to PublicGroup
const TREE_LABEL: &[u8] = b"Tree";
const GROUP_CONTEXT_LABEL: &[u8] = b"GroupContext";
const INTERIM_TRANSCRIPT_HASH_LABEL: &[u8] = b"InterimTranscriptHash";
const CONFIRMATION_TAG_LABEL: &[u8] = b"ConfirmationTag";

// related to CoreGroup
const OWN_LEAF_NODE_INDEX_LABEL: &[u8] = b"OwnLeafNodeIndex";
const EPOCH_SECRETS_LABEL: &[u8] = b"EpochSecrets";
const MESSAGE_SECRETS_LABEL: &[u8] = b"MessageSecrets";
const USE_RATCHET_TREE_LABEL: &[u8] = b"UseRatchetTree";

// related to MlsGroup
const JOIN_CONFIG_LABEL: &[u8] = b"MlsGroupJoinConfig";
const OWN_LEAF_NODES_LABEL: &[u8] = b"OwnLeafNodes";
const AAD_LABEL: &[u8] = b"AAD";
const GROUP_STATE_LABEL: &[u8] = b"GroupState";
const QUEUED_PROPOSAL_LABEL: &[u8] = b"QueuedProposal";
const PROPOSAL_QUEUE_REFS_LABEL: &[u8] = b"ProposalQueueRefs";

impl StorageProvider<CURRENT_VERSION> for SqlKeyStore<'_> {
    type Error = MemoryStorageError;

    fn queue_proposal<
        GroupId: traits::GroupId<CURRENT_VERSION>,
        ProposalRef: traits::ProposalRef<CURRENT_VERSION>,
        QueuedProposal: traits::QueuedProposal<CURRENT_VERSION>,
    >(
        &self,
        group_id: &GroupId,
        proposal_ref: &ProposalRef,
        proposal: &QueuedProposal,
    ) -> Result<(), Self::Error> {
        // write proposal to key (group_id, proposal_ref)
        let key = serde_json::to_vec(&(group_id, proposal_ref))?;
        let value = serde_json::to_vec(proposal)?;
        self.write::<CURRENT_VERSION>(QUEUED_PROPOSAL_LABEL, &key, &value)?;

        // update proposal list for group_id
        let key = build_key::<CURRENT_VERSION, &GroupId>(PROPOSAL_QUEUE_REFS_LABEL, group_id);
        let value = serde_json::to_vec(proposal_ref)?;
        self.append::<CURRENT_VERSION>(PROPOSAL_QUEUE_REFS_LABEL, &key, &value)?;

        Ok(())
    }

    fn write_tree<
        GroupId: traits::GroupId<CURRENT_VERSION>,
        TreeSync: traits::TreeSync<CURRENT_VERSION>,
    >(
        &self,
        group_id: &GroupId,
        tree: &TreeSync,
    ) -> Result<(), Self::Error> {
        let key = build_key::<CURRENT_VERSION, &GroupId>(TREE_LABEL, group_id);
        self.write::<CURRENT_VERSION>(TREE_LABEL, &key, &serde_json::to_vec(&tree).unwrap())
    }

    fn write_interim_transcript_hash<
        GroupId: traits::GroupId<CURRENT_VERSION>,
        InterimTranscriptHash: traits::InterimTranscriptHash<CURRENT_VERSION>,
    >(
        &self,
        group_id: &GroupId,
        interim_transcript_hash: &InterimTranscriptHash,
    ) -> Result<(), Self::Error> {
        let key = build_key::<CURRENT_VERSION, &GroupId>(INTERIM_TRANSCRIPT_HASH_LABEL, group_id);
        let value = serde_json::to_vec(&interim_transcript_hash).unwrap();
        let _ = self.write::<CURRENT_VERSION>(INTERIM_TRANSCRIPT_HASH_LABEL, &key[..], &value[..]);

        Ok(())
    }

    fn write_context<
        GroupId: traits::GroupId<CURRENT_VERSION>,
        GroupContext: traits::GroupContext<CURRENT_VERSION>,
    >(
        &self,
        group_id: &GroupId,
        group_context: &GroupContext,
    ) -> Result<(), Self::Error> {
        let key = build_key::<CURRENT_VERSION, &GroupId>(GROUP_CONTEXT_LABEL, group_id);
        let value = serde_json::to_vec(&group_context).unwrap();
        let _ = self.write::<CURRENT_VERSION>(GROUP_CONTEXT_LABEL, &key[..], &value[..]);

        Ok(())
    }

    fn write_confirmation_tag<
        GroupId: traits::GroupId<CURRENT_VERSION>,
        ConfirmationTag: traits::ConfirmationTag<CURRENT_VERSION>,
    >(
        &self,
        group_id: &GroupId,
        confirmation_tag: &ConfirmationTag,
    ) -> Result<(), Self::Error> {
        let key = build_key::<CURRENT_VERSION, &GroupId>(CONFIRMATION_TAG_LABEL, group_id);
        let value = serde_json::to_vec(&confirmation_tag).unwrap();
        let _ = self.write::<CURRENT_VERSION>(CONFIRMATION_TAG_LABEL, &key[..], &value[..]);

        Ok(())
    }

    fn write_signature_key_pair<
        SignaturePublicKey: traits::SignaturePublicKey<CURRENT_VERSION>,
        SignatureKeyPair: traits::SignatureKeyPair<CURRENT_VERSION>,
    >(
        &self,
        public_key: &SignaturePublicKey,
        signature_key_pair: &SignatureKeyPair,
    ) -> Result<(), Self::Error> {
        let key =
            build_key::<CURRENT_VERSION, &SignaturePublicKey>(SIGNATURE_KEY_PAIR_LABEL, public_key);
        let value = serde_json::to_vec(&signature_key_pair).unwrap();
        let _ = self.write::<CURRENT_VERSION>(SIGNATURE_KEY_PAIR_LABEL, &key[..], &value[..]);

        Ok(())
    }

    fn queued_proposal_refs<
        GroupId: traits::GroupId<CURRENT_VERSION>,
        ProposalRef: traits::ProposalRef<CURRENT_VERSION>,
    >(
        &self,
        group_id: &GroupId,
    ) -> Result<Vec<ProposalRef>, Self::Error> {
        let key = build_key::<CURRENT_VERSION, &GroupId>(PROPOSAL_QUEUE_REFS_LABEL, group_id);
        self.read_list(PROPOSAL_QUEUE_REFS_LABEL, &key)
    }

    fn queued_proposals<
        GroupId: traits::GroupId<CURRENT_VERSION>,
        ProposalRef: traits::ProposalRef<CURRENT_VERSION>,
        QueuedProposal: traits::QueuedProposal<CURRENT_VERSION>,
    >(
        &self,
        group_id: &GroupId,
    ) -> Result<Vec<(ProposalRef, QueuedProposal)>, Self::Error> {
        let key = build_key::<CURRENT_VERSION, &GroupId>(PROPOSAL_QUEUE_REFS_LABEL, group_id);
        let refs: Vec<ProposalRef> = self.read_list(PROPOSAL_QUEUE_REFS_LABEL, &key)?;

        refs.into_iter()
            .map(|proposal_ref| -> Result<_, _> {
                let key = serde_json::to_vec(&(group_id, &proposal_ref))?;

                let proposal = self.read(QUEUED_PROPOSAL_LABEL, &key)?.unwrap();
                Ok((proposal_ref, proposal))
            })
            .collect::<Result<Vec<_>, _>>()
    }

    fn treesync<
        GroupId: traits::GroupId<CURRENT_VERSION>,
        TreeSync: traits::TreeSync<CURRENT_VERSION>,
    >(
        &self,
        group_id: &GroupId,
    ) -> Result<Option<TreeSync>, Self::Error> {
        let key = build_key::<CURRENT_VERSION, &GroupId>(TREE_LABEL, group_id);

        match self.read(TREE_LABEL, &key) {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    fn group_context<
        GroupId: traits::GroupId<CURRENT_VERSION>,
        GroupContext: traits::GroupContext<CURRENT_VERSION>,
    >(
        &self,
        group_id: &GroupId,
    ) -> Result<Option<GroupContext>, Self::Error> {
        let key = build_key::<CURRENT_VERSION, &GroupId>(GROUP_CONTEXT_LABEL, group_id);

        match self.read(GROUP_CONTEXT_LABEL, &key) {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    fn interim_transcript_hash<
        GroupId: traits::GroupId<CURRENT_VERSION>,
        InterimTranscriptHash: traits::InterimTranscriptHash<CURRENT_VERSION>,
    >(
        &self,
        group_id: &GroupId,
    ) -> Result<Option<InterimTranscriptHash>, Self::Error> {
        let key = build_key::<CURRENT_VERSION, &GroupId>(INTERIM_TRANSCRIPT_HASH_LABEL, group_id);

        match self.read(INTERIM_TRANSCRIPT_HASH_LABEL, &key) {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    fn confirmation_tag<
        GroupId: traits::GroupId<CURRENT_VERSION>,
        ConfirmationTag: traits::ConfirmationTag<CURRENT_VERSION>,
    >(
        &self,
        group_id: &GroupId,
    ) -> Result<Option<ConfirmationTag>, Self::Error> {
        let key = build_key::<CURRENT_VERSION, &GroupId>(CONFIRMATION_TAG_LABEL, group_id);

        match self.read(CONFIRMATION_TAG_LABEL, &key) {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    fn signature_key_pair<
        SignaturePublicKey: traits::SignaturePublicKey<CURRENT_VERSION>,
        SignatureKeyPair: traits::SignatureKeyPair<CURRENT_VERSION>,
    >(
        &self,
        public_key: &SignaturePublicKey,
    ) -> Result<Option<SignatureKeyPair>, Self::Error> {
        let key =
            build_key::<CURRENT_VERSION, &SignaturePublicKey>(SIGNATURE_KEY_PAIR_LABEL, public_key);

        match self.read(SIGNATURE_KEY_PAIR_LABEL, &key) {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    fn write_key_package<
        HashReference: traits::HashReference<CURRENT_VERSION>,
        KeyPackage: traits::KeyPackage<CURRENT_VERSION>,
    >(
        &self,
        hash_ref: &HashReference,
        key_package: &KeyPackage,
    ) -> Result<(), Self::Error> {
        let key = build_key::<CURRENT_VERSION, &HashReference>(KEY_PACKAGE_LABEL, hash_ref);
        let value = serde_json::to_vec(&key_package).unwrap();

        self.write::<CURRENT_VERSION>(KEY_PACKAGE_LABEL, &key, &value)
            .unwrap();

        Ok(())
    }

    fn write_psk<
        PskId: traits::PskId<CURRENT_VERSION>,
        PskBundle: traits::PskBundle<CURRENT_VERSION>,
    >(
        &self,
        _psk_id: &PskId,
        _psk: &PskBundle,
    ) -> Result<(), Self::Error> {
        Err(MemoryStorageError::UnsupportedMethod)
    }

    fn write_encryption_key_pair<
        EncryptionKey: traits::EncryptionKey<CURRENT_VERSION>,
        HpkeKeyPair: traits::HpkeKeyPair<CURRENT_VERSION>,
    >(
        &self,
        public_key: &EncryptionKey,
        key_pair: &HpkeKeyPair,
    ) -> Result<(), Self::Error> {
        let key =
            build_key::<CURRENT_VERSION, &EncryptionKey>(ENCRYPTION_KEY_PAIR_LABEL, public_key);
        self.write::<CURRENT_VERSION>(
            ENCRYPTION_KEY_PAIR_LABEL,
            &key,
            &serde_json::to_vec(key_pair).unwrap(),
        )
    }

    fn key_package<
        HashReference: traits::HashReference<CURRENT_VERSION>,
        KeyPackage: traits::KeyPackage<CURRENT_VERSION>,
    >(
        &self,
        hash_ref: &HashReference,
    ) -> Result<Option<KeyPackage>, Self::Error> {
        let key = build_key::<CURRENT_VERSION, &HashReference>(KEY_PACKAGE_LABEL, hash_ref);
        self.read(KEY_PACKAGE_LABEL, &key)
    }

    fn psk<PskBundle: traits::PskBundle<CURRENT_VERSION>, PskId: traits::PskId<CURRENT_VERSION>>(
        &self,
        _psk_id: &PskId,
    ) -> Result<Option<PskBundle>, Self::Error> {
        Err(MemoryStorageError::UnsupportedMethod)
    }

    fn encryption_key_pair<
        HpkeKeyPair: traits::HpkeKeyPair<CURRENT_VERSION>,
        EncryptionKey: traits::EncryptionKey<CURRENT_VERSION>,
    >(
        &self,
        public_key: &EncryptionKey,
    ) -> Result<Option<HpkeKeyPair>, Self::Error> {
        let key =
            build_key::<CURRENT_VERSION, &EncryptionKey>(ENCRYPTION_KEY_PAIR_LABEL, public_key);
        self.read(ENCRYPTION_KEY_PAIR_LABEL, &key)
    }

    fn delete_signature_key_pair<
        SignaturePublicKey: traits::SignaturePublicKey<CURRENT_VERSION>,
    >(
        &self,
        public_key: &SignaturePublicKey,
    ) -> Result<(), Self::Error> {
        let key =
            build_key::<CURRENT_VERSION, &SignaturePublicKey>(SIGNATURE_KEY_PAIR_LABEL, public_key);
        self.delete::<CURRENT_VERSION>(SIGNATURE_KEY_PAIR_LABEL, &key)
    }

    fn delete_encryption_key_pair<EncryptionKey: traits::EncryptionKey<CURRENT_VERSION>>(
        &self,
        public_key: &EncryptionKey,
    ) -> Result<(), Self::Error> {
        let key =
            build_key::<CURRENT_VERSION, &EncryptionKey>(ENCRYPTION_KEY_PAIR_LABEL, public_key);
        self.delete::<CURRENT_VERSION>(ENCRYPTION_KEY_PAIR_LABEL, &key)
    }

    fn delete_key_package<HashReference: traits::HashReference<CURRENT_VERSION>>(
        &self,
        hash_ref: &HashReference,
    ) -> Result<(), Self::Error> {
        let key = build_key::<CURRENT_VERSION, &HashReference>(KEY_PACKAGE_LABEL, hash_ref);
        self.delete::<CURRENT_VERSION>(KEY_PACKAGE_LABEL, &key)
    }

    fn delete_psk<PskKey: traits::PskId<CURRENT_VERSION>>(
        &self,
        _psk_id: &PskKey,
    ) -> Result<(), Self::Error> {
        Err(MemoryStorageError::UnsupportedMethod)
    }

    fn group_state<
        GroupState: traits::GroupState<CURRENT_VERSION>,
        GroupId: traits::GroupId<CURRENT_VERSION>,
    >(
        &self,
        group_id: &GroupId,
    ) -> Result<Option<GroupState>, Self::Error> {
        let key = build_key::<CURRENT_VERSION, &GroupId>(GROUP_STATE_LABEL, group_id);
        self.read(GROUP_STATE_LABEL, &key)
    }

    fn write_group_state<
        GroupState: traits::GroupState<CURRENT_VERSION>,
        GroupId: traits::GroupId<CURRENT_VERSION>,
    >(
        &self,
        group_id: &GroupId,
        group_state: &GroupState,
    ) -> Result<(), Self::Error> {
        let key = build_key::<CURRENT_VERSION, &GroupId>(GROUP_STATE_LABEL, group_id);
        self.write::<CURRENT_VERSION>(GROUP_STATE_LABEL, &key, &serde_json::to_vec(group_state)?)
    }

    fn delete_group_state<GroupId: traits::GroupId<CURRENT_VERSION>>(
        &self,
        group_id: &GroupId,
    ) -> Result<(), Self::Error> {
        let key = build_key::<CURRENT_VERSION, &GroupId>(GROUP_STATE_LABEL, group_id);
        self.delete::<CURRENT_VERSION>(GROUP_STATE_LABEL, &key)
    }

    fn message_secrets<
        GroupId: traits::GroupId<CURRENT_VERSION>,
        MessageSecrets: traits::MessageSecrets<CURRENT_VERSION>,
    >(
        &self,
        group_id: &GroupId,
    ) -> Result<Option<MessageSecrets>, Self::Error> {
        let key = build_key::<CURRENT_VERSION, &GroupId>(MESSAGE_SECRETS_LABEL, group_id);
        self.read(MESSAGE_SECRETS_LABEL, &key)
    }

    fn write_message_secrets<
        GroupId: traits::GroupId<CURRENT_VERSION>,
        MessageSecrets: traits::MessageSecrets<CURRENT_VERSION>,
    >(
        &self,
        group_id: &GroupId,
        message_secrets: &MessageSecrets,
    ) -> Result<(), Self::Error> {
        let key = build_key::<CURRENT_VERSION, &GroupId>(MESSAGE_SECRETS_LABEL, group_id);
        self.write::<CURRENT_VERSION>(
            MESSAGE_SECRETS_LABEL,
            &key,
            &serde_json::to_vec(message_secrets)?,
        )
    }

    fn delete_message_secrets<GroupId: traits::GroupId<CURRENT_VERSION>>(
        &self,
        group_id: &GroupId,
    ) -> Result<(), Self::Error> {
        let key = build_key::<CURRENT_VERSION, &GroupId>(MESSAGE_SECRETS_LABEL, group_id);
        self.delete::<CURRENT_VERSION>(MESSAGE_SECRETS_LABEL, &key)
    }

    fn resumption_psk_store<
        GroupId: traits::GroupId<CURRENT_VERSION>,
        ResumptionPskStore: traits::ResumptionPskStore<CURRENT_VERSION>,
    >(
        &self,
        _group_id: &GroupId,
    ) -> Result<Option<ResumptionPskStore>, Self::Error> {
        Err(MemoryStorageError::UnsupportedMethod)
    }

    fn write_resumption_psk_store<
        GroupId: traits::GroupId<CURRENT_VERSION>,
        ResumptionPskStore: traits::ResumptionPskStore<CURRENT_VERSION>,
    >(
        &self,
        _group_id: &GroupId,
        _resumption_psk_store: &ResumptionPskStore,
    ) -> Result<(), Self::Error> {
        Err(MemoryStorageError::UnsupportedMethod)
    }

    fn delete_all_resumption_psk_secrets<GroupId: traits::GroupId<CURRENT_VERSION>>(
        &self,
        _group_id: &GroupId,
    ) -> Result<(), Self::Error> {
        Err(MemoryStorageError::UnsupportedMethod)
    }

    fn own_leaf_index<
        GroupId: traits::GroupId<CURRENT_VERSION>,
        LeafNodeIndex: traits::LeafNodeIndex<CURRENT_VERSION>,
    >(
        &self,
        group_id: &GroupId,
    ) -> Result<Option<LeafNodeIndex>, Self::Error> {
        let key = build_key::<CURRENT_VERSION, &GroupId>(OWN_LEAF_NODE_INDEX_LABEL, group_id);
        self.read(OWN_LEAF_NODE_INDEX_LABEL, &key)
    }

    fn write_own_leaf_index<
        GroupId: traits::GroupId<CURRENT_VERSION>,
        LeafNodeIndex: traits::LeafNodeIndex<CURRENT_VERSION>,
    >(
        &self,
        group_id: &GroupId,
        own_leaf_index: &LeafNodeIndex,
    ) -> Result<(), Self::Error> {
        let key = build_key::<CURRENT_VERSION, &GroupId>(OWN_LEAF_NODE_INDEX_LABEL, group_id);
        self.write::<CURRENT_VERSION>(
            OWN_LEAF_NODE_INDEX_LABEL,
            &key,
            &serde_json::to_vec(own_leaf_index)?,
        )
    }

    fn delete_own_leaf_index<GroupId: traits::GroupId<CURRENT_VERSION>>(
        &self,
        group_id: &GroupId,
    ) -> Result<(), Self::Error> {
        let key = build_key::<CURRENT_VERSION, &GroupId>(OWN_LEAF_NODE_INDEX_LABEL, group_id);
        self.delete::<CURRENT_VERSION>(OWN_LEAF_NODE_INDEX_LABEL, &key)
    }

    fn use_ratchet_tree_extension<GroupId: traits::GroupId<CURRENT_VERSION>>(
        &self,
        group_id: &GroupId,
    ) -> Result<Option<bool>, Self::Error> {
        let key = build_key::<CURRENT_VERSION, &GroupId>(USE_RATCHET_TREE_LABEL, group_id);
        self.read(USE_RATCHET_TREE_LABEL, &key)
    }

    fn set_use_ratchet_tree_extension<GroupId: traits::GroupId<CURRENT_VERSION>>(
        &self,
        group_id: &GroupId,
        value: bool,
    ) -> Result<(), Self::Error> {
        let key = build_key::<CURRENT_VERSION, &GroupId>(USE_RATCHET_TREE_LABEL, group_id);
        self.write::<CURRENT_VERSION>(USE_RATCHET_TREE_LABEL, &key, &serde_json::to_vec(&value)?)
    }

    fn delete_use_ratchet_tree_extension<GroupId: traits::GroupId<CURRENT_VERSION>>(
        &self,
        group_id: &GroupId,
    ) -> Result<(), Self::Error> {
        let key = build_key::<CURRENT_VERSION, &GroupId>(USE_RATCHET_TREE_LABEL, group_id);
        self.delete::<CURRENT_VERSION>(USE_RATCHET_TREE_LABEL, &key)
    }

    fn group_epoch_secrets<
        GroupId: traits::GroupId<CURRENT_VERSION>,
        GroupEpochSecrets: traits::GroupEpochSecrets<CURRENT_VERSION>,
    >(
        &self,
        group_id: &GroupId,
    ) -> Result<Option<GroupEpochSecrets>, Self::Error> {
        let key = build_key::<CURRENT_VERSION, &GroupId>(EPOCH_SECRETS_LABEL, group_id);
        self.read(EPOCH_SECRETS_LABEL, &key)
    }

    fn write_group_epoch_secrets<
        GroupId: traits::GroupId<CURRENT_VERSION>,
        GroupEpochSecrets: traits::GroupEpochSecrets<CURRENT_VERSION>,
    >(
        &self,
        group_id: &GroupId,
        group_epoch_secrets: &GroupEpochSecrets,
    ) -> Result<(), Self::Error> {
        let key = build_key::<CURRENT_VERSION, &GroupId>(EPOCH_SECRETS_LABEL, group_id);
        self.write::<CURRENT_VERSION>(
            EPOCH_SECRETS_LABEL,
            &key,
            &serde_json::to_vec(group_epoch_secrets)?,
        )
    }

    fn delete_group_epoch_secrets<GroupId: traits::GroupId<CURRENT_VERSION>>(
        &self,
        group_id: &GroupId,
    ) -> Result<(), Self::Error> {
        let key = build_key::<CURRENT_VERSION, &GroupId>(EPOCH_SECRETS_LABEL, group_id);
        self.delete::<CURRENT_VERSION>(EPOCH_SECRETS_LABEL, &key)
    }

    fn write_encryption_epoch_key_pairs<
        GroupId: traits::GroupId<CURRENT_VERSION>,
        EpochKey: traits::EpochKey<CURRENT_VERSION>,
        HpkeKeyPair: traits::HpkeKeyPair<CURRENT_VERSION>,
    >(
        &self,
        group_id: &GroupId,
        epoch: &EpochKey,
        leaf_index: u32,
        key_pairs: &[HpkeKeyPair],
    ) -> Result<(), Self::Error> {
        let key = epoch_key_pairs_id(group_id, epoch, leaf_index)?;
        let value = serde_json::to_vec(key_pairs)?;
        log::debug!("Writing encryption epoch key pairs");
        #[cfg(feature = "test-utils")]
        {
            log::debug!("  key: {}", hex::encode(&key));
            log::debug!("  value: {}", hex::encode(&value));
        }

        self.write::<CURRENT_VERSION>(EPOCH_KEY_PAIRS_LABEL, &key, &value)
    }

    fn encryption_epoch_key_pairs<
        GroupId: traits::GroupId<CURRENT_VERSION>,
        EpochKey: traits::EpochKey<CURRENT_VERSION>,
        HpkeKeyPair: traits::HpkeKeyPair<CURRENT_VERSION>,
    >(
        &self,
        group_id: &GroupId,
        epoch: &EpochKey,
        leaf_index: u32,
    ) -> Result<Vec<HpkeKeyPair>, Self::Error> {
        let key = epoch_key_pairs_id(group_id, epoch, leaf_index)?;
        let storage_key = build_key_from_vec::<CURRENT_VERSION>(EPOCH_KEY_PAIRS_LABEL, key);
        log::debug!("Reading encryption epoch key pairs");

        match self.read_list(EPOCH_KEY_PAIRS_LABEL, &storage_key) {
            Ok(data) => {
                #[cfg(feature = "test-utils")]
                log::debug!("  value: {}", hex::encode(&data));
                serde_json::from_slice::<Vec<HpkeKeyPair>>(&data)
                    .map_err(|_e| MemoryStorageError::SerializationError)
            }
            Err(e) => {
                log::error!("Failed to read from storage: {}", e);
                Err(e)
            }
        }
    }

    fn delete_encryption_epoch_key_pairs<
        GroupId: traits::GroupId<CURRENT_VERSION>,
        EpochKey: traits::EpochKey<CURRENT_VERSION>,
    >(
        &self,
        group_id: &GroupId,
        epoch: &EpochKey,
        leaf_index: u32,
    ) -> Result<(), Self::Error> {
        let key = epoch_key_pairs_id(group_id, epoch, leaf_index)?;
        self.delete::<CURRENT_VERSION>(EPOCH_KEY_PAIRS_LABEL, &key)
    }

    fn clear_proposal_queue<
        GroupId: traits::GroupId<CURRENT_VERSION>,
        ProposalRef: traits::ProposalRef<CURRENT_VERSION>,
    >(
        &self,
        group_id: &GroupId,
    ) -> Result<(), Self::Error> {
        let key = build_key::<CURRENT_VERSION, &GroupId>(PROPOSAL_QUEUE_REFS_LABEL, group_id);
        let proposal_refs: Vec<ProposalRef> = self.read_list(PROPOSAL_QUEUE_REFS_LABEL, &key)?;

        for proposal_ref in proposal_refs {
            let key = serde_json::to_vec(&(group_id, proposal_ref))?;
            let _ = self.delete::<CURRENT_VERSION>(QUEUED_PROPOSAL_LABEL, &key);
        }

        let key = build_key::<CURRENT_VERSION, &GroupId>(PROPOSAL_QUEUE_REFS_LABEL, group_id);
        let _ = self.delete::<CURRENT_VERSION>(PROPOSAL_QUEUE_REFS_LABEL, &key);

        Ok(())
    }

    fn mls_group_join_config<
        GroupId: traits::GroupId<CURRENT_VERSION>,
        MlsGroupJoinConfig: traits::MlsGroupJoinConfig<CURRENT_VERSION>,
    >(
        &self,
        group_id: &GroupId,
    ) -> Result<Option<MlsGroupJoinConfig>, Self::Error> {
        let key = build_key::<CURRENT_VERSION, &GroupId>(JOIN_CONFIG_LABEL, group_id);
        self.read(JOIN_CONFIG_LABEL, &key)
    }

    fn write_mls_join_config<
        GroupId: traits::GroupId<CURRENT_VERSION>,
        MlsGroupJoinConfig: traits::MlsGroupJoinConfig<CURRENT_VERSION>,
    >(
        &self,
        group_id: &GroupId,
        config: &MlsGroupJoinConfig,
    ) -> Result<(), Self::Error> {
        let key = build_key::<CURRENT_VERSION, &GroupId>(JOIN_CONFIG_LABEL, group_id);
        let value = serde_json::to_vec(config).unwrap();

        self.write::<CURRENT_VERSION>(JOIN_CONFIG_LABEL, &key, &value)
    }

    fn own_leaf_nodes<
        GroupId: traits::GroupId<CURRENT_VERSION>,
        LeafNode: traits::LeafNode<CURRENT_VERSION>,
    >(
        &self,
        group_id: &GroupId,
    ) -> Result<Vec<LeafNode>, Self::Error> {
        let key = build_key::<CURRENT_VERSION, &GroupId>(OWN_LEAF_NODES_LABEL, group_id);
        self.read_list(OWN_LEAF_NODES_LABEL, &key)
    }

    fn append_own_leaf_node<
        GroupId: traits::GroupId<CURRENT_VERSION>,
        LeafNode: traits::LeafNode<CURRENT_VERSION>,
    >(
        &self,
        group_id: &GroupId,
        leaf_node: &LeafNode,
    ) -> Result<(), Self::Error> {
        let key = build_key::<CURRENT_VERSION, &GroupId>(OWN_LEAF_NODES_LABEL, group_id);
        let value = serde_json::to_vec(leaf_node)?;
        self.append::<CURRENT_VERSION>(OWN_LEAF_NODES_LABEL, &key, &value)
    }

    fn clear_own_leaf_nodes<GroupId: traits::GroupId<CURRENT_VERSION>>(
        &self,
        group_id: &GroupId,
    ) -> Result<(), Self::Error> {
        let key = build_key::<CURRENT_VERSION, &GroupId>(OWN_LEAF_NODES_LABEL, group_id);
        self.delete::<CURRENT_VERSION>(OWN_LEAF_NODES_LABEL, &key)
    }

    fn aad<GroupId: traits::GroupId<CURRENT_VERSION>>(
        &self,
        group_id: &GroupId,
    ) -> Result<Vec<u8>, Self::Error> {
        let key = build_key::<CURRENT_VERSION, &GroupId>(AAD_LABEL, group_id);
        self.read::<CURRENT_VERSION, Vec<u8>>(AAD_LABEL, &key)
            .map(|v| v.unwrap_or_default())
    }

    fn write_aad<GroupId: traits::GroupId<CURRENT_VERSION>>(
        &self,
        group_id: &GroupId,
        aad: &[u8],
    ) -> Result<(), Self::Error> {
        let key = build_key::<CURRENT_VERSION, &GroupId>(AAD_LABEL, group_id);
        let value = serde_json::to_vec(&aad).unwrap();

        self.write::<CURRENT_VERSION>(AAD_LABEL, &key, &value)
    }

    fn delete_aad<GroupId: traits::GroupId<CURRENT_VERSION>>(
        &self,
        group_id: &GroupId,
    ) -> Result<(), Self::Error> {
        let key = build_key::<CURRENT_VERSION, &GroupId>(AAD_LABEL, group_id);
        self.delete::<CURRENT_VERSION>(AAD_LABEL, &key)
    }

    fn delete_own_leaf_nodes<GroupId: traits::GroupId<CURRENT_VERSION>>(
        &self,
        group_id: &GroupId,
    ) -> Result<(), Self::Error> {
        let key = build_key::<CURRENT_VERSION, &GroupId>(OWN_LEAF_NODES_LABEL, group_id);
        self.delete::<CURRENT_VERSION>(OWN_LEAF_NODES_LABEL, &key)
    }

    fn delete_group_config<GroupId: traits::GroupId<CURRENT_VERSION>>(
        &self,
        group_id: &GroupId,
    ) -> Result<(), Self::Error> {
        let key = build_key::<CURRENT_VERSION, &GroupId>(JOIN_CONFIG_LABEL, group_id);
        self.delete::<CURRENT_VERSION>(JOIN_CONFIG_LABEL, &key)
    }

    fn delete_tree<GroupId: traits::GroupId<CURRENT_VERSION>>(
        &self,
        group_id: &GroupId,
    ) -> Result<(), Self::Error> {
        let key = build_key::<CURRENT_VERSION, &GroupId>(TREE_LABEL, group_id);
        self.delete::<CURRENT_VERSION>(TREE_LABEL, &key)
    }

    fn delete_confirmation_tag<GroupId: traits::GroupId<CURRENT_VERSION>>(
        &self,
        group_id: &GroupId,
    ) -> Result<(), Self::Error> {
        let key = build_key::<CURRENT_VERSION, &GroupId>(CONFIRMATION_TAG_LABEL, group_id);
        self.delete::<CURRENT_VERSION>(CONFIRMATION_TAG_LABEL, &key)
    }

    fn delete_context<GroupId: traits::GroupId<CURRENT_VERSION>>(
        &self,
        group_id: &GroupId,
    ) -> Result<(), Self::Error> {
        let key = build_key::<CURRENT_VERSION, &GroupId>(GROUP_CONTEXT_LABEL, group_id);
        self.delete::<CURRENT_VERSION>(GROUP_CONTEXT_LABEL, &key)
    }

    fn delete_interim_transcript_hash<GroupId: traits::GroupId<CURRENT_VERSION>>(
        &self,
        group_id: &GroupId,
    ) -> Result<(), Self::Error> {
        let key = build_key::<CURRENT_VERSION, &GroupId>(INTERIM_TRANSCRIPT_HASH_LABEL, group_id);
        self.delete::<CURRENT_VERSION>(INTERIM_TRANSCRIPT_HASH_LABEL, &key)
    }

    fn remove_proposal<
        GroupId: traits::GroupId<CURRENT_VERSION>,
        ProposalRef: traits::ProposalRef<CURRENT_VERSION>,
    >(
        &self,
        group_id: &GroupId,
        proposal_ref: &ProposalRef,
    ) -> Result<(), Self::Error> {
        // Delete the proposal ref
        let key = build_key::<CURRENT_VERSION, &GroupId>(PROPOSAL_QUEUE_REFS_LABEL, group_id);
        let value = serde_json::to_vec(proposal_ref)?;
        self.remove_item::<CURRENT_VERSION>(PROPOSAL_QUEUE_REFS_LABEL, &key, &value)?;

        // Delete the proposal
        let key = serde_json::to_vec(&(group_id, proposal_ref))?;
        self.delete::<CURRENT_VERSION>(QUEUED_PROPOSAL_LABEL, &key)
    }
}

/// Build a key with version and label.
fn build_key_from_vec<const V: u16>(label: &[u8], key: Vec<u8>) -> Vec<u8> {
    let mut key_out = label.to_vec();
    key_out.extend_from_slice(&key);
    key_out.extend_from_slice(&u16::to_be_bytes(V));
    key_out
}

/// Build a key with version and label.
fn build_key<const V: u16, K: Serialize>(label: &[u8], key: K) -> Vec<u8> {
    build_key_from_vec::<V>(label, serde_json::to_vec(&key).unwrap())
}

fn epoch_key_pairs_id(
    group_id: &impl traits::GroupId<CURRENT_VERSION>,
    epoch: &impl traits::EpochKey<CURRENT_VERSION>,
    leaf_index: u32,
) -> Result<Vec<u8>, MemoryStorageError> {
    let mut key = serde_json::to_vec(group_id)?;
    key.extend_from_slice(&serde_json::to_vec(epoch)?);
    key.extend_from_slice(&serde_json::to_vec(&leaf_index)?);
    Ok(key)
}

impl From<serde_json::Error> for MemoryStorageError {
    fn from(_: serde_json::Error) -> Self {
        Self::SerializationError
    }
}

#[cfg(test)]
mod tests {
    use openmls::group::GroupId;
    use openmls_basic_credential::{SignatureKeyPair, StorageId};
    use openmls_traits::{
        storage::{traits, Entity, Key, StorageProvider, CURRENT_VERSION},
        OpenMlsProvider,
    };
    use serde::{Deserialize, Serialize};

    use super::SqlKeyStore;
    use crate::{
        configuration::CIPHERSUITE,
        storage::{sql_key_store::MemoryStorageError, EncryptedMessageStore, StorageOption},
        utils::test::tmp_path,
        xmtp_openmls_provider::XmtpOpenMlsProvider,
    };

    #[test]
    fn store_read_delete() {
        let db_path = tmp_path();
        let store = EncryptedMessageStore::new(
            StorageOption::Persistent(db_path),
            EncryptedMessageStore::generate_enc_key(),
        )
        .unwrap();
        let conn = &store.conn().unwrap();
        let key_store = SqlKeyStore::new(conn);

        let signature_keys = SignatureKeyPair::new(CIPHERSUITE.signature_algorithm()).unwrap();
        let public_key = StorageId::from(signature_keys.to_public_vec());
        assert!(key_store
            .signature_key_pair::<StorageId, SignatureKeyPair>(&public_key)
            .unwrap()
            .is_none());

        key_store
            .write_signature_key_pair::<StorageId, SignatureKeyPair>(&public_key, &signature_keys)
            .unwrap();

        assert!(key_store
            .signature_key_pair::<StorageId, SignatureKeyPair>(&public_key)
            .unwrap()
            .is_some());

        key_store
            .delete_signature_key_pair::<StorageId>(&public_key)
            .unwrap();

        assert!(key_store
            .signature_key_pair::<StorageId, SignatureKeyPair>(&public_key)
            .unwrap()
            .is_none());
    }

    #[test]
    fn list_write_remove() {
        let db_path = tmp_path();
        let store = EncryptedMessageStore::new(
            StorageOption::Persistent(db_path),
            EncryptedMessageStore::generate_enc_key(),
        )
        .unwrap();
        let conn = &store.conn().unwrap();
        let key_store = SqlKeyStore::new(conn);
        let provider = XmtpOpenMlsProvider::new(&conn);
        let group_id = GroupId::random(provider.rand());

        assert!(key_store.aad::<GroupId>(&group_id).unwrap().is_empty());
        // let json_str = r#"{"data": "aad"}"#;
        // let json_obj: serde_json::Value = serde_json::from_str(json_str).unwrap();
        // let stringified_json = serde_json::to_string(&json_obj).unwrap();

        key_store
            .write_aad::<GroupId>(&group_id, &"test".as_bytes())
            .unwrap();

        assert!(!key_store.aad::<GroupId>(&group_id).unwrap().is_empty());

        key_store.delete_aad::<GroupId>(&group_id).unwrap();

        assert!(key_store.aad::<GroupId>(&group_id).unwrap().is_empty());
    }

    #[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
    struct Proposal(Vec<u8>);
    impl traits::QueuedProposal<CURRENT_VERSION> for Proposal {}
    impl Entity<CURRENT_VERSION> for Proposal {}

    #[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Copy)]
    struct ProposalRef(usize);
    impl traits::ProposalRef<CURRENT_VERSION> for ProposalRef {}
    impl Key<CURRENT_VERSION> for ProposalRef {}
    impl Entity<CURRENT_VERSION> for ProposalRef {}

    #[tokio::test]
    async fn list_append_remove() {
        let db_path = tmp_path();
        let store = EncryptedMessageStore::new(
            StorageOption::Persistent(db_path),
            EncryptedMessageStore::generate_enc_key(),
        )
        .unwrap();
        let conn = &store.conn().unwrap();
        let key_store = SqlKeyStore::new(conn);
        let provider = XmtpOpenMlsProvider::new(&conn);
        let group_id = GroupId::random(provider.rand());
        let proposals = (0..10)
            .map(|i| Proposal(format!("TestProposal{i}").as_bytes().to_vec()))
            .collect::<Vec<_>>();

        // Store proposals
        for (i, proposal) in proposals.iter().enumerate() {
            key_store
                .queue_proposal::<GroupId, ProposalRef, Proposal>(
                    &group_id,
                    &ProposalRef(i),
                    proposal,
                )
                .unwrap();
        }

        // Read proposal refs
        let proposal_refs_read: Vec<ProposalRef> =
            key_store.queued_proposal_refs(&group_id).unwrap();
        assert_eq!(
            (0..10).map(|i| ProposalRef(i)).collect::<Vec<_>>(),
            proposal_refs_read
        );

        // Read proposals
        let proposals_read: Vec<(ProposalRef, Proposal)> =
            key_store.queued_proposals(&group_id).unwrap();
        let proposals_expected: Vec<(ProposalRef, Proposal)> = (0..10)
            .map(|i| ProposalRef(i))
            .zip(proposals.clone().into_iter())
            .collect();
        assert_eq!(proposals_expected, proposals_read);

        // Remove proposal 5
        key_store
            .remove_proposal(&group_id, &ProposalRef(5))
            .unwrap();

        let proposal_refs_read: Vec<ProposalRef> =
            key_store.queued_proposal_refs(&group_id).unwrap();
        let mut expected = (0..10).map(|i| ProposalRef(i)).collect::<Vec<_>>();
        expected.remove(5);
        assert_eq!(expected, proposal_refs_read);

        let proposals_read: Vec<(ProposalRef, Proposal)> =
            key_store.queued_proposals(&group_id).unwrap();
        let mut proposals_expected: Vec<(ProposalRef, Proposal)> = (0..10)
            .map(|i| ProposalRef(i))
            .zip(proposals.clone().into_iter())
            .collect();
        proposals_expected.remove(5);
        assert_eq!(proposals_expected, proposals_read);

        // Clear all proposals
        key_store
            .clear_proposal_queue::<GroupId, ProposalRef>(&group_id)
            .unwrap();
        let proposal_refs_read: Result<Vec<ProposalRef>, MemoryStorageError> =
            key_store.queued_proposal_refs(&group_id);
        assert!(matches!(
            proposal_refs_read.err(),
            Some(MemoryStorageError::None)
        ));

        let proposals_read: Result<Vec<(ProposalRef, Proposal)>, MemoryStorageError> =
            key_store.queued_proposals(&group_id);
        assert!(matches!(
            proposals_read.err(),
            Some(MemoryStorageError::None)
        ));
    }
}
