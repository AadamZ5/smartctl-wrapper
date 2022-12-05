use std::fmt::Display;

use crate::smartctl_dev::{
    SmartCtlCapacityInfo, SmartCtlDevice, SmartCtlDeviceFormFactor, SmartCtlDeviceHealth,
    SmartCtlWwn,
};
use anyhow::Error;
use log::{debug, error, warn};
use serde::Deserialize;
use std::process::Command;

pub type JsonVersionType = (u8, u8);

pub const SUPPORTED_JSON_VERSIONS: [JsonVersionType; 1] = [(1, 0)];
pub const SUPPORTED_SMARTCTL_VERSIONS: [JsonVersionType; 3] = [(7, 2), (7, 3), (7, 4)];

fn _parse_version(version_json: &serde_json::Value) -> Result<JsonVersionType, Error> {
    // Version should be an array
    let version_array = match version_json.as_array() {
        Some(array) => Ok(array),
        None => Err(Error::msg("Invalid version format")),
    }?;

    // Version array should have at least two numbers
    // We only know how to handle the first two right no

    //Ignore clippy here
    #[allow(clippy::comparison_chain)]
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

    Ok(version_tuple)
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

    false
}

fn parse_version_info<T>(scan_output: T) -> Result<SmartCtlVersionInfo, Error>
where
    T: Into<String>,
{
    let scan_output: serde_json::Value = serde_json::from_str(&scan_output.into())?;

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
        .ok_or_else(|| Error::msg("Invalid svn_revision format"))?
        .to_string();
    let platform_info = scan_output["smartctl"]["platform_info"]
        .as_str()
        .ok_or_else(|| Error::msg("Invalid platform_info format"))?
        .to_string();
    let build_info = scan_output["smartctl"]["build_info"]
        .as_str()
        .ok_or_else(|| Error::msg("Invalid build_info format"))?
        .to_string();

    Ok(SmartCtlVersionInfo {
        json_format_version: json_version_tuple,
        smartctl_version: smartctl_version_tuple,
        svn_revision,
        platform_info,
        build_info,
    })
}

fn parse_json_scan_output(json: serde_json::Value) -> Result<Vec<String>, Error> {
    let devices = json["devices"]
        .as_array()
        .ok_or_else(|| Error::msg("Invalid devices format"))?;

    let mut device_list = Vec::new();

    for device in devices.iter() {
        let device_name = device["name"]
            .as_str()
            .ok_or_else(|| Error::msg("Invalid device name format"))?
            .to_string();
        device_list.push(device_name);
    }

    Ok(device_list)
}

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

impl SmartCtl {
    fn _new<T>(path: Option<T>) -> Result<SmartCtl, Error>
    where
        T: Into<String>,
    {
        let fallback_logic = || {
            error!("Could not find smartctl binary using `which`. Falling back to invoking `smartctl` in the shell");
            String::from("smartctl")
        };

        // Determine the path if supplied
        let path: String = match path {
            Some(path) => path.into(),
            None => match Command::new("which").arg("smartctl").output() {
                Ok(output) => {
                    let output = String::from_utf8(output.stdout)?;
                    let trimmed = output.trim().to_string();

                    if trimmed.is_empty() {
                        fallback_logic()
                    } else {
                        trimmed
                    }
                }
                Err(_) => fallback_logic(),
            },
        };

        // Try a simple scan output to get the version info stuff up top

        let scan_output = {
            let output = Command::new(&path)
                .arg("-j")
                .arg("--scan")
                .output()
                .map_err(|e| {
                    Error::msg(format!("Failed to execute smartctl ({:?}): {}", path, e))
                })?;
            String::from_utf8(output.stdout)?
        };

        let version_info = parse_version_info(scan_output)?;

        debug!("JSON Version: {:?}", version_info.json_format_version);
        debug!("smartctl Version: {:?}", version_info.smartctl_version);
        debug!("SVN Revision: {}", version_info.svn_revision);
        debug!("Platform Info: {}", version_info.platform_info);
        debug!("Build Info: {}", version_info.build_info);
        debug!("Using path {:?}", path);

        let definition = SmartCtl { path, version_info };

        Ok(definition)
    }

    /// Construct a new wrapper instance for the `smartctl` binary.
    ///
    /// Let the wrapper determine the path to the binary. This
    /// is first done by trying the `which` command to see if the
    /// binary is in the path. If that fails, the wrapper will
    /// fall back to just using `smartctl` in shell and hope it's
    /// in the path at the time of execution.
    pub fn new() -> Result<SmartCtl, Error> {
        Self::_new::<String>(None)
    }

    /// Construct a new wrapper instance for the `smartctl` binary
    /// at the specified path.
    pub fn new_with_path<T>(path: T) -> Result<SmartCtl, Error>
    where
        T: Into<String>,
    {
        Self::_new(Some(path))
    }

    /// Get the version info for the smartctl binary
    pub fn get_version_info(&self) -> &SmartCtlVersionInfo {
        &self.version_info
    }

    /// Return the path being used to the `smartctl` binary
    pub fn get_path(&self) -> String {
        self.path.clone()
    }

    /// Execute the smartctl binary with the supplied arguments. This is the lowest
    /// level wrapper around the binary. There is no parsing of the output.
    ///
    /// # Arguments
    ///
    /// * `args` - An iterable item that contains items that can be turned `Into<String>`
    ///
    /// # Example
    /// ```rust,ignore
    /// use smartctl_wrapper::SmartCtl;
    ///
    /// let smartctl_bin = SmartCtl::new().unwrap();
    ///
    /// let output = smartctl_bin.execute(["-j", "--scan"]).unwrap();
    ///
    /// ```
    pub fn execute<T, S>(&self, args: T) -> Result<String, Error>
    where
        T: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let output = Command::new(&self.path)
            .args(args.into_iter().map(|v| Into::<String>::into(v)))
            .output()
            .map_err(|e| {
                Error::msg(format!(
                    "Failed to execute smartctl ({:?}): {}",
                    self.path, e
                ))
            })?;

