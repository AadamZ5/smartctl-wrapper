//! This module contains the structs and functions for parsing and representing
//! smartctl test entries. Test entries are logs of **past** tests. Currently
//! running tests are not included in test entries.

mod smartctl_test_entry_defintion;
mod smartctl_test_entry_status_value;

pub use smartctl_test_entry_defintion::{
    parse_json_all_output_test_entries, parse_json_ata_test_entry, SmartCtlTestEntry,
    SmartCtlTestEntryStatus, SmartCtlTestEntryType,
};
pub use smartctl_test_entry_status_value::SmartCtlTestEntryStatusValue;
