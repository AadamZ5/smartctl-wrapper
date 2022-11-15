mod smartctl_bin;
mod smartctl_dev;
mod test_util;

pub use smartctl_bin::SmartCtl;
pub use smartctl_dev::{
    SmartCtlCapacityInfo, SmartCtlDevice, SmartCtlDeviceFormFactor, SmartCtlDeviceHealth,
    SmartCtlWwn,
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
    fn no_path_supplied() {
        init();
        let result = SmartCtl::new(None);
        assert!(result.is_ok());
    }

    #[test]
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
        let result = SmartCtl::new(Some(path));
        assert!(result.is_ok());
        info!("Smartctl version: {}", result.unwrap().get_version_info())
    }

    #[test]
    fn list_devices() {
        init();
        let smartctl = SmartCtl::new(None).unwrap();
        let devices = smartctl.scan().unwrap();
        for device in devices {
            info!("Device: {:?}", device);
        }
    }

    #[test]
    fn get_device() {
        init();
        let smartctl = SmartCtl::new(None).unwrap();
        let device = smartctl.get_device("/dev/sda".to_string()).unwrap();
        info!("Device: {:?}", device);
    }
}
