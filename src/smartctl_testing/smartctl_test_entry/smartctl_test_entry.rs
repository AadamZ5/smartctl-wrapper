use anyhow::Error;
use serde::{Deserialize, Serialize};

use super::smartctl_test_entry_status_value::SmartCtlTestEntryStatusValue;
use crate::smartctl_dev::parse_json_device_output;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SmartCtlTestEntryType {
    /// The numeric representation of the test type
    value: u8,
    /// The string representation of the test type
    string: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SmartCtlTestEntryStatus {
    /// The numeric representation of the test status
    value: SmartCtlTestEntryStatusValue,
    /// The string representation of the test status
    string: String,
    /// Whether the test passed or encountered an error
    passed: Option<bool>,
}

/// A struct representing a past test entry in the SMART drive.
///
/// This struct does *not* represent a currently running test.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SmartCtlTestEntry {
    #[serde(rename = "type")]
    type_: SmartCtlTestEntryType,
    status: SmartCtlTestEntryStatus,
    lifetime_hours: u64,
}

pub fn parse_json_ata_test_entry(entry: &serde_json::Value) -> Result<SmartCtlTestEntry, Error> {
    // Entry should be an object within the ata_smart_self_test_log.standard.table array

    let deserialized_entry = SmartCtlTestEntry::deserialize(entry)?;

    Ok(deserialized_entry)
}

pub fn parse_json_all_output_test_entries(
    json: serde_json::Value,
) -> Result<Vec<SmartCtlTestEntry>, Error> {
    // First check to see if this is a proper device JSON output
    let _device = parse_json_device_output(&json)?;

    let standard_tests = json
        .get("ata_smart_self_test_log")
        .ok_or(Error::msg(
            "Missing \"ata_smart_self_test_log\" in JSON response!",
        ))?
        .get("standard")
        .ok_or(Error::msg(
            "Missing \"ata_smart_self_test_log.standard\" in JSON response!",
        ))?;

    let standard_tests_revision = standard_tests.get("revision").ok_or(Error::msg(
        "Missing \"ata_smart_self_test_log.standard.revision\" in JSON response!",
    ))?;

    if standard_tests_revision != 1 {
        return Err(Error::msg(
            "Unsupported \"ata_smart_self_test_log.standard.revision\" in JSON response!",
        ));
    }

    let standard_tests_entries = standard_tests
        .get("table")
        .ok_or(Error::msg(
            "Missing \"ata_smart_self_test_log.standard.table\" in JSON response!",
        ))?
        .as_array()
        .ok_or(Error::msg(
            "Invalid \"ata_smart_self_test_log.standard.table\" in JSON response!",
        ))?;

    let parsed_test_entries = standard_tests_entries
        .iter()
        .map(|entry| parse_json_ata_test_entry(entry))
        .collect::<Result<Vec<SmartCtlTestEntry>, Error>>()?;

    Ok(parsed_test_entries)
}

#[cfg(test)]
mod tests {

    use std::{collections::HashSet, panic};

    use super::*;
    use crate::test_util::example_outputs::{
        output_pieces, EXAMPLE_ALL, EXAMPLE_ALL_DURING_TESTING,
    };