        let output = String::from_utf8(output.stdout)?;

        Ok(output)
    }

    /// Execute the smartctl binary with the supplied arguments. This execute function
    /// prepends the `-j` argument to the supplied arguments. The output is parsed
    /// into a `serde_json::Value` object.
    ///
    /// # Arguments
    ///
    /// * `args` - An iterable item that contains items that can be turned `Into<String>`
    ///
    pub fn execute_json<T, S>(&self, args: T) -> Result<serde_json::Value, Error>
    where
        T: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let j_arg = ["-j".to_string()].into_iter();
        let mapped_args = args.into_iter().map(|arg| Into::<String>::into(arg));
        let new_args = j_arg.chain(mapped_args);
        let output = self.execute(new_args)?;

        let output: serde_json::Value = serde_json::from_str(&output)?;

        Ok(output)
    }

    /// Scan for devices using the `smartctl` binary. Equivalent to `smartctl -j --scan`
    /// This function will return a list of device names that can
    /// be used with the `get_device` function.
    pub fn scan(&self) -> Result<Vec<String>, Error> {
        let output = self.execute_json(["--scan"])?;

        parse_json_scan_output(output)
    }

    /// Scan for devices using the `smartctl` binary. Equivalent to `smartctl -j --scan-open`
    /// This function will return a list of device names that can
    /// be used with the `get_device` function.
    pub fn scan_open(&self) -> Result<Vec<String>, Error> {
        let output = self.execute_json(["--scan-open"])?;

        parse_json_scan_output(output)
    }

    /// Get a device using the `smartctl` binary. This function will return a `SmartCtlDevice`
    /// object that can be used to query the device.
    pub fn get_device(&self, device_path: String) -> Result<SmartCtlDevice, Error> {
        let json_response = self.execute_json(["--info".to_string(), device_path])?;

        let form_factor =
            SmartCtlDeviceFormFactor::deserialize(&json_response["device"]["form_factor"]).ok();

        let wwn = SmartCtlWwn::deserialize(&json_response["wwn"]).ok();

        let capacity = SmartCtlCapacityInfo {
            blocks: json_response["user_capacity"]["blocks"]
                .as_u64()
                .ok_or_else(|| Error::msg("Invalid user_capacity/blocks format"))?,
            bytes: json_response["user_capacity"]["bytes"]
                .as_u64()
                .ok_or_else(|| Error::msg("Invalid user_capacity/bytes format"))?,
            logical_block_size: json_response["logical_block_size"]
                .as_u64()
                .ok_or_else(|| Error::msg("Invalid logical_block_size format"))?,
            physical_block_size: json_response["physical_block_size"]
                .as_u64()
                .ok_or_else(|| Error::msg("Invalid physical_block_size format"))?,
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

        Ok(device)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_version_info() {
        let example_output = r#"
        {
            "json_format_version": [
              1,
              0
            ],
            "smartctl": {
              "version": [
                7,
                2
              ],
              "svn_revision": "5155",
              "platform_info": "x86_64-linux-6.0.6-76060006-generic",
              "build_info": "(local build)",
              "argv": [
                "smartctl",
                "--scan",
                "-j"
              ],
              "exit_status": 0
            },
            "devices": [
              {
                "name": "/dev/sda",
                "info_name": "/dev/sda",
                "type": "scsi",
                "protocol": "SCSI"
              },
              {
                "name": "/dev/sdb",
                "info_name": "/dev/sdb",
                "type": "scsi",
                "protocol": "SCSI"
              },
              {
                "name": "/dev/sdc",
                "info_name": "/dev/sdc",
                "type": "scsi",
                "protocol": "SCSI"
              }
            ]
          }          
        "#;

        let version_info = parse_version_info(example_output).unwrap();
        assert!(version_info.smartctl_version == (7, 2));
        assert!(version_info.svn_revision == "5155");
        assert!(version_info.platform_info == "x86_64-linux-6.0.6-76060006-generic");
        assert!(version_info.build_info == "(local build)");
        assert!(version_info.json_format_version == (1, 0));
    }

    #[test]
    fn test_parse_json_scan_output() {
        let example_output = r#"
        {
            "json_format_version": [
              1,
              0
            ],
            "smartctl": {
              "version": [
                7,
                2
              ],
              "svn_revision": "5155",
              "platform_info": "x86_64-linux-6.0.6-76060006-generic",
              "build_info": "(local build)",
              "argv": [
                "smartctl",
                "--scan",
                "-j"
              ],
              "exit_status": 0
            },
            "devices": [
              {
                "name": "/dev/sda",
                "info_name": "/dev/sda",
                "type": "scsi",
                "protocol": "SCSI"
              },
              {
                "name": "/dev/sdb",
                "info_name": "/dev/sdb",
                "type": "scsi",
                "protocol": "SCSI"
              },
              {
                "name": "/dev/sdc",
                "info_name": "/dev/sdc",
                "type": "scsi",
                "protocol": "SCSI"
              }
            ]
          }          
        "#;

        let json_output = serde_json::from_str(example_output).unwrap();

        let scan_output = parse_json_scan_output(json_output).unwrap();
        assert!(scan_output.len() == 3);
    }
}
