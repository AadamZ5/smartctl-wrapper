use anyhow::Error;
use serde::{Deserialize, Serialize};

use crate::{
    smartctl_bin::SmartCtl,
    smartctl_testing::{
        smartctl_test::parse_json_ata_smart_data_to_self_test,
        smartctl_test_entry::parse_json_all_output_test_entries,
    },
    SmartCtlSelfTest, SmartCtlTestEntry,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartCtlWwn {
    pub naa: u64,
    pub oui: u64,
    pub id: u64,
}

#[derive(Debug, Clone)]
pub struct SmartCtlCapacityInfo {
    pub blocks: u64,
    pub bytes: u64,
    pub logical_block_size: u64,
    pub physical_block_size: u64,
}

impl SmartCtlCapacityInfo {
    pub fn get_size_kb(&self) -> f32 {
        (self.bytes as f32) / 1024.0
    }

    pub fn get_size_mb(&self) -> f32 {
        //Take bytes and divide by 1,048,576 to get MB
        (self.bytes as f32) / 1048576.0
    }

    pub fn get_size_gb(&self) -> f32 {
        //Take bytes and divide by 1,073,741,824 to get GB
        (self.bytes as f32) / 1073741824.0
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartCtlDeviceFormFactor {
    pub name: String,
    pub ata_value: u8,
}

#[derive(Debug, Clone)]
pub struct SmartCtlDeviceHealth {}

#[derive(Debug, Clone)]
pub struct SmartCtlDevice {
    pub name: String,
    pub info_name: String,
    pub type_: String,
    pub protocol: String,

    pub model_family: Option<String>,
    pub model_name: Option<String>,
    pub serial_number: Option<String>,
    pub firmware_version: Option<String>,

    pub wwn: Option<SmartCtlWwn>,

    pub capacity: SmartCtlCapacityInfo,

    pub rotation_rate: Option<u64>,

    pub form_factor: Option<SmartCtlDeviceFormFactor>,

    pub device_health: SmartCtlDeviceHealth,

    bin_instance: SmartCtl,
}

impl SmartCtlDevice {
    pub fn new(path: String, smart_ctl: Option<SmartCtl>) -> Result<Self, Error> {
        let bin_instance = smart_ctl.unwrap_or(SmartCtl::new()?);

        let dev = bin_instance.get_device(path)?;

        Ok(dev)
    }

    pub fn get_test_entries(&self) -> Result<Vec<SmartCtlTestEntry>, Error> {
        let all_json = self
            .bin_instance
            .execute_json(["-all", self.name.as_str()])?;

        parse_json_all_output_test_entries(&all_json)
    }

    /// Returns a list of supported test types and their estimated time in minutes reported
    /// from `smartctl`. This is found in the `ata_smart_data.self_test.polling_minutes`
    /// section of the JSON output.
    pub fn get_supported_test_types_and_minutes(&self) -> Result<Vec<(String, u64)>, Error> {
        let all_json = self
            .bin_instance
            .execute_json(["--all", self.name.as_str()])?;

        parse_json_ata_smart_data_to_self_test(&all_json).and_then(|v| v.get_test_types())
    }

    pub fn start_self_test<T>(&self, test_type: T) -> Result<SmartCtlSelfTest, Error>
    where
        T: Into<String>,
    {
        // First check if the test type is one supported by this drive
        let mut supported_test_types = self
            .get_supported_test_types_and_minutes()?
            .into_iter()
            .map(|(t, _)| t);

        // TODO: Add a list of tests that all drives support but don't report
        // TODO: (if any)

        let test_type_ref = &Into::<String>::into(test_type);

        if !supported_test_types.any(|t| &t == test_type_ref) {
            return Err(Error::msg(format!(
                "Test type {} is not supported by this drive",
                test_type_ref
            )));
        }

        self.bin_instance
            .execute(["-t", test_type_ref.as_str(), self.name.as_str()])?;

        let all_json_output = self
            .bin_instance
            .execute_json(["--all", self.name.as_str()])?;
        let self_test = parse_json_ata_smart_data_to_self_test(&all_json_output)?;

        Ok(self_test)
    }
}

pub fn parse_json_device_output(
    json: &serde_json::Value,
    bin_instance_to_supply: Option<SmartCtl>,
) -> Result<SmartCtlDevice, Error> {
    let form_factor = SmartCtlDeviceFormFactor::deserialize(&json["form_factor"]).ok();

    let wwn = SmartCtlWwn::deserialize(&json["wwn"]).ok();

    let capacity = SmartCtlCapacityInfo {
        blocks: json["user_capacity"]["blocks"]
            .as_u64()
            .ok_or_else(|| Error::msg("Invalid user_capacity/blocks format"))?,
        bytes: json["user_capacity"]["bytes"]
            .as_u64()
            .ok_or_else(|| Error::msg("Invalid user_capacity/bytes format"))?,
        logical_block_size: json["logical_block_size"]
            .as_u64()
            .ok_or_else(|| Error::msg("Invalid logical_block_size format"))?,
        physical_block_size: json["physical_block_size"]
            .as_u64()
            .ok_or_else(|| Error::msg("Invalid physical_block_size format"))?,
    };

    let device = SmartCtlDevice {
        name: json["device"]["name"]
            .as_str()
            .unwrap_or("Unknown")
            .to_string(),
        info_name: json["device"]["info_name"]
            .as_str()
            .unwrap_or("Unknown")
            .to_string(),
        type_: json["device"]["type"]
            .as_str()
            .unwrap_or("Unknown")
            .to_string(),
        protocol: json["device"]["protocol"]
            .as_str()
            .unwrap_or("Unknown")
            .to_string(),

        model_family: json["model_family"].as_str().map(|s| s.to_string()),
        model_name: json["model_name"].as_str().map(|s| s.to_string()),
        serial_number: json["serial_number"].as_str().map(|s| s.to_string()),
        firmware_version: json["firmware_version"].as_str().map(|s| s.to_string()),
        wwn,
        capacity,
        rotation_rate: json["rotation_rate"].as_u64(),

        form_factor,

        device_health: SmartCtlDeviceHealth {},

        bin_instance: bin_instance_to_supply.unwrap_or(SmartCtl::new()?),
    };

    Ok(device)
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::test_util::example_outputs::EXAMPLE_INFO;

    #[test]
    fn test_parse_json_device_output() {
        let example_outputs = EXAMPLE_INFO;

        for output in example_outputs {
            let json_output = serde_json::from_str(output).unwrap();

            let device_output = parse_json_device_output(&json_output, None).unwrap();

            assert_eq!(
                device_output.model_family.unwrap(),
                json_output
                    .get("model_family")
                    .and_then(|v| v.as_str())
                    .unwrap()
            );
            assert_eq!(
                device_output.model_name.unwrap(),
                json_output
                    .get("model_name")
                    .and_then(|v| v.as_str())
                    .unwrap()
            );
            assert_eq!(
                device_output.serial_number.unwrap(),
                json_output
                    .get("serial_number")
                    .and_then(|v| v.as_str())
                    .unwrap()
            );
            assert_eq!(
                device_output.wwn.is_some(),
                json_output.get("wwn").is_some()
            );
            if let Some(wwn) = device_output.wwn {
                assert_eq!(
                    wwn.naa,
                    json_output
                        .get("wwn")
                        .and_then(|v| v.get("naa"))
                        .and_then(|v| v.as_u64())
                        .unwrap()
                );
                assert_eq!(
                    wwn.oui,
                    json_output
                        .get("wwn")
                        .and_then(|v| v.get("oui"))
                        .and_then(|v| v.as_u64())
                        .unwrap()
                );
                assert_eq!(
                    wwn.id,
                    json_output
                        .get("wwn")
                        .and_then(|v| v.get("id"))
                        .and_then(|v| v.as_u64())
                        .unwrap()
                );
            }
            assert_eq!(
                device_output.firmware_version,
                json_output
                    .get("firmware_version")
                    .and_then(|v| v.as_str())
                    .map(|v| v.to_string())
            );
            assert_eq!(
                device_output.capacity.blocks,
                json_output
                    .get("user_capacity")
                    .and_then(|v| v.get("blocks"))
                    .and_then(|v| v.as_u64())
                    .unwrap()
            );
            assert_eq!(
                device_output.capacity.bytes,
                json_output
                    .get("user_capacity")
                    .and_then(|v| v.get("bytes"))
                    .and_then(|v| v.as_u64())
                    .unwrap()
            );
            assert_eq!(
                device_output.capacity.logical_block_size,
                json_output
                    .get("logical_block_size")
                    .and_then(|v| v.as_u64())
                    .unwrap()
            );
            assert_eq!(
                device_output.capacity.physical_block_size,
                json_output
                    .get("physical_block_size")
                    .and_then(|v| v.as_u64())
                    .unwrap()
            );
            assert_eq!(
                device_output.rotation_rate,
                json_output.get("rotation_rate").and_then(|v| v.as_u64())
            );

            assert_eq!(
                device_output.form_factor.is_some(),
                json_output.get("form_factor").is_some()
            );
            if let Some(form_factor) = device_output.form_factor {
                assert_eq!(
                    form_factor.ata_value,
                    json_output
                        .get("form_factor")
                        .and_then(|v| v.get("ata_value"))
                        .and_then(|v| v.as_i64())
                        .unwrap() as u8
                );
                assert_eq!(
                    form_factor.name,
                    json_output
                        .get("form_factor")
                        .and_then(|v| v.get("name"))
                        .and_then(|v| v.as_str())
                        .unwrap()
                );
            }
        }
    }
}
