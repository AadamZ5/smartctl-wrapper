use std::fmt::Display;

use crate::smartctl_dev::{
    SmartCtlCapacityInfo, SmartCtlDevice, SmartCtlDeviceFormFactor, SmartCtlDeviceHealth,
    SmartCtlWwn,
};
use anyhow::Error;
use log::{debug, warn};
use serde::Deserialize;
use std::process::Command;

pub type JsonVersionType = (u8, u8);

pub const SUPPORTED_JSON_VERSIONS: [JsonVersionType; 1] = [(1, 0)];
pub const SUPPORTED_SMARTCTL_VERSIONS: [JsonVersionType; 3] = [(7, 2), (7, 3), (7, 4)];

#[derive(Debug, Clone)]
pub struct SmartCtlVersionInfo {
    pub json_format_version: JsonVersionType,
    pub smartctl_version: JsonVersionType,
    pub svn_revision: String,
    pub platform_info: String,
    pub build_info: String,
}

impl Display for SmartCtlVersionInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Write like this:
        // smartctl 7.2 2020-12-30 r5155 [x86_64-linux-6.0.6-76060006-generic] (local build)

        write!(
            f,
            "smartctl {}.{} r{} [{}] {} (json {}.{})",
            self.smartctl_version.0,
            self.smartctl_version.1,
            self.svn_revision,
            self.platform_info,
            self.build_info,
            self.json_format_version.0,
            self.json_format_version.1,
        )
    }
}

pub struct SmartCtl {
    path: String,

    version_info: SmartCtlVersionInfo,
}

fn _parse_version(version_json: &serde_json::Value) -> Result<JsonVersionType, Error> {
    // Version should be an array
    let version_array = match version_json.as_array() {
        Some(array) => Ok(array),
        None => Err(Error::msg("Invalid version format")),
    }?;

    // Version array should have at least two numbers
    // We only know how to handle the first two right now
    if version_array.len() < 2 {
        return Err(Error::msg("Invalid version format").context(format!(
            "Expected at least two numbers, got {}",
            version_array.len()
        )));
    } else if version_array.len() > 2 {
        warn!(
            "Warning: Only the first two numbers of the version are supported, ignoring the rest"
        );
    }

    let version_tuple = match (version_array[0].as_u64(), version_array[1].as_u64()) {
        (Some(major), Some(minor)) => Ok((major as u8, minor as u8)),
        _ => Err(Error::msg("Invalid version format")),
    }?;

    return Ok(version_tuple);
}

fn _validate_version(
    json_version: JsonVersionType,
    supported_versions: &[JsonVersionType],
) -> bool {
    for supported_version in supported_versions.iter() {
        if json_version.0 == supported_version.0 && json_version.1 == supported_version.1 {
            return true;
        }
    }

    return false;
}

impl SmartCtl {
    // Construct a new wrapper instance for the `smartctl` binary
    pub fn new(path: Option<String>) -> Result<SmartCtl, Error> {
        // Determine the path if supplied
        let path = match path {
            Some(path) => path,
            None => match Command::new("which").arg("smartctl").output() {
                Ok(output) => {
                    let output = String::from_utf8(output.stdout)?;
                    output.trim().to_string()
                }
                Err(_) => String::from("smartctl"),
            },
        };

        // Try a simple scan output to get the version info stuff up top
        let scan_output = Command::new(&path)
            .arg("-j")
            .arg("--scan")
            .output()
            .map_err(|e| Error::msg(format!("Failed to execute smartctl ({:?}): {}", path, e)))?;

        let scan_output = String::from_utf8(scan_output.stdout)?;

        //TODO Parse JSON and check supported json-format version!

        let scan_output: serde_json::Value = serde_json::from_str(&scan_output)?;

        let json_version = &scan_output["json_format_version"];
        let json_version_tuple = _parse_version(json_version)?;
        if !_validate_version(json_version_tuple, &SUPPORTED_JSON_VERSIONS) {
            return Err(Error::msg("Unsupported JSON version").context(format!(
                "Expected one of {:?}, got {:?}",
                SUPPORTED_JSON_VERSIONS, json_version_tuple
            )));
        }

        let smartctl_version = &scan_output["smartctl"]["version"];
        let smartctl_version_tuple = _parse_version(smartctl_version)?;
        if !_validate_version(smartctl_version_tuple, &SUPPORTED_SMARTCTL_VERSIONS) {
            return Err(Error::msg("Unsupported smartctl version").context(format!(
                "Expected one of {:?}, got {:?}",
                SUPPORTED_SMARTCTL_VERSIONS, smartctl_version_tuple
            )));
        }

        let svn_revision = scan_output["smartctl"]["svn_revision"]
            .as_str()
            .ok_or(Error::msg("Invalid svn_revision format"))?
            .to_string();
        let platform_info = scan_output["smartctl"]["platform_info"]
            .as_str()
            .ok_or(Error::msg("Invalid platform_info format"))?
            .to_string();
        let build_info = scan_output["smartctl"]["build_info"]
            .as_str()
            .ok_or(Error::msg("Invalid build_info format"))?
            .to_string();

        let version_info = SmartCtlVersionInfo {
            json_format_version: json_version_tuple,
            smartctl_version: smartctl_version_tuple,
            svn_revision,
            platform_info,
            build_info,
        };

        debug!("JSON Version: {:?}", json_version_tuple);
        debug!("smartctl Version: {:?}", smartctl_version_tuple);
        debug!("SVN Revision: {}", version_info.svn_revision);
        debug!("Platform Info: {}", version_info.platform_info);
        debug!("Build Info: {}", version_info.build_info);
        debug!("Using path {:?}", path);

        let definition = SmartCtl {
            path: path.to_string(),
            version_info,
        };

        return Ok(definition);
    }

