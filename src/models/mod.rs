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
pub struct Fields {
    pub GPRMC_VALID: Option<Base64>,
    pub GPS_SPEED: Option<Base64>,
    pub GPS_DIR: Option<Base64>,
    pub GPS_ALTITUDE: Option<Base64>,
    pub DIO_IGNITION: Option<Base64>,
    pub BATT: Option<Base64>,
    pub GPRS_HEADER: Option<Base64>,
    pub RSSI: Option<Base64>,
    pub MDI_EXT_BATT_PRESENT: Option<Base64>,
    pub ODO_PARTIAL_KM: Option<Base64>,
    pub ODO_FULL: Option<Base64>,
    pub DIO_ALARM: Option<Base64>,
    pub DRIVER_ID: Option<Base64>,
    pub DIO_IN_TOR: Option<Base64>,
    pub GPS_HDOP: Option<Base64>,
    pub GPS_PDOP: Option<Base64>,
    pub GPS_VDOP: Option<Base64>,
    pub BATT_TEMP: Option<Base64>,
    pub CASE_TEMP: Option<Base64>,
    pub MODEM_NETWORK_OPERATOR: Option<Base64>,
    pub GPS_AVERAGE_PDOP_STATUS: Option<Base64>,
    pub MDI_LAST_VALID_GPS_LATITUDE: Option<Base64>,
    pub MDI_LAST_VALID_GPS_LONGITUDE: Option<Base64>,
    pub AREA_LIST: Option<Base64>,
    pub GPS_FIXED_SAT_NUM: Option<Base64>,
    pub MVT_STATE: Option<Base64>,
    pub BOOT_REASON: Option<Base64>,
    pub SHUTDOWN_TYPE_AND_REASON: Option<Base64>,
    pub BATT_VOLT: Option<Base64>,
    pub TX_KBYTES: Option<Base64>,
    pub RX_KBYTES: Option<Base64>,
    pub BATT_WARMUP: Option<Base64>,
    pub BATT_CHARGING: Option<Base64>,
    pub DIO_OUT_TOR: Option<Base64>,
    pub MODEM_SIM_ICCID: Option<Base64>,
    pub MODEM_SIM_IMSI: Option<Base64>,
    pub SERIAL_PPP_STATE: Option<Base64>,
    pub BOARD_ID: Option<Base64>,
    pub EVENT: Option<Base64>,
    pub MDI_EXT_BATT_LOW: Option<Base64>,
    pub MDI_EXT_BATT_VOLTAGE: Option<Base64>,
    pub ODO_PARTIAL_METER: Option<Base64>,
    pub ODO_FULL_METER: Option<Base64>,
    pub MDI_ZONE_COUNTRY: Option<Base64>,
    pub MDI_ZONE_STATE: Option<Base64>,
    pub MDI_VEHICLE_STATE_MVT: Option<Base64>,
    pub MDI_GPS_ANTENNA: Option<Base64>,
    pub MDI_DIO_AIN1: Option<Base64>,
    pub MDI_FUEL_TYPE: Option<Base64>,
    pub MDI_TIMEZONE: Option<Base64>,
    pub MDI_NIGHT_AND_DAY: Option<Base64>,
    pub MDI_STAT_GLOBAL_TRIP_DISTANCE: Option<Base64>,
    pub MDI_STAT_GLOBAL_TRIP_FUEL_CONSUMED: Option<Base64>,
    pub MDI_SERIAL_NUMBER: Option<Base64>,
    pub MDI_SOFTWARE_VERSION: Option<Base64>,
    pub MDI_UNPLUG_DURATION: Option<Base64>,
    pub MDI_UNPLUG_TIMESTAMP: Option<Base64>,
    pub MDI_UNPLUG_SPORADIC: Option<Base64>,
    pub MDI_REPLUG_TIMESTAMP: Option<Base64>,
    pub MDI_UNPLUG_COUNT: Option<Base64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Base64 {
    b64_value: Option<String>,
}
