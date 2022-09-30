//! Module defining common sensor functionality.

mod humidity;
mod people_now_present;
mod power_consumption;
mod temperature;

pub use humidity::{HumiditySensor, HumiditySensorTemplate};
pub use people_now_present::{PeopleNowPresentSensor, PeopleNowPresentSensorTemplate};
pub use power_consumption::{PowerConsumptionSensor, PowerConsumptionSensorTemplate};
pub use temperature::{TemperatureSensor, TemperatureSensorTemplate};

use log::warn;
use serde::{Deserialize, Serialize};

/// Common information describing any sensor.
#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct SensorMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Common information describing any sensor which requires a specified location.
#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct LocalisedSensorMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub location: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// A trait for all possible sensor templates.
///
/// A sensor template is like a sensor struct, but without the actual data in it.
/// A `SensorTemplate` is capable of registering itself in a `Sensors` struct.
pub trait SensorTemplate: Send + Sync {
    fn to_sensor(&self, value_str: &str, sensors: &mut Sensors) {
        if let Err(e) = self.try_to_sensor(value_str, sensors) {
            warn!("Omitting sensor. Reason: {}", e);
        }
    }

    fn try_to_sensor(&self, value_str: &str, sensors: &mut Sensors)
        -> Result<(), Box<dyn std::error::Error>>;
}

/// Container for instances of all sensor types.
#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct Sensors {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub people_now_present: Vec<PeopleNowPresentSensor>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub temperature: Vec<TemperatureSensor>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub humidity: Vec<HumiditySensor>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub power_consumption: Vec<PowerConsumptionSensor>,
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::{from_str, to_string};

    #[test]
    fn serialize_deserialize_sensors() {
        let a = Sensors {
            people_now_present: vec![],
            temperature: vec![],
            humidity: vec![],
            power_consumption: vec![],
        };
        let b: Sensors = from_str(&to_string(&a).unwrap()).unwrap();
        assert_eq!(a, b);
    }
}