    pub fn get_version_info(&self) -> &SmartCtlVersionInfo {
        &self.version_info
    }

    pub fn get_path(&self) -> String {
        self.path.clone()
    }

    pub fn execute(&self, args: Vec<String>) -> Result<serde_json::Value, Error> {
        let output = Command::new(&self.path).args(args).output().map_err(|e| {
            Error::msg(format!(
                "Failed to execute smartctl ({:?}): {}",
                self.path, e
            ))
        })?;

        let output = String::from_utf8(output.stdout)?;

        let output: serde_json::Value = serde_json::from_str(&output)?;

        return Ok(output);
    }

    pub fn get_device(&self, device_path: String) -> Result<SmartCtlDevice, Error> {
        let json_response =
            self.execute(vec!["-j".to_string(), "--info".to_string(), device_path])?;

        let form_factor =
            SmartCtlDeviceFormFactor::deserialize(&json_response["device"]["form_factor"]).ok();

        let wwn = SmartCtlWwn::deserialize(&json_response["wwn"]).ok();

        let capacity = SmartCtlCapacityInfo {
            blocks: json_response["user_capacity"]["blocks"]
                .as_u64()
                .ok_or(Error::msg("Invalid user_capacity/blocks format"))?,
            bytes: json_response["user_capacity"]["bytes"]
                .as_u64()
                .ok_or(Error::msg("Invalid user_capacity/bytes format"))?,
            logical_block_size: json_response["logical_block_size"]
                .as_u64()
                .ok_or(Error::msg("Invalid logical_block_size format"))?,
            physical_block_size: json_response["physical_block_size"]
                .as_u64()
                .ok_or(Error::msg("Invalid physical_block_size format"))?,
        };

        let device = SmartCtlDevice {
            name: json_response["device"]["name"]
                .as_str()
                .unwrap_or("Unknown")
                .to_string(),
            info_name: json_response["device"]["info_name"]
                .as_str()
                .unwrap_or("Unknown")
                .to_string(),
            type_: json_response["device"]["type"]
                .as_str()
                .unwrap_or("Unknown")
                .to_string(),
            protocol: json_response["device"]["protocol"]
                .as_str()
                .unwrap_or("Unknown")
                .to_string(),

            model_family: json_response["device"]["model_family"]
                .as_str()
                .map(|s| s.to_string()),
            model_name: json_response["model_name"].as_str().map(|s| s.to_string()),
            serial_number: json_response["serial_number"]
                .as_str()
                .map(|s| s.to_string()),
            firmware_version: json_response["firmware_version"]
                .as_str()
                .map(|s| s.to_string()),
            wwn,
            capacity,
            rotation_rate: json_response["rotation_rate"].as_u64(),

            form_factor,

            device_health: SmartCtlDeviceHealth {},
        };

        return Ok(device);
    }
}
