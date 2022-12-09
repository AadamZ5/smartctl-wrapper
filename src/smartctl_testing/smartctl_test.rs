//! This module contains the structs and functions for parsing currently running
//! tests.

use anyhow::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SmartCtlSelfTestStatus {
    value: u8,
    string: String,
    remaining_percent: Option<u8>,
    passed: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SmartCtlSelfTestPolling {
    short: u8,
    extended: u8,
}

/// A struct representing a currently running test on the SMART drive.
///
/// This struct does *not* represent any past tests.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SmartCtlSelfTest {
    status: SmartCtlSelfTestStatus,
    polling_minutes: SmartCtlSelfTestPolling,
}

pub fn parse_json_ata_smart_data(
    ata_smart_data: &serde_json::Value,
) -> Result<SmartCtlSelfTest, Error> {
    // json should be the "ata_smart_data" field of the json output

    let self_test = ata_smart_data
        .get("self_test")
        .ok_or_else(|| Error::msg("Missing self_test field"))?;

    Ok(SmartCtlSelfTest::deserialize(self_test)?)
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::test_util::example_outputs::{EXAMPLE_ALL, EXAMPLE_ALL_DURING_TESTING};

    #[test]
    fn test_parse_json_ata_smart_data() {
        let example_outputs = EXAMPLE_ALL_DURING_TESTING.iter().chain(EXAMPLE_ALL.iter());

        for output in example_outputs {
            let json: serde_json::Value = serde_json::from_str(output).unwrap();

            let ata_smart_data = json.get("ata_smart_data").unwrap();

            let self_test = parse_json_ata_smart_data(ata_smart_data).unwrap();

            println!("{}", output);

            let self_test_actual = ata_smart_data.get("self_test").unwrap();

            // Is this test meaningful? I guess it checks to make sure
            // our structs are holding the right data we want it to.
            // Serde's deserialize should be doing this for us, but
            // I guess it's good to have a test to make sure it's working.

            assert_eq!(
                self_test.status.value,
                self_test_actual
                    .get("status")
                    .and_then(|v| v.get("value"))
                    .and_then(|v| v.as_u64())
                    .unwrap() as u8
            );
            assert_eq!(
                self_test.status.string,
                self_test_actual
                    .get("status")
                    .and_then(|v| v.get("string"))
                    .and_then(|v| v.as_str())
                    .unwrap()
                    .to_string()
            );
            assert_eq!(
                self_test.status.remaining_percent,
                self_test_actual
                    .get("status")
                    .and_then(|v| v.get("remaining_percent"))
                    .and_then(|v| v.as_u64())
                    .map(|v| v as u8)
            );
            assert_eq!(
                self_test.status.passed,
                self_test_actual
                    .get("status")
                    .and_then(|v| v.get("passed"))
                    .and_then(|v| v.as_bool())
            );
            assert_eq!(
                self_test.polling_minutes.short,
                self_test_actual
                    .get("polling_minutes")
                    .and_then(|v| v.get("short"))
                    .and_then(|v| v.as_u64())
                    .unwrap() as u8
            );
            assert_eq!(
                self_test.polling_minutes.extended,
                self_test_actual
                    .get("polling_minutes")
                    .and_then(|v| v.get("extended"))
                    .and_then(|v| v.as_u64())
                    .unwrap() as u8
            );
        }
    }
}
