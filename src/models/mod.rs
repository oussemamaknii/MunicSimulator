// models/mod.rs

use bson::Document;
use is_empty::IsEmpty;
use serde::{ser::SerializeMap, Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Presence {
    pub id: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_str: Option<String>,

    #[serde(rename(serialize = "type", deserialize = "type"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typ: Option<String>,
    pub connection_id: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fullreason: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cs: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct req<T> {
    pub meta: event,
    pub payload: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct event {
    pub account: String,
    pub event: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tracks {
    pub id: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_str: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub recorded_at: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub recorded_at_ms: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub received_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loc: Option<[f64; 2]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<[f64; 2]>,
    pub connection_id: i64,
    pub index: i64,

    // #[serde(serialize_with = "serialize_fields")]
    pub fields: Fields,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

fn serialize_fields<S>(f: &Fields, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut map = serializer.serialize_map(Some(f.map.len()))?;
    for (k, v) in f {
        match v {
            Some(e) => {
                if e.b64_value != None {
                    map.serialize_entry(k, v)?;
                }
            }
            None => continue,
        };
    }
    map.end()
}

type FieldMap = HashMap<String, Option<Base64>>;

impl<'a> IntoIterator for &'a Fields {
    type Item = <&'a FieldMap as IntoIterator>::Item;
    type IntoIter = <&'a FieldMap as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.map.iter()
    }
}

trait ShouldSkip {
    fn should_skip(&self) -> bool;
}

impl<T: Default + PartialEq> ShouldSkip for Option<T> {
    fn should_skip(&self) -> bool {
        self.as_ref().map(|v| *v == T::default()).unwrap_or(true)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "UPPERCASE", deserialize = "UPPERCASE"))]
pub struct Fields {
    #[serde(skip_serializing)]
    #[serde(default)]
    pub map: HashMap<String, Option<Base64>>,
    //---------------Generic Data-------------------------
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub gprmc_valid: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub gps_speed: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub gps_dir: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub gps_altitude: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub dio_ignition: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub batt: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub gprs_header: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub rssi: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub mdi_ext_batt_present: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub odo_partial_km: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub odo_full: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub dio_alarm: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub driver_id: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub dio_in_tor: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub gps_hdop: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub gps_pdop: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub gps_vdop: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub batt_temp: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub case_temp: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub modem_network_operator: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub gps_average_pdop_status: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub mdi_last_valid_gps_latitude: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub mdi_last_valid_gps_longitude: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub area_list: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub gps_fixed_sat_num: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub mvt_state: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub boot_reason: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub shutdown_type_and_reason: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub batt_volt: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub tx_kbytes: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub rx_kbytes: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub batt_warmup: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub batt_charging: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub dio_out_tor: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub modem_sim_iccid: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub modem_sim_imsi: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub serial_ppp_state: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub board_id: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub event: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub mdi_ext_batt_low: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub mdi_ext_batt_voltage: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub odo_partial_meter: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub odo_full_meter: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub mdi_zone_country: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub mdi_zone_state: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub mdi_vehicle_state_mvt: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub mdi_gps_antenna: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub mdi_dio_ain1: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub mdi_fuel_type: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub mdi_timezone: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub mdi_night_and_day: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub mdi_stat_global_trip_distance: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub mdi_stat_global_trip_fuel_consumed: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub mdi_serial_number: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub mdi_software_version: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub mdi_unplug_duration: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub mdi_unplug_timestamp: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub mdi_unplug_sporadic: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub mdi_replug_timestamp: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub mdi_unplug_count: Option<Base64>,
    //---------------OBD Data-------------------------
    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub obd_connected_protocol: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub obd_fuel_level_ratio: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub mdi_obd_fuel_level: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub mdi_obd_engine_load: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub mdi_obd_fuel_pressure: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub mdi_obd_throttle_position: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub mdi_obd_engine_oil_temp: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub mdi_obd_transmission_oil_temp: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub mdi_obd_engine_oil_pressure: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub mdi_obd_cruise_control: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub mdi_obd_state_water_in_fuel: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub mdi_obd_engine_coolant_level: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub mdi_obd_engine_coolant_temp: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub mdi_obd_engine_coolant_pressure: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub mdi_obd_fuel_delivery_pressure: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub mdi_obd_parking_brake_switch: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub mdi_obd_brake_application_pressure: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub mdi_obd_brake_pedal_status: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub mdi_obd_stack_name: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub mdi_obd_brake_pedal_position: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub mdi_obd_vin_alt: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub mdi_obd_fuel_type: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub mdi_obd_monitor_status: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub mdi_obd_vin_hash: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub mdi_obd_pid_1: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub mdi_obd_pid_2: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub mdi_obd_pid_3: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub mdi_obd_pid_4: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub mdi_obd_pid_5: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub mdi_obd_squish_vin: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub mdi_obd_mileage_meters: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub mdi_obd_speed: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub mdi_obd_rpm: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub mdi_obd_fuel: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub mdi_obd_vin: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub mdi_obd_mileage: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub mdi_obd_hev_engine_mode: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub mdi_obd_hev_engine_combustion_mode_time: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub mdi_obd_hev_engine_electric_mode_time: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub mdi_obd_hev_charging_state: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub mdi_obd_hev_battery_voltage: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub mdi_obd_hev_battery_current: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub mdi_obd_hev_battery_state_of_charge: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub mdi_obd_hev_state_supported: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub mdi_obd_co2_emissions: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub obd_supported_pids_00: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub mdi_obd_distance_since_dtc_cleared: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub mdi_obd_time_since_dtc_cleared: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub mdi_ext_volt_snapshot: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub mdi_obd_ambient_air_temperature: Option<Base64>,

    #[serde(skip_serializing_if = "ShouldSkip::should_skip")]
    pub mdi_obd_barometric_pressure: Option<Base64>,
}

#[derive(Debug, Serialize, Default, PartialEq, Deserialize, IsEmpty, Clone)]
pub struct Base64 {
    #[serde(skip_serializing_if = "Option::is_none")]
    b64_value: Option<String>,
}

impl From<&Document> for Fields {
    fn from(doc: &Document) -> Fields {
        let mut field = Fields {
            map: HashMap::new(),

            gprmc_valid: Some(Base64::from(match doc.get_document("gprmc_valid") {
                Ok(e) => Some(e),
                Err(e) => None,
            })),

            gps_speed: Some(Base64::from(match doc.get_document("gps_speed") {
                Ok(e) => Some(e),
                Err(e) => None,
            })),
            gps_dir: Some(Base64::from(match doc.get_document("gps_dir") {
                Ok(e) => Some(e),
                Err(e) => None,
            })),
            gps_altitude: Some(Base64::from(match doc.get_document("gps_altitude") {
                Ok(e) => Some(e),
                Err(e) => None,
            })),
            dio_ignition: Some(Base64::from(match doc.get_document("dio_ignition") {
                Ok(e) => Some(e),
                Err(e) => None,
            })),
            batt: Some(Base64::from(match doc.get_document("batt") {
                Ok(e) => Some(e),
                Err(e) => None,
            })),
            gprs_header: Some(Base64::from(match doc.get_document("gprs_header") {
                Ok(e) => Some(e),
                Err(e) => None,
            })),
            rssi: Some(Base64::from(match doc.get_document("rssi") {
                Ok(e) => Some(e),
                Err(e) => None,
            })),
            mdi_ext_batt_present: Some(Base64::from(
                match doc.get_document("mdi_ext_batt_present") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            )),
            odo_partial_km: Some(Base64::from(match doc.get_document("odo_partial_km") {
                Ok(e) => Some(e),
                Err(e) => None,
            })),
            odo_full: Some(Base64::from(match doc.get_document("odo_full") {
                Ok(e) => Some(e),
                Err(e) => None,
            })),
            dio_alarm: Some(Base64::from(match doc.get_document("dio_alarm") {
                Ok(e) => Some(e),
                Err(e) => None,
            })),
            driver_id: Some(Base64::from(match doc.get_document("driver_id") {
                Ok(e) => Some(e),
                Err(e) => None,
            })),
            dio_in_tor: Some(Base64::from(match doc.get_document("dio_in_tor") {
                Ok(e) => Some(e),
                Err(e) => None,
            })),
            gps_hdop: Some(Base64::from(match doc.get_document("gps_hdop") {
                Ok(e) => Some(e),
                Err(e) => None,
            })),
            gps_pdop: Some(Base64::from(match doc.get_document("gps_pdop") {
                Ok(e) => Some(e),
                Err(e) => None,
            })),
            gps_vdop: Some(Base64::from(match doc.get_document("gps_vdop") {
                Ok(e) => Some(e),
                Err(e) => None,
            })),
            batt_temp: Some(Base64::from(match doc.get_document("batt_temp") {
                Ok(e) => Some(e),
                Err(e) => None,
            })),
            case_temp: Some(Base64::from(match doc.get_document("case_temp") {
                Ok(e) => Some(e),
                Err(e) => None,
            })),
            modem_network_operator: Some(Base64::from(
                match doc.get_document("modem_network_operator") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            )),
            gps_average_pdop_status: Some(Base64::from(
                match doc.get_document("gps_average_pdop_status") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            )),
            mdi_last_valid_gps_latitude: Some(Base64::from(
                match doc.get_document("mdi_last_valid_gps_latitude") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            )),
            mdi_last_valid_gps_longitude: Some(Base64::from(
                match doc.get_document("mdi_last_valid_gps_longitude") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            )),
            area_list: Some(Base64::from(match doc.get_document("area_list") {
                Ok(e) => Some(e),
                Err(e) => None,
            })),
            gps_fixed_sat_num: Some(Base64::from(match doc.get_document("gps_fixed_sat_num") {
                Ok(e) => Some(e),
                Err(e) => None,
            })),
            mvt_state: Some(Base64::from(match doc.get_document("mvt_state") {
                Ok(e) => Some(e),
                Err(e) => None,
            })),
            boot_reason: Some(Base64::from(match doc.get_document("boot_reason") {
                Ok(e) => Some(e),
                Err(e) => None,
            })),
            shutdown_type_and_reason: Some(Base64::from(
                match doc.get_document("shutdown_type_and_reason") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            )),
            batt_volt: Some(Base64::from(match doc.get_document("batt_volt") {
                Ok(e) => Some(e),
                Err(e) => None,
            })),
            tx_kbytes: Some(Base64::from(match doc.get_document("tx_kbytes") {
                Ok(e) => Some(e),
                Err(e) => None,
            })),
            rx_kbytes: Some(Base64::from(match doc.get_document("rx_kbytes") {
                Ok(e) => Some(e),
                Err(e) => None,
            })),
            batt_warmup: Some(Base64::from(match doc.get_document("batt_warmup") {
                Ok(e) => Some(e),
                Err(e) => None,
            })),
            batt_charging: Some(Base64::from(match doc.get_document("batt_charging") {
                Ok(e) => Some(e),
                Err(e) => None,
            })),
            dio_out_tor: Some(Base64::from(match doc.get_document("dio_out_tor") {
                Ok(e) => Some(e),
                Err(e) => None,
            })),
            modem_sim_iccid: Some(Base64::from(match doc.get_document("modem_sim_iccid") {
                Ok(e) => Some(e),
                Err(e) => None,
            })),
            modem_sim_imsi: Some(Base64::from(match doc.get_document("modem_sim_imsi") {
                Ok(e) => Some(e),
                Err(e) => None,
            })),
            serial_ppp_state: Some(Base64::from(match doc.get_document("serial_ppp_state") {
                Ok(e) => Some(e),
                Err(e) => None,
            })),
            board_id: Some(Base64::from(match doc.get_document("board_id") {
                Ok(e) => Some(e),
                Err(e) => None,
            })),
            event: Some(Base64::from(match doc.get_document("event") {
                Ok(e) => Some(e),
                Err(e) => None,
            })),
            mdi_ext_batt_low: Some(Base64::from(match doc.get_document("mdi_ext_batt_low") {
                Ok(e) => Some(e),
                Err(e) => None,
            })),
            mdi_ext_batt_voltage: Some(Base64::from(
                match doc.get_document("mdi_ext_batt_voltage") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            )),
            odo_partial_meter: Some(Base64::from(match doc.get_document("odo_partial_meter") {
                Ok(e) => Some(e),
                Err(e) => None,
            })),
            odo_full_meter: Some(Base64::from(match doc.get_document("odo_full_meter") {
                Ok(e) => Some(e),
                Err(e) => None,
            })),
            mdi_zone_country: Some(Base64::from(match doc.get_document("mdi_zone_country") {
                Ok(e) => Some(e),
                Err(e) => None,
            })),
            mdi_zone_state: Some(Base64::from(match doc.get_document("mdi_zone_state") {
                Ok(e) => Some(e),
                Err(e) => None,
            })),
            mdi_vehicle_state_mvt: Some(Base64::from(
                match doc.get_document("mdi_vehicle_state_mvt") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            )),
            mdi_gps_antenna: Some(Base64::from(match doc.get_document("mdi_gps_antenna") {
                Ok(e) => Some(e),
                Err(e) => None,
            })),
            mdi_dio_ain1: Some(Base64::from(match doc.get_document("mdi_dio_ain1") {
                Ok(e) => Some(e),
                Err(e) => None,
            })),
            mdi_fuel_type: Some(Base64::from(match doc.get_document("mdi_fuel_type") {
                Ok(e) => Some(e),
                Err(e) => None,
            })),
            mdi_timezone: Some(Base64::from(match doc.get_document("mdi_timezone") {
                Ok(e) => Some(e),
                Err(e) => None,
            })),
            mdi_night_and_day: Some(Base64::from(match doc.get_document("mdi_night_and_day") {
                Ok(e) => Some(e),
                Err(e) => None,
            })),
            mdi_stat_global_trip_distance: Some(Base64::from(
                match doc.get_document("mdi_stat_global_trip_distance") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            )),
            mdi_stat_global_trip_fuel_consumed: Some(Base64::from(
                match doc.get_document("mdi_stat_global_trip_fuel_consumed") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            )),
            mdi_serial_number: Some(Base64::from(match doc.get_document("mdi_serial_number") {
                Ok(e) => Some(e),
                Err(e) => None,
            })),
            mdi_software_version: Some(Base64::from(
                match doc.get_document("mdi_software_version") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            )),
            mdi_unplug_duration: Some(Base64::from(
                match doc.get_document("mdi_unplug_duration") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            )),
            mdi_unplug_timestamp: Some(Base64::from(
                match doc.get_document("mdi_unplug_timestamp") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            )),
            mdi_unplug_sporadic: Some(Base64::from(
                match doc.get_document("mdi_unplug_sporadic") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            )),
            mdi_replug_timestamp: Some(Base64::from(
                match doc.get_document("mdi_replug_timestamp") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            )),
            mdi_unplug_count: Some(Base64::from(match doc.get_document("mdi_unplug_count") {
                Ok(e) => Some(e),
                Err(e) => None,
            })), //------------------OBD Data--------------------------------------
            obd_connected_protocol: Some(Base64::from(
                match doc.get_document("obd_connected_protocol") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            )),
            obd_fuel_level_ratio: Some(Base64::from(
                match doc.get_document("obd_fuel_level_ratio") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            )),
            mdi_obd_fuel_level: Some(Base64::from(match doc.get_document("mdi_obd_fuel_level") {
                Ok(e) => Some(e),
                Err(e) => None,
            })),
            mdi_obd_engine_load: Some(Base64::from(
                match doc.get_document("mdi_obd_engine_load") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            )),
            mdi_obd_fuel_pressure: Some(Base64::from(
                match doc.get_document("mdi_obd_fuel_pressure") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            )),
            mdi_obd_throttle_position: Some(Base64::from(
                match doc.get_document("mdi_obd_throttle_position") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            )),
            mdi_obd_engine_oil_temp: Some(Base64::from(
                match doc.get_document("mdi_obd_engine_oil_temp") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            )),
            mdi_obd_transmission_oil_temp: Some(Base64::from(
                match doc.get_document("mdi_obd_transmission_oil_temp") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            )),
            mdi_obd_engine_oil_pressure: Some(Base64::from(
                match doc.get_document("mdi_obd_engine_oil_pressure") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            )),
            mdi_obd_cruise_control: Some(Base64::from(
                match doc.get_document("mdi_obd_cruise_control") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            )),
            mdi_obd_state_water_in_fuel: Some(Base64::from(
                match doc.get_document("mdi_obd_state_water_in_fuel") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            )),
            mdi_obd_engine_coolant_level: Some(Base64::from(
                match doc.get_document("mdi_obd_engine_coolant_level") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            )),
            mdi_obd_engine_coolant_temp: Some(Base64::from(
                match doc.get_document("mdi_obd_engine_coolant_temp") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            )),
            mdi_obd_engine_coolant_pressure: Some(Base64::from(
                match doc.get_document("mdi_obd_engine_coolant_pressure") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            )),
            mdi_obd_fuel_delivery_pressure: Some(Base64::from(
                match doc.get_document("mdi_obd_fuel_delivery_pressure") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            )),
            mdi_obd_parking_brake_switch: Some(Base64::from(
                match doc.get_document("mdi_obd_parking_brake_switch") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            )),
            mdi_obd_brake_application_pressure: Some(Base64::from(
                match doc.get_document("mdi_obd_brake_application_pressure") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            )),
            mdi_obd_brake_pedal_status: Some(Base64::from(
                match doc.get_document("mdi_obd_brake_pedal_status") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            )),
            mdi_obd_stack_name: Some(Base64::from(match doc.get_document("mdi_obd_stack_name") {
                Ok(e) => Some(e),
                Err(e) => None,
            })),
            mdi_obd_brake_pedal_position: Some(Base64::from(
                match doc.get_document("mdi_obd_brake_pedal_position") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            )),
            mdi_obd_vin_alt: Some(Base64::from(match doc.get_document("mdi_obd_vin_alt") {
                Ok(e) => Some(e),
                Err(e) => None,
            })),
            mdi_obd_fuel_type: Some(Base64::from(match doc.get_document("mdi_obd_fuel_type") {
                Ok(e) => Some(e),
                Err(e) => None,
            })),
            mdi_obd_monitor_status: Some(Base64::from(
                match doc.get_document("mdi_obd_monitor_status") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            )),
            mdi_obd_vin_hash: Some(Base64::from(match doc.get_document("mdi_obd_vin_hash") {
                Ok(e) => Some(e),
                Err(e) => None,
            })),
            mdi_obd_pid_1: Some(Base64::from(match doc.get_document("mdi_obd_pid_1") {
                Ok(e) => Some(e),
                Err(e) => None,
            })),
            mdi_obd_pid_2: Some(Base64::from(match doc.get_document("mdi_obd_pid_2") {
                Ok(e) => Some(e),
                Err(e) => None,
            })),
            mdi_obd_pid_3: Some(Base64::from(match doc.get_document("mdi_obd_pid_3") {
                Ok(e) => Some(e),
                Err(e) => None,
            })),
            mdi_obd_pid_4: Some(Base64::from(match doc.get_document("mdi_obd_pid_4") {
                Ok(e) => Some(e),
                Err(e) => None,
            })),
            mdi_obd_pid_5: Some(Base64::from(match doc.get_document("mdi_obd_pid_5") {
                Ok(e) => Some(e),
                Err(e) => None,
            })),
            mdi_obd_squish_vin: Some(Base64::from(match doc.get_document("mdi_obd_squish_vin") {
                Ok(e) => Some(e),
                Err(e) => None,
            })),
            mdi_obd_mileage_meters: Some(Base64::from(
                match doc.get_document("mdi_obd_mileage_meters") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            )),
            mdi_obd_speed: Some(Base64::from(match doc.get_document("mdi_obd_speed") {
                Ok(e) => Some(e),
                Err(e) => None,
            })),
            mdi_obd_rpm: Some(Base64::from(match doc.get_document("mdi_obd_rpm") {
                Ok(e) => Some(e),
                Err(e) => None,
            })),
            mdi_obd_fuel: Some(Base64::from(match doc.get_document("mdi_obd_fuel") {
                Ok(e) => Some(e),
                Err(e) => None,
            })),
            mdi_obd_vin: Some(Base64::from(match doc.get_document("mdi_obd_vin") {
                Ok(e) => Some(e),
                Err(e) => None,
            })),
            mdi_obd_mileage: Some(Base64::from(match doc.get_document("mdi_obd_mileage") {
                Ok(e) => Some(e),
                Err(e) => None,
            })),
            mdi_obd_hev_engine_mode: Some(Base64::from(
                match doc.get_document("mdi_obd_hev_engine_mode") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            )),
            mdi_obd_hev_engine_combustion_mode_time: Some(Base64::from(
                match doc.get_document("mdi_obd_hev_engine_combustion_mode_time") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            )),
            mdi_obd_hev_engine_electric_mode_time: Some(Base64::from(
                match doc.get_document("mdi_obd_hev_engine_electric_mode_time") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            )),
            mdi_obd_hev_charging_state: Some(Base64::from(
                match doc.get_document("mdi_obd_hev_charging_state") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            )),
            mdi_obd_hev_battery_voltage: Some(Base64::from(
                match doc.get_document("mdi_obd_hev_battery_voltage") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            )),
            mdi_obd_hev_battery_current: Some(Base64::from(
                match doc.get_document("mdi_obd_hev_battery_current") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            )),
            mdi_obd_hev_battery_state_of_charge: Some(Base64::from(
                match doc.get_document("mdi_obd_hev_battery_state_of_charge") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            )),
            mdi_obd_hev_state_supported: Some(Base64::from(
                match doc.get_document("mdi_obd_hev_state_supported") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            )),
            mdi_obd_co2_emissions: Some(Base64::from(
                match doc.get_document("mdi_obd_co2_emissions") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            )),
            obd_supported_pids_00: Some(Base64::from(
                match doc.get_document("obd_supported_pids_00") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            )),
            mdi_obd_distance_since_dtc_cleared: Some(Base64::from(
                match doc.get_document("mdi_obd_distance_since_dtc_cleared") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            )),
            mdi_obd_time_since_dtc_cleared: Some(Base64::from(
                match doc.get_document("mdi_obd_time_since_dtc_cleared") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            )),
            mdi_ext_volt_snapshot: Some(Base64::from(
                match doc.get_document("mdi_ext_volt_snapshot") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            )),
            mdi_obd_ambient_air_temperature: Some(Base64::from(
                match doc.get_document("mdi_obd_ambient_air_temperature") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            )),
            mdi_obd_barometric_pressure: Some(Base64::from(
                match doc.get_document("mdi_obd_barometric_pressure") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            )),
        };
        field.map = HashMap::from([
            ("gprmc_valid".to_string(), field.gprmc_valid.clone()),
            ("gps_speed".to_string(), field.gps_speed.clone()),
            ("gps_dir".to_string(), field.gps_dir.clone()),
            ("gps_altitude".to_string(), field.gps_altitude.clone()),
            ("dio_ignition".to_string(), field.dio_ignition.clone()),
            ("batt".to_string(), field.batt.clone()),
            ("gprs_header".to_string(), field.gprs_header.clone()),
            ("rssi".to_string(), field.rssi.clone()),
            (
                "mdi_ext_batt_present".to_string(),
                field.mdi_ext_batt_present.clone(),
            ),
            ("odo_partial_km".to_string(), field.odo_partial_km.clone()),
            ("odo_full".to_string(), field.odo_full.clone()),
            ("dio_alarm".to_string(), field.dio_alarm.clone()),
            ("driver_id".to_string(), field.driver_id.clone()),
            ("dio_in_tor".to_string(), field.dio_in_tor.clone()),
            ("gps_hdop".to_string(), field.gps_hdop.clone()),
            ("gps_pdop".to_string(), field.gps_pdop.clone()),
            ("gps_vdop".to_string(), field.gps_vdop.clone()),
            ("batt_temp".to_string(), field.batt_temp.clone()),
            ("case_temp".to_string(), field.case_temp.clone()),
            (
                "modem_network_operator".to_string(),
                field.modem_network_operator.clone(),
            ),
            (
                "gps_average_pdop_status".to_string(),
                field.gps_average_pdop_status.clone(),
            ),
            (
                "mdi_last_valid_gps_latitude".to_string(),
                field.mdi_last_valid_gps_latitude.clone(),
            ),
            (
                "mdi_last_valid_gps_longitude".to_string(),
                field.mdi_last_valid_gps_longitude.clone(),
            ),
            ("area_list".to_string(), field.area_list.clone()),
            (
                "gps_fixed_sat_num".to_string(),
                field.gps_fixed_sat_num.clone(),
            ),
            ("mvt_state".to_string(), field.mvt_state.clone()),
            ("boot_reason".to_string(), field.boot_reason.clone()),
            (
                "shutdown_type_and_reason".to_string(),
                field.shutdown_type_and_reason.clone(),
            ),
            ("batt_volt".to_string(), field.batt_volt.clone()),
            ("tx_kbytes".to_string(), field.tx_kbytes.clone()),
            ("rx_kbytes".to_string(), field.rx_kbytes.clone()),
            ("batt_warmup".to_string(), field.batt_warmup.clone()),
            ("batt_charging".to_string(), field.batt_charging.clone()),
            ("dio_out_tor".to_string(), field.dio_out_tor.clone()),
            ("modem_sim_iccid".to_string(), field.modem_sim_iccid.clone()),
            ("modem_sim_imsi".to_string(), field.modem_sim_imsi.clone()),
            (
                "serial_ppp_state".to_string(),
                field.serial_ppp_state.clone(),
            ),
            ("board_id".to_string(), field.board_id.clone()),
            ("event".to_string(), field.event.clone()),
            (
                "mdi_ext_batt_low".to_string(),
                field.mdi_ext_batt_low.clone(),
            ),
            (
                "mdi_ext_batt_voltage".to_string(),
                field.mdi_ext_batt_voltage.clone(),
            ),
            (
                "odo_partial_meter".to_string(),
                field.odo_partial_meter.clone(),
            ),
            ("odo_full_meter".to_string(), field.odo_full_meter.clone()),
            (
                "mdi_zone_country".to_string(),
                field.mdi_zone_country.clone(),
            ),
            ("mdi_zone_state".to_string(), field.mdi_zone_state.clone()),
            (
                "mdi_vehicle_state_mvt".to_string(),
                field.mdi_vehicle_state_mvt.clone(),
            ),
            ("mdi_gps_antenna".to_string(), field.mdi_gps_antenna.clone()),
            ("mdi_dio_ain1".to_string(), field.mdi_dio_ain1.clone()),
            ("mdi_fuel_type".to_string(), field.mdi_fuel_type.clone()),
            ("mdi_timezone".to_string(), field.mdi_timezone.clone()),
            (
                "mdi_night_and_day".to_string(),
                field.mdi_night_and_day.clone(),
            ),
            (
                "mdi_stat_global_trip_distance".to_string(),
                field.mdi_stat_global_trip_distance.clone(),
            ),
            (
                "mdi_stat_global_trip_fuel_consumed".to_string(),
                field.mdi_stat_global_trip_fuel_consumed.clone(),
            ),
            (
                "mdi_serial_number".to_string(),
                field.mdi_serial_number.clone(),
            ),
            (
                "mdi_software_version".to_string(),
                field.mdi_software_version.clone(),
            ),
            (
                "mdi_unplug_duration".to_string(),
                field.mdi_unplug_duration.clone(),
            ),
            (
                "mdi_unplug_timestamp".to_string(),
                field.mdi_unplug_timestamp.clone(),
            ),
            (
                "mdi_unplug_sporadic".to_string(),
                field.mdi_unplug_sporadic.clone(),
            ),
            (
                "mdi_replug_timestamp".to_string(),
                field.mdi_replug_timestamp.clone(),
            ),
            (
                "mdi_unplug_count".to_string(),
                field.mdi_unplug_count.clone(),
            ),
            (
                "obd_connected_protocol".to_string(),
                field.obd_connected_protocol.clone(),
            ),
            (
                "obd_fuel_level_ratio".to_string(),
                field.obd_fuel_level_ratio.clone(),
            ),
            (
                "mdi_obd_fuel_level".to_string(),
                field.mdi_obd_fuel_level.clone(),
            ),
            (
                "mdi_obd_engine_load".to_string(),
                field.mdi_obd_engine_load.clone(),
            ),
            (
                "mdi_obd_fuel_pressure".to_string(),
                field.mdi_obd_fuel_pressure.clone(),
            ),
            (
                "mdi_obd_throttle_position".to_string(),
                field.mdi_obd_throttle_position.clone(),
            ),
            (
                "mdi_obd_engine_oil_temp".to_string(),
                field.mdi_obd_engine_oil_temp.clone(),
            ),
            (
                "mdi_obd_transmission_oil_temp".to_string(),
                field.mdi_obd_transmission_oil_temp.clone(),
            ),
            (
                "mdi_obd_engine_oil_pressure".to_string(),
                field.mdi_obd_engine_oil_pressure.clone(),
            ),
            (
                "mdi_obd_cruise_control".to_string(),
                field.mdi_obd_cruise_control.clone(),
            ),
            (
                "mdi_obd_state_water_in_fuel".to_string(),
                field.mdi_obd_state_water_in_fuel.clone(),
            ),
            (
                "mdi_obd_engine_coolant_level".to_string(),
                field.mdi_obd_engine_coolant_level.clone(),
            ),
            (
                "mdi_obd_engine_coolant_temp".to_string(),
                field.mdi_obd_engine_coolant_temp.clone(),
            ),
            (
                "mdi_obd_engine_coolant_pressure".to_string(),
                field.mdi_obd_engine_coolant_pressure.clone(),
            ),
            (
                "mdi_obd_fuel_delivery_pressure".to_string(),
                field.mdi_obd_fuel_delivery_pressure.clone(),
            ),
            (
                "mdi_obd_parking_brake_switch".to_string(),
                field.mdi_obd_parking_brake_switch.clone(),
            ),
            (
                "mdi_obd_brake_application_pressure".to_string(),
                field.mdi_obd_brake_application_pressure.clone(),
            ),
            (
                "mdi_obd_brake_pedal_status".to_string(),
                field.mdi_obd_brake_pedal_status.clone(),
            ),
            (
                "mdi_obd_stack_name".to_string(),
                field.mdi_obd_stack_name.clone(),
            ),
            (
                "mdi_obd_brake_pedal_position".to_string(),
                field.mdi_obd_brake_pedal_position.clone(),
            ),
            ("mdi_obd_vin_alt".to_string(), field.mdi_obd_vin_alt.clone()),
            (
                "mdi_obd_fuel_type".to_string(),
                field.mdi_obd_fuel_type.clone(),
            ),
            (
                "mdi_obd_monitor_status".to_string(),
                field.mdi_obd_monitor_status.clone(),
            ),
            (
                "mdi_obd_vin_hash".to_string(),
                field.mdi_obd_vin_hash.clone(),
            ),
            ("mdi_obd_pid_1".to_string(), field.mdi_obd_pid_1.clone()),
            ("mdi_obd_pid_2".to_string(), field.mdi_obd_pid_2.clone()),
            ("mdi_obd_pid_3".to_string(), field.mdi_obd_pid_3.clone()),
            ("mdi_obd_pid_4".to_string(), field.mdi_obd_pid_4.clone()),
            ("mdi_obd_pid_5".to_string(), field.mdi_obd_pid_5.clone()),
            (
                "mdi_obd_squish_vin".to_string(),
                field.mdi_obd_squish_vin.clone(),
            ),
            (
                "mdi_obd_mileage_meters".to_string(),
                field.mdi_obd_mileage_meters.clone(),
            ),
            ("mdi_obd_speed".to_string(), field.mdi_obd_speed.clone()),
            ("mdi_obd_rpm".to_string(), field.mdi_obd_rpm.clone()),
            ("mdi_obd_fuel".to_string(), field.mdi_obd_fuel.clone()),
            ("mdi_obd_vin".to_string(), field.mdi_obd_vin.clone()),
            ("mdi_obd_mileage".to_string(), field.mdi_obd_mileage.clone()),
            (
                "mdi_obd_hev_engine_mode".to_string(),
                field.mdi_obd_hev_engine_mode.clone(),
            ),
            (
                "mdi_obd_hev_engine_combustion_mode_time".to_string(),
                field.mdi_obd_hev_engine_combustion_mode_time.clone(),
            ),
            (
                "mdi_obd_hev_engine_electric_mode_time".to_string(),
                field.mdi_obd_hev_engine_electric_mode_time.clone(),
            ),
            (
                "mdi_obd_hev_charging_state".to_string(),
                field.mdi_obd_hev_charging_state.clone(),
            ),
            (
                "mdi_obd_hev_battery_voltage".to_string(),
                field.mdi_obd_hev_battery_voltage.clone(),
            ),
            (
                "mdi_obd_hev_battery_current".to_string(),
                field.mdi_obd_hev_battery_current.clone(),
            ),
            (
                "mdi_obd_hev_battery_state_of_charge".to_string(),
                field.mdi_obd_hev_battery_state_of_charge.clone(),
            ),
            (
                "mdi_obd_hev_state_supported".to_string(),
                field.mdi_obd_hev_state_supported.clone(),
            ),
            (
                "mdi_obd_co2_emissions".to_string(),
                field.mdi_obd_co2_emissions.clone(),
            ),
            (
                "obd_supported_pids_00".to_string(),
                field.obd_supported_pids_00.clone(),
            ),
            (
                "mdi_obd_distance_since_dtc_cleared".to_string(),
                field.mdi_obd_distance_since_dtc_cleared.clone(),
            ),
            (
                "mdi_obd_time_since_dtc_cleared".to_string(),
                field.mdi_obd_time_since_dtc_cleared.clone(),
            ),
            (
                "mdi_ext_volt_snapshot".to_string(),
                field.mdi_ext_volt_snapshot.clone(),
            ),
            (
                "mdi_obd_ambient_air_temperature".to_string(),
                field.mdi_obd_ambient_air_temperature.clone(),
            ),
            (
                "mdi_obd_barometric_pressure".to_string(),
                field.mdi_obd_barometric_pressure.clone(),
            ),
        ]);
        field
    }
}

impl From<Option<&bson::Document>> for Base64 {
    fn from(doc: Option<&bson::Document>) -> Base64 {
        let base64 = Base64 {
            b64_value: match doc {
                Some(e) => Some(e.get_str("b64_value").unwrap().to_string()),
                None => Some(String::new()).filter(|s| !s.is_empty()),
            },
        };
        base64
    }
}
