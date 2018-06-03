use serde::Serialize;
use serde_json;

use semver;

use util;

#[derive(Serialize)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64
}

impl Version {
    pub fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self {
            major: major,
            minor: minor,
            patch: patch
        }
    }
}

impl From<semver::Version> for Version {
    fn from(version: semver::Version) -> Self {
        Self {
            major: version.major,
            minor: version.major,
            patch: version.patch
        }
    }
}

#[derive(Serialize)]
pub struct Header {
    pub version: Version
}

#[derive(Serialize)]
pub struct Message<T: Serialize + Sized>{
    pub header: Header,
    pub payload: T
}

pub struct MessageBuilder<T: Serialize + Sized> {
    version: Version,
    payload: T
}

impl <T: Serialize + Default + Sized> MessageBuilder<T> {
    pub fn new() -> Self {
        Self {
            version: util::TENTACLE_VERSION,
            payload: T::default()
        }
    }

    pub fn version(mut self, version: Version) -> Self {
        self.version = version;
        self
    }

    pub fn payload(mut self, payload: T) -> Self {
        self.payload = payload;
        self
    }

    pub fn build(self) -> Message<T> {
        Message {
            header: Header { version: self.version },
            payload: self.payload
        }
    }
}
