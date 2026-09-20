use core_foundation::{
    base::{kCFAllocatorDefault, TCFType},
    data::CFData,
    dictionary::CFDictionary,
    propertylist::{
        kCFPropertyListBinaryFormat_v1_0, CFPropertyListCreateData, CFPropertyListSubClass,
    },
};
use serde::de::DeserializeOwned;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DictParseError {
    #[error("Failed to serialize property list")]
    PropertyListData,

    #[error("Failed to parse plist: {0}")]
    Deserialize(#[from] plist::Error),
}

pub fn dict_into<T: DeserializeOwned>(data: CFDictionary) -> Result<T, DictParseError> {
    let data = unsafe {
        CFPropertyListCreateData(
            kCFAllocatorDefault,
            data.to_CFPropertyList().as_CFTypeRef(),
            kCFPropertyListBinaryFormat_v1_0,
            0,
            std::ptr::null_mut(),
        )
    };

    if data.is_null() {
        return Err(DictParseError::PropertyListData);
    }

    let plist_data = unsafe { CFData::wrap_under_create_rule(data) };

    Ok(plist::from_bytes::<T>(plist_data.bytes())?)
}

pub fn get_mac_name() -> Option<String> {
    let output = std::process::Command::new("scutil")
        .arg("--get")
        .arg("ComputerName")
        .output()
        .ok()?;

    Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
}
