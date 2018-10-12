//! Find the ID of a device by its path, or find a device path by its ID.

use std::fs;
use std::path::{Path, PathBuf};

/// Describes a partition identity.
/// 
/// A device path may be recovered from this.
#[derive(Clone, Debug, Hash, PartialEq)]
pub struct PartitionID {
    pub variant: PartitionIDVariant,
    pub id: String
}

impl PartitionID {
    pub fn new(variant: PartitionIDVariant, id: String) -> Self {
        Self { variant, id }
    }
}

/// Describes the type of partition identity.
#[derive(Copy, Clone, Debug, Hash, PartialEq)]
pub enum PartitionIDVariant {
    ID,
    Label,
    PartLabel,
    PartUUID,
    Path,
    UUID
}

impl PartitionIDVariant {
    pub fn to_str(self) -> &'static str {
        match self {
            PartitionIDVariant::ID => "id",
            PartitionIDVariant::Label => "label",
            PartitionIDVariant::PartLabel => "partlabel",
            PartitionIDVariant::PartUUID => "partuuid",
            PartitionIDVariant::Path => "path",
            PartitionIDVariant::UUID => "uuid"
        }
    }

    pub fn disk_by_path(&self) -> PathBuf {
        PathBuf::from(["/dev/disk/by-", self.to_str()].concat())
    }
}


impl PartitionID {
    /// Find the ID of the device at the given path.
    pub fn by_id<P: AsRef<Path>>(variant: PartitionIDVariant, path: P) -> Option<String> {
        find_uuid(path.as_ref(), Self::dir(variant))
    }

    /// Find the device path of this ID.
    pub fn from_id(&self) -> Option<PathBuf> {
        from_uuid(&self.id, Self::dir(self.variant))
    }

    fn dir(variant: PartitionIDVariant) -> fs::ReadDir {
        let idpath = variant.disk_by_path();
        idpath.read_dir().expect(&format!("unable to find {:?}", idpath))
    }
}

fn find_uuid(path: &Path, uuid_dir: fs::ReadDir) -> Option<String> {
    if let Ok(path) = path.canonicalize() {
        for uuid_entry in uuid_dir.filter_map(|entry| entry.ok()) {
            if let Ok(ref uuid_path) = uuid_entry.path().canonicalize() {
                if uuid_path == &path {
                    if let Some(uuid_entry) = uuid_entry.file_name().to_str() {
                        return Some(uuid_entry.into());
                    }
                }
            }
        }
    }

    None
}

fn from_uuid(uuid: &str, uuid_dir: fs::ReadDir) -> Option<PathBuf> {
    for uuid_entry in uuid_dir.filter_map(|entry| entry.ok()) {
        let uuid_entry = uuid_entry.path();
        if let Some(name) = uuid_entry.file_name() {
            if name == uuid {
                if let Ok(uuid_entry) = uuid_entry.canonicalize() {
                    return Some(uuid_entry);
                }
            }
        }
    }

    None
}