    #[test]
    fn test_parse_json_ata_test_entry() {
        let example_entries = output_pieces::TEST_ENTRY;

        for output in example_entries.iter() {
            let entry = parse_json_ata_test_entry(&serde_json::from_str(output).unwrap()).unwrap();

            let test_entry_json: serde_json::Value = serde_json::from_str(output).unwrap();

            assert_eq!(
                entry.type_.value,
                test_entry_json
                    .get("type")
                    .and_then(|v| v.get("value"))
                    .and_then(|v| v.as_u64())
                    .unwrap() as u8
            );
            assert_eq!(
                entry.type_.string,
                test_entry_json
                    .get("type")
                    .and_then(|v| v.get("string"))
                    .and_then(|v| v.as_str())
                    .unwrap()
                    .to_string()
            );
            assert_eq!(
                entry.status.value,
                SmartCtlTestEntryStatusValue::try_from(
                    test_entry_json
                        .get("status")
                        .and_then(|v| v.get("value"))
                        .and_then(|v| v.as_u64())
                        .unwrap() as u8
                )
                .unwrap()
            );
            assert_eq!(
                entry.status.string,
                test_entry_json
                    .get("status")
                    .and_then(|v| v.get("string"))
                    .and_then(|v| v.as_str())
                    .unwrap()
                    .to_string()
            );
            assert_eq!(
                entry.status.passed,
                test_entry_json
                    .get("status")
                    .and_then(|v| v.get("passed"))
                    .and_then(|v| v.as_bool())
            );
            assert_eq!(
                entry.lifetime_hours,
                test_entry_json
                    .get("lifetime_hours")
                    .and_then(|v| v.as_u64())
                    .unwrap()
            );
        }
    }

    #[test]
    fn test_no_unused_keys_in_test_data() {
        let example_entries = output_pieces::TEST_ENTRY;

        for (i, output) in example_entries.iter().enumerate() {
            let deserializer = &mut serde_json::Deserializer::from_str(output);

            let mut unused_keys = HashSet::new();

            let _entry: SmartCtlTestEntry =
                serde_ignored::deserialize(deserializer, |unused_path| {
                    unused_keys.insert(unused_path.to_string());
                })
                .unwrap();

            if unused_keys.len() > 0 {
                panic!(
                    "Unused keys in test data at index {i}! Unused keys: {:?}",
                    unused_keys
                );
            }
        }
    }

    #[test]
    fn test_parse_json_all_output() {
        let example_outputs = EXAMPLE_ALL.iter().chain(EXAMPLE_ALL_DURING_TESTING.iter());

        // Helper function to get the test entry by index from
        // the serde_json object
        let get_test_entry_from_json =
            |json: &serde_json::Value, index: u8| -> Option<serde_json::Value> {
                let standard_test_table = json
                    .get("ata_smart_self_test_log")
                    .and_then(|v| v.get("standard"))
                    .and_then(|v| v.get("table"))
                    .and_then(|v| v.as_array())
                    .and_then(|v| v.get(index as usize));

                standard_test_table.cloned()
            };

        // Check all supplied outputs for correctly parsing test entries
        for output in example_outputs {
            let json_output: serde_json::Value = serde_json::from_str(output).unwrap();

            let parsed_entries = parse_json_all_output_test_entries(json_output.clone()).unwrap();

            for (i, entry) in parsed_entries.iter().enumerate() {
                let test_entry_json = get_test_entry_from_json(&json_output, i as u8).unwrap();

                assert_eq!(
                    entry.type_.value,
                    test_entry_json
                        .get("type")
                        .and_then(|v| v.get("value"))
                        .and_then(|v| v.as_u64())
                        .unwrap() as u8
                );
                assert_eq!(
                    entry.type_.string,
                    test_entry_json
                        .get("type")
                        .and_then(|v| v.get("string"))
                        .and_then(|v| v.as_str())
                        .unwrap()
                        .to_string()
                );
                assert_eq!(
                    entry.status.value,
                    SmartCtlTestEntryStatusValue::try_from(
                        test_entry_json
                            .get("status")
                            .and_then(|v| v.get("value"))
                            .and_then(|v| v.as_u64())
                            .unwrap() as u8
                    )
                    .unwrap()
                );
                assert_eq!(
                    entry.status.string,
                    test_entry_json
                        .get("status")
                        .and_then(|v| v.get("string"))
                        .and_then(|v| v.as_str())
                        .unwrap()
                        .to_string()
                );
                assert_eq!(
                    entry.status.passed,
                    test_entry_json
                        .get("status")
                        .and_then(|v| v.get("passed"))
                        .and_then(|v| v.as_bool())
                );
                assert_eq!(
                    entry.lifetime_hours,
                    test_entry_json
                        .get("lifetime_hours")
                        .and_then(|v| v.as_u64())
                        .unwrap()
                );
            }
        }
    }
}
