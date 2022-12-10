use anyhow::Error;
use serde::{de::Visitor, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum SmartCtlTestEntryStatusValue {
    Passed = 0,
    Failed = 1,
    Aborted = 16,
}

impl TryFrom<u8> for SmartCtlTestEntryStatusValue {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        // We can not simply cast from u8 to the enum, since a value
        // that is not present in the enum will cause undefined behavior 😱

        // So make sure to keep this in sync with the enum definition!

        match value {
            0 => Ok(SmartCtlTestEntryStatusValue::Passed),
            1 => Ok(SmartCtlTestEntryStatusValue::Failed),
            16 => Ok(SmartCtlTestEntryStatusValue::Aborted),
            _ => Err(Error::msg(format!(
                "Invalid test entry status value: {}",
                value
            ))),
        }
    }
}

impl TryFrom<i8> for SmartCtlTestEntryStatusValue {
    type Error = Error;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        // We can not simply cast from i8 to the enum, since a value
        // that is not present in the enum will cause undefined behavior 😱

        // So make sure to keep this in sync with the enum definition!

        match value {
            0 => Ok(SmartCtlTestEntryStatusValue::Passed),
            1 => Ok(SmartCtlTestEntryStatusValue::Failed),
            16 => Ok(SmartCtlTestEntryStatusValue::Aborted),
            _ => Err(Error::msg(format!(
                "Invalid test entry status value: {}",
                value
            ))),
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<u8> for SmartCtlTestEntryStatusValue {
    fn into(self) -> u8 {
        // We can simple cast the enum to a u8 since the repr is set to u8,
        // and since we already know an existing enum value was valid.
        self as u8
    }
}

impl<'de> Deserialize<'de> for SmartCtlTestEntryStatusValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct SmartCtlTestEntryStatusValueVisitor;

        impl<'de> Visitor<'de> for SmartCtlTestEntryStatusValueVisitor {
            type Value = SmartCtlTestEntryStatusValue;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a u8 representing a test entry status value")
            }

            fn visit_u8<E>(self, value: u8) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                SmartCtlTestEntryStatusValue::try_from(value).map_err(|_| {
                    serde::de::Error::invalid_value(
                        serde::de::Unexpected::Unsigned(value as u64),
                        &self,
                    )
                })
            }

            fn visit_i8<E>(self, value: i8) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                SmartCtlTestEntryStatusValue::try_from(value).map_err(|_| {
                    serde::de::Error::invalid_value(
                        serde::de::Unexpected::Signed(value as i64),
                        &self,
                    )
                })
            }

            // Serde assumes all JSON values are i64! We have to try and cast down to i8.
            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let cast_value: i8 = value.try_into().map_err(|_| {
                    serde::de::Error::invalid_value(serde::de::Unexpected::Signed(value), &self)
                })?;

                self.visit_i8(cast_value)
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let cast_value: u8 = value.try_into().map_err(|_| {
                    serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(value), &self)
                })?;

                self.visit_u8(cast_value)
            }
        }

        deserializer.deserialize_u8(SmartCtlTestEntryStatusValueVisitor {})
    }
}

impl Serialize for SmartCtlTestEntryStatusValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u8(self.clone() as u8)
    }
}

#[cfg(test)]
mod tests {

    // Not sure if we need these tests since we've landed
    // on a successful implementation of deserialize/serialize,
    // but perhaps this will cover other cases that we haven't
    // thought of yet.

    use super::*;

    #[test]
    fn test_smartctl_test_entry_status_value_try_from() {
        assert_eq!(
            SmartCtlTestEntryStatusValue::try_from(0 as u8).unwrap(),
            SmartCtlTestEntryStatusValue::Passed
        );
        assert_eq!(
            SmartCtlTestEntryStatusValue::try_from(1 as u8).unwrap(),
            SmartCtlTestEntryStatusValue::Failed
        );
        assert_eq!(
            SmartCtlTestEntryStatusValue::try_from(16 as u8).unwrap(),
            SmartCtlTestEntryStatusValue::Aborted
        );
        assert!(SmartCtlTestEntryStatusValue::try_from(2 as u8).is_err());

        assert_eq!(
            SmartCtlTestEntryStatusValue::try_from(0 as i8).unwrap(),
            SmartCtlTestEntryStatusValue::Passed
        );
        assert_eq!(
            SmartCtlTestEntryStatusValue::try_from(1 as i8).unwrap(),
            SmartCtlTestEntryStatusValue::Failed
        );
        assert_eq!(
            SmartCtlTestEntryStatusValue::try_from(16 as i8).unwrap(),
            SmartCtlTestEntryStatusValue::Aborted
        );
        assert!(SmartCtlTestEntryStatusValue::try_from(2 as i8).is_err());
    }

    #[test]
    fn serializing_smartctl_test_entry_status_value() {
        let value = SmartCtlTestEntryStatusValue::Passed;
        let serialized = serde_json::to_string(&value).unwrap();
        assert_eq!(serialized, "0");
    }

    #[test]
    fn deserializing_smartctl_test_entry_status_value() {
        let value = SmartCtlTestEntryStatusValue::Passed;
        let deserialized: SmartCtlTestEntryStatusValue = serde_json::from_str("0").unwrap();
        assert_eq!(deserialized, value);
    }
}
