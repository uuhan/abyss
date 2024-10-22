use std::collections::BTreeMap;

use super::*;

/// A simple map that can be used to store metadata
/// for the pagecache tenant.
#[derive(Clone, Debug, Eq, PartialEq, Default, Serialize, Deserialize)]
pub struct Meta {
    // TODO finality cleanup
    // Finality hash code
    // pub(crate) finality: [u8; 32],
    /// Hash to PageId for diff block
    pub(crate) blocks: BTreeMap<Vec<u8>, PageId>,
    /// Name to PageId for database bucket
    pub(crate) bucket: BTreeMap<Vec<u8>, PageId>,
}

impl Meta {
    /// Retrieve the Diff Block PageId associated with an identifier
    pub fn get_block(&self, table: &[u8]) -> Option<PageId> {
        self.blocks.get(table).cloned()
    }

    /// Set the Diff Block PageId associated with an identifier
    pub fn set_block(&mut self, name: Vec<u8>, pid: PageId) {
        self.blocks.insert(name, pid);
    }

    /// Remove the page mapping of Diff Block for a given identifier
    pub fn del_block(&mut self, name: &[u8]) -> Option<PageId> {
        self.blocks.remove(name)
    }

    /// Retrieve the PageId associated with an identifier
    pub fn get_bucket(&self, table: &[u8]) -> Option<PageId> {
        self.bucket.get(table).cloned()
    }

    /// Set the PageId associated with an identifier
    pub fn set_bucket(&mut self, name: Vec<u8>, pid: PageId) {
        self.bucket.insert(name, pid);
    }

    /// Remove the page mapping for a given identifier
    pub fn del_bucket(&mut self, name: &[u8]) -> Option<PageId> {
        self.bucket.remove(name)
    }

    /// Return the current rooted tenants in Meta
    pub fn block_tenants(&self) -> BTreeMap<Vec<u8>, PageId> {
        self.blocks.clone()
    }

    /// Return the current rooted tenants in Meta
    pub fn bucket_tenants(&self) -> BTreeMap<Vec<u8>, PageId> {
        self.bucket.clone()
    }

    pub(crate) fn size_in_bytes(&self) -> u64 {
        self.bucket
            .iter()
            .map(|(k, _pid)| k.len() as u64 + std::mem::size_of::<PageId>() as u64)
            .sum()
    }
}
