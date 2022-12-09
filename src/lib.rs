//! A thin wrapper around the `smartctl` binary from the [smartmontools](https://www.smartmontools.org/) project.
//!
//! # Requirements
//! - This crate depends on the `-j` JSON output option of `smartctl` being available.
//! This is available in smartmontools 7 and later.
//! - This crate depends on the `smartctl` binary being available on the system.
//! - The program that uses this crate must have necessary permisssions to execute
//! `smartctl`. `smartctl` needs kernel-level access to devices.
//!
//! # Examples
//!
//! ## Get a list of all S.M.A.R.T. devices
//! ```rust,ignore
//! use smartctl_wrapper::SmartCtl;
//!
//! let smartctl = SmartCtl::new().unwrap();
//! let devices = smartctl.scan().unwrap();
//!
//! println!("Devices:\n {}", devices.join("\n"));
//! ```

mod smartctl_bin;
mod smartctl_dev;
mod smartctl_testing;
mod test_util;

pub use smartctl_bin::SmartCtl;
pub use smartctl_dev::{
    SmartCtlCapacityInfo, SmartCtlDevice, SmartCtlDeviceFormFactor, SmartCtlDeviceHealth,
    SmartCtlWwn,
};
pub use smartctl_testing::smartctl_test::{
    SmartCtlSelfTest, SmartCtlSelfTestPolling, SmartCtlSelfTestStatus,
};
pub use smartctl_testing::smartctl_test_entry::{
    smartctl_test_entry::{SmartCtlTestEntry, SmartCtlTestEntryStatus, SmartCtlTestEntryType},
    smartctl_test_entry_status_value::SmartCtlTestEntryStatusValue,
};

#[cfg(test)]
mod tests {

    extern crate pretty_env_logger;
    use super::*;
    use log::{debug, info};
    use std::process::Command;
    use std::sync::Once;

    static INIT: Once = Once::new();

    fn init() {
        INIT.call_once(|| {
            let _ = pretty_env_logger::formatted_builder()
                .is_test(true)
                .filter_level(log::LevelFilter::Trace)
                .try_init();
        });
    }

    #[test]
    #[ignore]
    fn no_path_supplied() {
        init();
        let result = SmartCtl::new();
        assert!(result.is_ok());
    }

    #[test]
    #[ignore]
    fn with_path_supplied() {
        init();
        let path = String::from_utf8(
            Command::new("which")
                .arg("smartctl")
                .output()
                .unwrap()
                .stdout,
        )
        .unwrap()
        .trim()
        .to_string();
        debug!("Using path {:?}", path);
        let result = SmartCtl::new_with_path(path);
        assert!(result.is_ok());
        let smartctl = result.unwrap();
        info!("Smartctl version: {}", smartctl.get_version_info())
    }

    #[test]
    #[ignore]
    fn list_devices() {
        init();
        let smartctl = SmartCtl::new().unwrap();
        let devices = smartctl.scan().unwrap();
        for device in devices {
            info!("Device: {:?}", device);
        }
    }

    #[test]
    #[ignore]
    fn get_device() {
        init();
        let smartctl = SmartCtl::new().unwrap();
        let device = smartctl.get_device("/dev/sda".to_string()).unwrap();
        info!("Device: {:?}", device);
    }
}
