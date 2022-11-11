use anyhow::Error;
use serde::{Deserialize, Serialize};

use crate::smartctl_bin::SmartCtl;

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
}

impl SmartCtlDevice {
    pub fn new(path: String, smart_ctl: Option<SmartCtl>) -> Result<Self, Error> {
        let bin_instance = smart_ctl.unwrap_or(SmartCtl::new(None)?);

        let dev = bin_instance.get_device(path)?;

        Ok(dev)
    }
}
