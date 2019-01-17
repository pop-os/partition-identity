//! Find the ID of a device by its path, or find a device path by its ID.

#[macro_use]
extern crate err_derive;

use self::PartitionSource::*;
use self::PartitionSource::Path as SourcePath;
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;

#[derive(Debug, Error, PartialEq)]
pub enum Error {
    #[error(display = "the partition ID key was invalid")]
    InvalidKey,
    #[error(display = "the provided path was not valid in this context")]
    InvalidPath,
    #[error(display = "the provided `/dev/disk/by-` path was not supported")]
    UnknownByPath
}

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

    /// Construct a new `PartitionID` as a `ID` source.
    pub fn new_id(id: String) -> Self {
        Self::new(ID, id)
    }

    /// Construct a new `PartitionID` as a `Label` source.
    pub fn new_label(id: String) -> Self {
        Self::new(Label, id)
    }

    /// Construct a new `PartitionID` as a `UUID` source.
    pub fn new_uuid(id: String) -> Self {
        Self::new(UUID, id)
    }

    /// Construct a new `PartitionID` as a `PartLabel` source.
    pub fn new_partlabel(id: String) -> Self {
        Self::new(PartLabel, id)
    }

    /// Construct a new `PartitionID` as a `PartUUID` source.
    pub fn new_partuuid(id: String) -> Self {
        Self::new(PartUUID, id)
    }

    /// Construct a new `PartitionID` as a `Path` source.
    pub fn new_path(id: String) -> Self {
        Self::new(SourcePath, id)
    }

    /// Find the device path of this ID.
    pub fn get_device_path(&self) -> Option<PathBuf> {
        from_uuid(&self.id, Self::dir(self.variant)?)
    }

    /// Find the given source ID of the device at the given path.
    pub fn get_source<P: AsRef<Path>>(variant: PartitionSource, path: P) -> Option<Self> {
        Some(Self {
            variant,
            id: find_uuid(path.as_ref(), Self::dir(variant)?)?
        })
    }

    /// Find the UUID of the device at the given path.
    pub fn get_uuid<P: AsRef<Path>>(path: P) -> Option<Self> {
        Self::get_source(UUID, path)
    }

    /// Find the PARTUUID of the device at the given path.
    pub fn get_partuuid<P: AsRef<Path>>(path: P) -> Option<Self> {
        Self::get_source(PartUUID, path)
    }

    /// Fetch a partition ID by a `/dev/disk/by-` path.
    pub fn from_disk_by_path<S: AsRef<str>>(path: S) -> Result<Self, Error> {
        let path = path.as_ref();

        let path = if path.starts_with("/dev/disk/by-") {
            &path[13..]
        } else {
            return Err(Error::InvalidPath);
        };

        let id = if path.starts_with("id/") {
            Self::new(ID, path[3..].into())
        } else if path.starts_with("label/") {
            Self::new(Label, path[6..].into())
        } else if path.starts_with("partlabel/") {
            Self::new(PartLabel, path[10..].into())
        } else if path.starts_with("partuuid/") {
            Self::new(PartUUID, path[9..].into())
        } else if path.starts_with("path/") {
            Self::new(Path, path[5..].into())
        } else if path.starts_with("uuid/") {
            Self::new(UUID, path[5..].into())
        } else {
            return Err(Error::UnknownByPath);
        };

        Ok(id)
    }

    fn dir(variant: PartitionSource) -> Option<fs::ReadDir> {
        let idpath = variant.disk_by_path();
        idpath.read_dir().ok()
    }
}

impl FromStr for PartitionID {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input.starts_with('/') {
            Ok(PartitionID { variant: SourcePath, id: input.to_owned() })
        } else if input.starts_with("ID=") {
            Ok(PartitionID { variant: ID, id: input[3..].to_owned() })
        } else if input.starts_with("LABEL=") {
            Ok(PartitionID { variant: Label, id: input[6..].to_owned() })
        } else if input.starts_with("PARTLABEL=") {
            Ok(PartitionID { variant: PartLabel, id: input[10..].to_owned() })
        } else if input.starts_with("PARTUUID=") {
            Ok(PartitionID { variant: PartUUID, id: input[9..].to_owned() })
        } else if input.starts_with("UUID=") {
            Ok(PartitionID { variant: UUID, id: input[5..].to_owned() })
        } else {
            Err(Error::InvalidKey)
        }
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

/// A collection of all discoverable identifiers for a partition.
#[derive(Debug, Default, Clone, Hash, PartialEq)]
pub struct PartitionIdentifiers {
    pub id: Option<String>,
    pub label: Option<String>,
    pub part_label: Option<String>,
    pub part_uuid: Option<String>,
    pub path: Option<String>,
    pub uuid: Option<String>
}

impl PartitionIdentifiers {
    /// Fetches all discoverable identifiers for a partition by the path to that partition.
    pub fn from_path<P: AsRef<Path>>(path: P) -> PartitionIdentifiers {
        let path = path.as_ref();

        PartitionIdentifiers {
            path: PartitionID::get_source(SourcePath, path).map(|id| id.id),
            id: PartitionID::get_source(ID, path).map(|id| id.id),
            label: PartitionID::get_source(Label, path).map(|id| id.id),
            part_label: PartitionID::get_source(PartLabel, path).map(|id| id.id),
            part_uuid: PartitionID::get_source(PartUUID, path).map(|id| id.id),
            uuid: PartitionID::get_source(UUID, path).map(|id| id.id),
        }
    }

    /// Checks if the given identity matches one of the available identifiers.
    pub fn matches(&self, id: &PartitionID) -> bool {
        match id.variant {
            ID => self.id.as_ref().map_or(false, |s| &id.id == s),
            Label => self.label.as_ref().map_or(false, |s| &id.id == s),
            PartLabel => self.part_label.as_ref().map_or(false, |s| &id.id == s),
            PartUUID => self.part_uuid.as_ref().map_or(false, |s| &id.id == s),
            SourcePath => self.path.as_ref().map_or(false, |s| &id.id == s),
            UUID => self.uuid.as_ref().map_or(false, |s| &id.id == s),
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partition_id_from_str() {
        assert_eq!(
            "/dev/sda1".parse::<PartitionID>(),
            Ok(PartitionID::new_path("/dev/sda1".into()))
        );

        assert_eq!(
            "ID=abcd".parse::<PartitionID>(),
            Ok(PartitionID::new_id("abcd".into()))
        );

        assert_eq!(
            "LABEL=abcd".parse::<PartitionID>(),
            Ok(PartitionID::new_label("abcd".into()))
        );

        assert_eq!(
            "PARTLABEL=abcd".parse::<PartitionID>(),
            Ok(PartitionID::new_partlabel("abcd".into()))
        );

        assert_eq!(
            "PARTUUID=abcd".parse::<PartitionID>(),
            Ok(PartitionID::new_partuuid("abcd".into()))
        );

        assert_eq!(
            "UUID=abcd".parse::<PartitionID>(),
            Ok(PartitionID::new_uuid("abcd".into()))
        );
    }
}
