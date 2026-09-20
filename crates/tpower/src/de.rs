use std::ops::Deref;

use serde::{Deserialize, Serialize};

macro_rules! with_repr {
    ($(
        #[out, $($out:meta),*]
        #[repr, $repr:meta]
        #[$($meta:meta),*]
        $item:item
    )*) => {
        $(
            $(#[$meta])*
            $(#[$out])*
            $item
        )*

        pub mod repr {
            use super::*;
            $(
                $(#[$meta])*
                #[$repr]
                $item
            )*
        }
    };
}

with_repr! {
    #[out, serde(rename_all = "camelCase"), cfg_attr(feature = "specta", derive(specta::Type))]
    #[repr, serde(rename_all(deserialize = "PascalCase", serialize = "camelCase"))]
    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct IORegistryDiagnostic {
        pub diagnostics: Diagnostics,
    }

    #[out, serde(rename_all = "camelCase"), cfg_attr(feature = "specta", derive(specta::Type))]
    #[repr, serde(rename_all(deserialize = "PascalCase", serialize = "camelCase"))]
    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct Diagnostics {
        #[serde(rename = "IORegistry")]
        pub ioregistry: IORegistry,
    }

    #[out, serde(rename_all = "camelCase"), cfg_attr(feature = "specta", derive(specta::Type))]
    #[repr, serde(default, rename_all(deserialize = "PascalCase", serialize = "camelCase"))]
    #[derive(Debug, Clone, Default, Deserialize, Serialize)]
    pub struct AdapterDetails {
        pub adapter_voltage: Option<i32>,
        pub is_wireless: Option<bool>,
        pub watts: Option<i32>,
        pub name: Option<String>,
        pub current: Option<i32>,
        pub description: Option<String>,
        // Negotiated USB-C PD tier. Useful for explaining why a high-wattage
        // charger is delivering less than its rating.
        pub adapter_power_tier: Option<i32>,
        // Ceilings the adapter advertises, as opposed to what is in use now.
        pub max_voltage: Option<i32>,
        pub max_current: Option<i32>,
    }


    #[out, serde(rename_all = "camelCase"), cfg_attr(feature = "specta", derive(specta::Type))]
    #[repr, serde(default, rename_all(deserialize = "PascalCase", serialize = "camelCase"))]
    #[derive(Debug, Clone, Default, Deserialize, Serialize)]
    pub struct PowerTelemetryData {
        pub adapter_efficiency_loss: i32,
        pub battery_power: i64,
        pub system_current_in: i32,
        pub system_energy_consumed: i64,
        pub system_load: i64,
        pub system_power_in: i32,
        pub system_voltage_in: i32,
    }

    #[out, serde(rename_all = "camelCase"), cfg_attr(feature = "specta", derive(specta::Type))]
    #[repr, serde(default, rename_all(deserialize = "PascalCase", serialize = "camelCase"))]
    #[derive(Debug, Clone, Default, Deserialize, Serialize)]
    pub struct BatteryData {
        /// Full charge capacity in mAh (macOS 27+ replaces AppleRawMaxCapacity with this).
        #[serde(default)]
        pub full_charge_capacity: i32,
        /// Remaining capacity in mAh (macOS 27+ replaces AppleRawCurrentCapacity with this).
        #[serde(default)]
        pub remaining_capacity: Option<i32>,
        /// Design capacity in mAh (macOS 27+ moved DesignCapacity into this nested dict).
        #[serde(default)]
        pub design_capacity: i32,
        /// Nominal charge capacity in mAh.
        #[serde(default)]
        pub nominal_charge_capacity: i32,
    }

    #[out, serde(rename_all = "camelCase"), cfg_attr(feature = "specta", derive(specta::Type))]
    #[repr, serde(default, rename_all(deserialize = "PascalCase", serialize = "camelCase"))]
    #[derive(Debug, Clone, Default, Deserialize, Serialize)]
    pub struct IORegistry {
        pub adapter_details: AdapterDetails,
        pub power_telemetry_data: Option<PowerTelemetryData>,
        // macOS 27+ moved capacity data (FullChargeCapacity, RemainingCapacity,
        // DesignCapacity in mAh) into this nested dict.
        #[serde(default)]
        pub battery_data: Option<BatteryData>,
        // macOS 27+ no longer exposes AbsoluteCapacity at the top level of
        // AppleSmartBattery (it moved into the nested BatteryData dict).
        #[serde(default)]
        pub absolute_capacity: i32,
        pub amperage: i32,
        pub voltage: i32,
        pub apple_raw_battery_voltage: Option<i32>,
        // macOS 27+ no longer exposes AppleRawCurrentCapacity /
        // AppleRawMaxCapacity via AppleSmartBattery. Default to 0 and let
        // callers fall back to BatteryData or CurrentCapacity / MaxCapacity.
        #[serde(default)]
        pub apple_raw_current_capacity: i32,
        #[serde(default)]
        pub apple_raw_max_capacity: i32,
        pub current_capacity: i32,
        pub cycle_count: i32,
        // macOS 27+ moved DesignCapacity into the nested BatteryData dict,
        // so the top-level key is often missing.
        #[serde(default)]
        pub design_capacity: i32,
        // Whether an adapter is plugged in. Distinct from `is_charging`: at
        // 100% on the adapter, external power is connected but nothing is
        // being charged.
        #[serde(default)]
        pub external_connected: bool,
        pub fully_charged: bool,
        pub instant_amperage: i32,
        pub is_charging: bool,
        pub max_capacity: i32,
        // Temperature is not always present (e.g. macOS 27 beta).
        #[serde(default)]
        pub temperature: i32,
        pub time_remaining: i32,
        #[serde(default)]
        pub update_time: i64,
    }
}

impl Deref for IORegistry {
    type Target = Option<PowerTelemetryData>;
    fn deref(&self) -> &Self::Target {
        &self.power_telemetry_data
    }
}

impl IORegistry {
    pub fn ptd(&self) -> Option<&PowerTelemetryData> {
        self.power_telemetry_data.as_ref()
    }
}

impl From<repr::AdapterDetails> for AdapterDetails {
    fn from(value: repr::AdapterDetails) -> Self {
        Self {
            adapter_voltage: value.adapter_voltage,
            is_wireless: value.is_wireless,
            watts: value.watts,
            name: value.name,
            current: value.current,
            description: value.description,
            adapter_power_tier: value.adapter_power_tier,
            max_voltage: value.max_voltage,
            max_current: value.max_current,
        }
    }
}

impl From<repr::BatteryData> for BatteryData {
    fn from(value: repr::BatteryData) -> Self {
        Self {
            full_charge_capacity: value.full_charge_capacity,
            remaining_capacity: value.remaining_capacity,
            design_capacity: value.design_capacity,
            nominal_charge_capacity: value.nominal_charge_capacity,
        }
    }
}

impl From<repr::PowerTelemetryData> for PowerTelemetryData {
    fn from(value: repr::PowerTelemetryData) -> Self {
        Self {
            adapter_efficiency_loss: value.adapter_efficiency_loss,
            battery_power: value.battery_power,
            system_current_in: value.system_current_in,
            system_energy_consumed: value.system_energy_consumed,
            system_load: value.system_load,
            system_power_in: value.system_power_in,
            system_voltage_in: value.system_voltage_in,
        }
    }
}

impl From<repr::IORegistry> for IORegistry {
    fn from(value: repr::IORegistry) -> Self {
        Self {
            adapter_details: value.adapter_details.into(),
            power_telemetry_data: value.power_telemetry_data.map(Into::into),
            battery_data: value.battery_data.map(Into::into),
            absolute_capacity: value.absolute_capacity,
            amperage: value.amperage,
            voltage: value.voltage,
            apple_raw_battery_voltage: value.apple_raw_battery_voltage,
            apple_raw_current_capacity: value.apple_raw_current_capacity,
            apple_raw_max_capacity: value.apple_raw_max_capacity,
            current_capacity: value.current_capacity,
            cycle_count: value.cycle_count,
            design_capacity: value.design_capacity,
            external_connected: value.external_connected,
            fully_charged: value.fully_charged,
            instant_amperage: value.instant_amperage,
            is_charging: value.is_charging,
            max_capacity: value.max_capacity,
            temperature: value.temperature,
            time_remaining: value.time_remaining,
            update_time: value.update_time,
        }
    }
}

impl From<repr::Diagnostics> for Diagnostics {
    fn from(value: repr::Diagnostics) -> Self {
        Self {
            ioregistry: value.ioregistry.into(),
        }
    }
}

impl From<repr::IORegistryDiagnostic> for IORegistryDiagnostic {
    fn from(value: repr::IORegistryDiagnostic) -> Self {
        Self {
            diagnostics: value.diagnostics.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(body: &str) -> Result<repr::IORegistry, plist::Error> {
        plist::from_bytes(format!("<plist version=\"1.0\"><dict>{body}</dict></plist>").as_bytes())
    }

    #[test]
    fn nested_capacities_survive_without_optional_telemetry() {
        let parsed = parse(
            r#"
            <key>CurrentCapacity</key><integer>0</integer>
            <key>MaxCapacity</key><integer>100</integer>
            <key>BatteryData</key><dict>
                <key>FullChargeCapacity</key><integer>5492</integer>
                <key>RemainingCapacity</key><integer>0</integer>
                <key>DesignCapacity</key><integer>6249</integer>
            </dict>
        "#,
        )
        .unwrap();
        let data = crate::provider::NormalizedResource::from(&IORegistry::from(parsed));
        assert_eq!(data.max_capacity, 5492);
        assert_eq!(data.design_capacity, 6249);
        assert_eq!(data.current_capacity, 0);
        assert!(data.absolute_battery_level.is_finite());
        assert!(data.power_estimated);
    }

    #[test]
    fn absent_fields_are_tolerated_but_malformed_values_return_errors() {
        assert!(parse("").is_ok());
        assert!(parse("<key>Amperage</key><string>invalid</string>").is_err());
        let partial = parse("<key>PowerTelemetryData</key><dict><key>BatteryPower</key><integer>-1250</integer></dict>").unwrap();
        let data = crate::provider::NormalizedResource::from(&IORegistry::from(partial));
        assert_eq!(data.battery_power, 1.25);
        assert!(!data.power_estimated);
    }

    #[test]
    fn remote_diagnostics_convert_without_reinterpreting_memory() {
        let raw: repr::IORegistryDiagnostic = plist::from_bytes(
            br#"<plist version="1.0"><dict>
          <key>Diagnostics</key><dict><key>IORegistry</key><dict>
            <key>AppleRawCurrentCapacity</key><integer>2100</integer>
            <key>AppleRawMaxCapacity</key><integer>3200</integer>
            <key>Amperage</key><integer>-250</integer>
          </dict></dict></dict></plist>"#,
        )
        .unwrap();
        let data: IORegistryDiagnostic = raw.into();
        assert_eq!(data.diagnostics.ioregistry.apple_raw_current_capacity, 2100);
        assert_eq!(data.diagnostics.ioregistry.amperage, -250);
    }
}
