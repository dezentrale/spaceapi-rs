//! Module defining all sensor related structures.

use log::warn;
use serde::{Deserialize, Serialize};
use std::convert::TryInto;

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

impl TryInto<LocalisedSensorMetadata> for SensorMetadata {
    type Error = &'static str;

    fn try_into(self) -> Result<LocalisedSensorMetadata, Self::Error> {
        match self.location {
            Some(location) => Ok(LocalisedSensorMetadata {
                name: self.name,
                location,
                description: self.description,
            }),
            None => Err("No location specified when one is required"),
        }
    }
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

//--- Templates ---//
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

#[derive(Debug, Clone)]
pub struct PeopleNowPresentSensorTemplate {
    pub metadata: SensorMetadata,
    pub names: Option<Vec<String>>,
}

impl Into<PeopleNowPresentSensor> for PeopleNowPresentSensorTemplate {
    fn into(self) -> PeopleNowPresentSensor {
        PeopleNowPresentSensor {
            metadata: self.metadata,
            ..PeopleNowPresentSensor::default()
        }
    }
}

impl SensorTemplate for PeopleNowPresentSensorTemplate {
    fn try_to_sensor(
        &self,
        value_str: &str,
        sensors: &mut Sensors,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut sensor: PeopleNowPresentSensor = self.clone().into();
        sensor.value = value_str.parse::<u64>()?;
        sensors.people_now_present.push(sensor);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct TemperatureSensorTemplate {
    pub metadata: SensorMetadata,
    pub unit: String,
}

impl TryInto<TemperatureSensor> for TemperatureSensorTemplate {
    type Error = Box<dyn std::error::Error>;

    fn try_into(self) -> Result<TemperatureSensor, Self::Error> {
        Ok(TemperatureSensor {
            metadata: self.metadata.try_into()?,
            unit: self.unit,
            ..TemperatureSensor::default()
        })
    }
}

impl SensorTemplate for TemperatureSensorTemplate {
    fn try_to_sensor(
        &self,
        value_str: &str,
        sensors: &mut Sensors,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut sensor: TemperatureSensor = self.clone().try_into()?;
        sensor.value = value_str.parse::<f64>()?;
        sensors.temperature.push(sensor);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct HumiditySensorTemplate {
    pub metadata: SensorMetadata,
    pub unit: String,
}

impl TryInto<HumiditySensor> for HumiditySensorTemplate {
    type Error = Box<dyn std::error::Error>;

    fn try_into(self) -> Result<HumiditySensor, Self::Error> {
        Ok(HumiditySensor {
            metadata: self.metadata.try_into()?,
            unit: self.unit,
            ..HumiditySensor::default()
        })
    }
}

impl SensorTemplate for HumiditySensorTemplate {
    fn try_to_sensor(
        &self,
        value_str: &str,
        sensors: &mut Sensors,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut sensor: HumiditySensor = self.clone().try_into()?;
        sensor.value = value_str.parse::<f64>()?;
        sensors.humidity.push(sensor);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct PowerConsumptionSensorTemplate {
    pub metadata: SensorMetadata,
    pub unit: String,
}

impl TryInto<PowerConsumptionSensor> for PowerConsumptionSensorTemplate {
    type Error = Box<dyn std::error::Error>;

    fn try_into(self) -> Result<PowerConsumptionSensor, Self::Error> {
        Ok(PowerConsumptionSensor {
            metadata: self.metadata.try_into()?,
            unit: self.unit,
            ..PowerConsumptionSensor::default()
        })
    }
}

impl SensorTemplate for PowerConsumptionSensorTemplate {
    fn try_to_sensor(
        &self,
        value_str: &str,
        sensors: &mut Sensors,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut sensor: PowerConsumptionSensor = self.clone().try_into()?;
        sensor.value = value_str.parse::<f64>()?;
        sensors.power_consumption.push(sensor);
        Ok(())
    }
}

//--- Structures ---//

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct PeopleNowPresentSensor {
    #[serde(flatten)]
    pub metadata: SensorMetadata,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<String>>,
    pub value: u64,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct TemperatureSensor {
    #[serde(flatten)]
    pub metadata: LocalisedSensorMetadata,
    pub unit: String,
    pub value: f64,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct HumiditySensor {
    #[serde(flatten)]
    pub metadata: LocalisedSensorMetadata,
    pub unit: String,
    pub value: f64,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct PowerConsumptionSensor {
    #[serde(flatten)]
    pub metadata: LocalisedSensorMetadata,
    pub unit: String,
    pub value: f64,
}

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
