//! Find the ID of a device by its path, or find a device path by its ID.

use std::fs;
use std::path::{Path, PathBuf};

/// Describes a partition identity.
/// 
/// A device path may be recovered from this.
#[derive(Clone, Debug, Hash, PartialEq)]
pub struct PartitionID {
    pub variant: PartitionSource,
    pub id: String
}

impl PartitionID {
    /// Construct a new `PartitionID` as the given source.
    pub fn new(variant: PartitionSource, id: String) -> Self {
        Self { variant, id }
    }

    /// Construct a new `PartitionID` as a `UUID` source.
    pub fn new_uuid(id: String) -> Self {
        Self::new(PartitionSource::UUID, id)
    }

    /// Construct a new `PartitionID` as a `PartUUID` source.
    pub fn new_partuuid(id: String) -> Self {
        Self::new(PartitionSource::PartUUID, id)
    }

    /// Find the device path of this ID.
    pub fn get_device_path(&self) -> Option<PathBuf> {
        from_uuid(&self.id, Self::dir(self.variant))
    }

    /// Find the given source ID of the device at the given path.
    pub fn get_source<P: AsRef<Path>>(variant: PartitionSource, path: P) -> Option<Self> {
        Some(Self {
            variant,
            id: find_uuid(path.as_ref(), Self::dir(variant))?
        })
    }

    /// Find the UUID of the device at the given path.
    pub fn get_uuid<P: AsRef<Path>>(path: P) -> Option<Self> {
        Self::get_source(PartitionSource::UUID, path)
    }

    /// Find the PARTUUID of the device at the given path.
    pub fn get_partuuid<P: AsRef<Path>>(path: P) -> Option<Self> {
        Self::get_source(PartitionSource::PartUUID, path)
    }

    fn dir(variant: PartitionSource) -> fs::ReadDir {
        let idpath = variant.disk_by_path();
        idpath.read_dir().expect(&format!("unable to find {:?}", idpath))
    }
}

/// Describes the type of partition identity.
#[derive(Copy, Clone, Debug, Hash, PartialEq)]
pub enum PartitionSource {
    ID,
    Label,
    PartLabel,
    PartUUID,
    Path,
    UUID
}

impl From<PartitionSource> for &'static str {
    fn from(pid: PartitionSource) -> &'static str {
        match pid {
            PartitionSource::ID => "id",
            PartitionSource::Label => "label",
            PartitionSource::PartLabel => "partlabel",
            PartitionSource::PartUUID => "partuuid",
            PartitionSource::Path => "path",
            PartitionSource::UUID => "uuid"
        }
    }
}

impl PartitionSource {
    fn disk_by_path(self) -> PathBuf {
        PathBuf::from(["/dev/disk/by-", <&'static str>::from(self)].concat())
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