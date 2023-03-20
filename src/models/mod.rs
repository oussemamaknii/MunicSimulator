// models/mod.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Presence {
    pub id: i64,
    pub id_str: Option<String>,
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub typ: Option<String>,
    pub connection_id: i64,
    pub fullreason: Option<String>,
    pub cs: Option<String>,
    pub ip: Option<String>,
    pub protocol: Option<String>,
    pub reason: Option<String>,
    pub asset: Option<String>,
    pub time: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tracks {
    pub id: i64,
    pub id_str: Option<String>,
    pub asset: Option<String>,
    pub recorded_at: Option<String>,
    pub recorded_at_ms: Option<String>,
    pub received_at: Option<String>,
    pub loc: Option<[f64; 2]>,
    pub location: Option<[f64; 2]>,
    pub connection_id: i64,
    pub index: i64,
    pub fields: Option<Fields>,
    pub url: Option<String>,
}
// Only Generic Fields
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "lowercase", deserialize = "UPPERCASE"))]
pub struct Fields {
    pub gprmc_valid: Option<Base64>,
    pub gps_speed: Option<Base64>,
    pub gps_dir: Option<Base64>,
    pub gps_altitude: Option<Base64>,
    pub dio_ignition: Option<Base64>,
    pub batt: Option<Base64>,
    pub gprs_header: Option<Base64>,
    pub rssi: Option<Base64>,
    pub mdi_ext_batt_present: Option<Base64>,
    pub odo_partial_km: Option<Base64>,
    pub odo_full: Option<Base64>,
    pub dio_alarm: Option<Base64>,
    pub driver_id: Option<Base64>,
    pub dio_in_tor: Option<Base64>,
    pub gps_hdop: Option<Base64>,
    pub gps_pdop: Option<Base64>,
    pub gps_vdop: Option<Base64>,
    pub batt_temp: Option<Base64>,
    pub case_temp: Option<Base64>,
    pub modem_network_operator: Option<Base64>,
    pub gps_average_pdop_status: Option<Base64>,
    pub mdi_last_valid_gps_latitude: Option<Base64>,
    pub mdi_last_valid_gps_longitude: Option<Base64>,
    pub area_list: Option<Base64>,
    pub gps_fixed_sat_num: Option<Base64>,
    pub mvt_state: Option<Base64>,
    pub boot_reason: Option<Base64>,
    pub shutdown_type_and_reason: Option<Base64>,
    pub batt_volt: Option<Base64>,
    pub tx_kbytes: Option<Base64>,
    pub rx_kbytes: Option<Base64>,
    pub batt_warmup: Option<Base64>,
    pub batt_charging: Option<Base64>,
    pub dio_out_tor: Option<Base64>,
    pub modem_sim_iccid: Option<Base64>,
    pub modem_sim_imsi: Option<Base64>,
    pub serial_ppp_state: Option<Base64>,
    pub board_id: Option<Base64>,
    pub event: Option<Base64>,
    pub mdi_ext_batt_low: Option<Base64>,
    pub mdi_ext_batt_voltage: Option<Base64>,
    pub odo_partial_meter: Option<Base64>,
    pub odo_full_meter: Option<Base64>,
    pub mdi_zone_country: Option<Base64>,
    pub mdi_zone_state: Option<Base64>,
    pub mdi_vehicle_state_mvt: Option<Base64>,
    pub mdi_gps_antenna: Option<Base64>,
    pub mdi_dio_ain1: Option<Base64>,
    pub mdi_fuel_type: Option<Base64>,
    pub mdi_timezone: Option<Base64>,
    pub mdi_night_and_day: Option<Base64>,
    pub mdi_stat_global_trip_distance: Option<Base64>,
    pub mdi_stat_global_trip_fuel_consumed: Option<Base64>,
    pub mdi_serial_number: Option<Base64>,
    pub mdi_software_version: Option<Base64>,
    pub mdi_unplug_duration: Option<Base64>,
    pub mdi_unplug_timestamp: Option<Base64>,
    pub mdi_unplug_sporadic: Option<Base64>,
    pub mdi_replug_timestamp: Option<Base64>,
    pub mdi_unplug_count: Option<Base64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Base64 {
    b64_value: Option<String>,
}
