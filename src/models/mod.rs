// models/mod.rs

use bson::Document;
use is_empty::IsEmpty;
use serde::{Deserialize, Serialize};

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

    pub fields: Fields,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

trait ShouldSkip {
    fn should_skip(&self) -> bool;
}

impl<T: Default + PartialEq> ShouldSkip for Option<T> {
    fn should_skip(&self) -> bool {
        self.as_ref().map(|v| *v == T::default()).unwrap_or(true)
    }
}

#[derive(Debug, Serialize, Default, Deserialize)]
#[serde(rename_all(serialize = "UPPERCASE", deserialize = "UPPERCASE"))]
pub struct Fields {
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
        let field = Fields {
            gprmc_valid: Some(Base64::from(match doc.get_document("GPRMC_VALID") {
                Ok(e) => Some(e),
                Err(_e) => None,
            })),

            gps_speed: Some(Base64::from(match doc.get_document("GPS_SPEED") {
                Ok(e) => Some(e),
                Err(_e) => None,
            })),
            gps_dir: Some(Base64::from(match doc.get_document("GPS_DIR") {
                Ok(e) => Some(e),
                Err(_e) => None,
            })),
            gps_altitude: Some(Base64::from(match doc.get_document("GPS_ALTITUDE") {
                Ok(e) => Some(e),
                Err(_e) => None,
            })),
            dio_ignition: Some(Base64::from(match doc.get_document("DIO_IGNITION") {
                Ok(e) => Some(e),
                Err(_e) => None,
            })),
            batt: Some(Base64::from(match doc.get_document("batt") {
                Ok(e) => Some(e),
                Err(_e) => None,
            })),
            gprs_header: Some(Base64::from(match doc.get_document("GPRS_HEADER") {
                Ok(e) => Some(e),
                Err(_e) => None,
            })),
            rssi: Some(Base64::from(match doc.get_document("rssi") {
                Ok(e) => Some(e),
                Err(_e) => None,
            })),
            mdi_ext_batt_present: Some(Base64::from(
                match doc.get_document("MDI_EXT_BATT_PRESENT") {
                    Ok(e) => Some(e),
                    Err(_e) => None,
                },
            )),
            odo_partial_km: Some(Base64::from(match doc.get_document("ODO_PARTIAL_KM") {
                Ok(e) => Some(e),
                Err(_e) => None,
            })),
            odo_full: Some(Base64::from(match doc.get_document("ODO_FULL") {
                Ok(e) => Some(e),
                Err(_e) => None,
            })),
            dio_alarm: Some(Base64::from(match doc.get_document("DIO_ALARM") {
                Ok(e) => Some(e),
                Err(_e) => None,
            })),
            driver_id: Some(Base64::from(match doc.get_document("DRIVER_ID") {
                Ok(e) => Some(e),
                Err(_e) => None,
            })),
            dio_in_tor: Some(Base64::from(match doc.get_document("DIO_IN_TOR") {
                Ok(e) => Some(e),
                Err(_e) => None,
            })),
            gps_hdop: Some(Base64::from(match doc.get_document("GPS_HDOP") {
                Ok(e) => Some(e),
                Err(_e) => None,
            })),
            gps_pdop: Some(Base64::from(match doc.get_document("GPS_PDOP") {
                Ok(e) => Some(e),
                Err(_e) => None,
            })),
            gps_vdop: Some(Base64::from(match doc.get_document("GPS_VDOP") {
                Ok(e) => Some(e),
                Err(_e) => None,
            })),
            batt_temp: Some(Base64::from(match doc.get_document("BATT_TEMP") {
                Ok(e) => Some(e),
                Err(_e) => None,
            })),
            case_temp: Some(Base64::from(match doc.get_document("CASE_TEMP") {
                Ok(e) => Some(e),
                Err(_e) => None,
            })),
            modem_network_operator: Some(Base64::from(
                match doc.get_document("MODEM_NETWORK_OPERATOR") {
                    Ok(e) => Some(e),
                    Err(_e) => None,
                },
            )),
            gps_average_pdop_status: Some(Base64::from(
                match doc.get_document("GPS_AVERAGE_PDOP_STATUS") {
                    Ok(e) => Some(e),
                    Err(_e) => None,
                },
            )),
            mdi_last_valid_gps_latitude: Some(Base64::from(
                match doc.get_document("MDI_LAST_VALID_GPS_LATITUDE") {
                    Ok(e) => Some(e),
                    Err(_e) => None,
                },
            )),
            mdi_last_valid_gps_longitude: Some(Base64::from(
                match doc.get_document("MDI_LAST_VALID_GPS_LONGITUDE") {
                    Ok(e) => Some(e),
                    Err(_e) => None,
                },
            )),
            area_list: Some(Base64::from(match doc.get_document("AREA_LIST") {
                Ok(e) => Some(e),
                Err(_e) => None,
            })),
            gps_fixed_sat_num: Some(Base64::from(match doc.get_document("GPS_FIXED_SAT_NUM") {
                Ok(e) => Some(e),
                Err(_e) => None,
            })),
            mvt_state: Some(Base64::from(match doc.get_document("MVT_STATE") {
                Ok(e) => Some(e),
                Err(_e) => None,
            })),
            boot_reason: Some(Base64::from(match doc.get_document("BOOT_REASON") {
                Ok(e) => Some(e),
                Err(_e) => None,
            })),
            shutdown_type_and_reason: Some(Base64::from(
                match doc.get_document("SHUTDOWN_TYPE_AND_REASON") {
                    Ok(e) => Some(e),
                    Err(_e) => None,
                },
            )),
            batt_volt: Some(Base64::from(match doc.get_document("BATT_VOLT") {
                Ok(e) => Some(e),
                Err(_e) => None,
            })),
            tx_kbytes: Some(Base64::from(match doc.get_document("TX_KBYTES") {
                Ok(e) => Some(e),
                Err(_e) => None,
            })),
            rx_kbytes: Some(Base64::from(match doc.get_document("RX_KBYTES") {
                Ok(e) => Some(e),
                Err(_e) => None,
            })),
            batt_warmup: Some(Base64::from(match doc.get_document("BATT_WARMUP") {
                Ok(e) => Some(e),
                Err(_e) => None,
            })),
            batt_charging: Some(Base64::from(match doc.get_document("BATT_CHARGING") {
                Ok(e) => Some(e),
                Err(_e) => None,
            })),
            dio_out_tor: Some(Base64::from(match doc.get_document("DIO_OUT_TOR") {
                Ok(e) => Some(e),
                Err(_e) => None,
            })),
            modem_sim_iccid: Some(Base64::from(match doc.get_document("MODEM_SIM_ICCID") {
                Ok(e) => Some(e),
                Err(_e) => None,
            })),
            modem_sim_imsi: Some(Base64::from(match doc.get_document("MODEM_SIM_IMSI") {
                Ok(e) => Some(e),
                Err(_e) => None,
            })),
            serial_ppp_state: Some(Base64::from(match doc.get_document("SERIAL_PPP_STATE") {
                Ok(e) => Some(e),
                Err(_e) => None,
            })),
            board_id: Some(Base64::from(match doc.get_document("BOARD_ID") {
                Ok(e) => Some(e),
                Err(_e) => None,
            })),
            event: Some(Base64::from(match doc.get_document("event") {
                Ok(e) => Some(e),
                Err(_e) => None,
            })),
            mdi_ext_batt_low: Some(Base64::from(match doc.get_document("MDI_EXT_BATT_LOW") {
                Ok(e) => Some(e),
                Err(_e) => None,
            })),
            mdi_ext_batt_voltage: Some(Base64::from(
                match doc.get_document("MDI_EXT_BATT_VOLTAGE") {
                    Ok(e) => Some(e),
                    Err(_e) => None,
                },
            )),
            odo_partial_meter: Some(Base64::from(match doc.get_document("ODO_PARTIAL_METER") {
                Ok(e) => Some(e),
                Err(_e) => None,
            })),
            odo_full_meter: Some(Base64::from(match doc.get_document("ODO_FULL_METER") {
                Ok(e) => Some(e),
                Err(_e) => None,
            })),
            mdi_zone_country: Some(Base64::from(match doc.get_document("MDI_ZONE_COUNTRY") {
                Ok(e) => Some(e),
                Err(_e) => None,
            })),
            mdi_zone_state: Some(Base64::from(match doc.get_document("MDI_ZONE_STATE") {
                Ok(e) => Some(e),
                Err(_e) => None,
            })),
            mdi_vehicle_state_mvt: Some(Base64::from(
                match doc.get_document("MDI_VEHICLE_STATE_MVT") {
                    Ok(e) => Some(e),
                    Err(_e) => None,
                },
            )),
            mdi_gps_antenna: Some(Base64::from(match doc.get_document("MDI_GPS_ANTENNA") {
                Ok(e) => Some(e),
                Err(_e) => None,
            })),
            mdi_dio_ain1: Some(Base64::from(match doc.get_document("MDI_DIO_AIN1") {
                Ok(e) => Some(e),
                Err(_e) => None,
            })),
            mdi_fuel_type: Some(Base64::from(match doc.get_document("MDI_FUEL_TYPE") {
                Ok(e) => Some(e),
                Err(_e) => None,
            })),
            mdi_timezone: Some(Base64::from(match doc.get_document("MDI_TIMEZONE") {
                Ok(e) => Some(e),
                Err(_e) => None,
            })),
            mdi_night_and_day: Some(Base64::from(match doc.get_document("MDI_NIGHT_AND_DAY") {
                Ok(e) => Some(e),
                Err(_e) => None,
            })),
            mdi_stat_global_trip_distance: Some(Base64::from(
                match doc.get_document("MDI_STAT_GLOBAL_TRIP_DISTANCE") {
                    Ok(e) => Some(e),
                    Err(_e) => None,
                },
            )),
            mdi_stat_global_trip_fuel_consumed: Some(Base64::from(
                match doc.get_document("MDI_STAT_GLOBAL_TRIP_FUEL_CONSUMED") {
                    Ok(e) => Some(e),
                    Err(_e) => None,
                },
            )),
            mdi_serial_number: Some(Base64::from(match doc.get_document("MDI_SERIAL_NUMBER") {
                Ok(e) => Some(e),
                Err(_e) => None,
            })),
            mdi_software_version: Some(Base64::from(
                match doc.get_document("MDI_SOFTWARE_VERSION") {
                    Ok(e) => Some(e),
                    Err(_e) => None,
                },
            )),
            mdi_unplug_duration: Some(Base64::from(
                match doc.get_document("MDI_UNPLUG_DURATION") {
                    Ok(e) => Some(e),
                    Err(_e) => None,
                },
            )),
            mdi_unplug_timestamp: Some(Base64::from(
                match doc.get_document("MDI_UNPLUG_TIMESTAMP") {
                    Ok(e) => Some(e),
                    Err(_e) => None,
                },
            )),
            mdi_unplug_sporadic: Some(Base64::from(
                match doc.get_document("MDI_UNPLUG_SPORADIC") {
                    Ok(e) => Some(e),
                    Err(_e) => None,
                },
            )),
            mdi_replug_timestamp: Some(Base64::from(
                match doc.get_document("MDI_REPLUG_TIMESTAMP") {
                    Ok(e) => Some(e),
                    Err(_e) => None,
                },
            )),
            mdi_unplug_count: Some(Base64::from(match doc.get_document("MDI_UNPLUG_COUNT") {
                Ok(e) => Some(e),
                Err(_e) => None,
            })), //------------------OBD Data--------------------------------------
            obd_connected_protocol: Some(Base64::from(
                match doc.get_document("OBD_CONNECTED_PROTOCOL") {
                    Ok(e) => Some(e),
                    Err(_e) => None,
                },
            )),
            obd_fuel_level_ratio: Some(Base64::from(
                match doc.get_document("OBD_FUEL_LEVEL_RATIO") {
                    Ok(e) => Some(e),
                    Err(_e) => None,
                },
            )),
            mdi_obd_fuel_level: Some(Base64::from(match doc.get_document("MDI_OBD_FUEL_LEVEL") {
                Ok(e) => Some(e),
                Err(_e) => None,
            })),
            mdi_obd_engine_load: Some(Base64::from(
                match doc.get_document("MDI_OBD_ENGINE_LOAD") {
                    Ok(e) => Some(e),
                    Err(_e) => None,
                },
            )),
            mdi_obd_fuel_pressure: Some(Base64::from(
                match doc.get_document("MDI_OBD_FUEL_PRESSURE") {
                    Ok(e) => Some(e),
                    Err(_e) => None,
                },
            )),
            mdi_obd_throttle_position: Some(Base64::from(
                match doc.get_document("MDI_OBD_THROTTLE_POSITION") {
                    Ok(e) => Some(e),
                    Err(_e) => None,
                },
            )),
            mdi_obd_engine_oil_temp: Some(Base64::from(
                match doc.get_document("MDI_OBD_ENGINE_OIL_TEMP") {
                    Ok(e) => Some(e),
                    Err(_e) => None,
                },
            )),
            mdi_obd_transmission_oil_temp: Some(Base64::from(
                match doc.get_document("MDI_OBD_TRANSMISSION_OIL_TEMP") {
                    Ok(e) => Some(e),
                    Err(_e) => None,
                },
            )),
            mdi_obd_engine_oil_pressure: Some(Base64::from(
                match doc.get_document("MDI_OBD_ENGINE_OIL_PRESSURE") {
                    Ok(e) => Some(e),
                    Err(_e) => None,
                },
            )),
            mdi_obd_cruise_control: Some(Base64::from(
                match doc.get_document("MDI_OBD_CRUISE_CONTROL") {
                    Ok(e) => Some(e),
                    Err(_e) => None,
                },
            )),
            mdi_obd_state_water_in_fuel: Some(Base64::from(
                match doc.get_document("MDI_OBD_STATE_WATER_IN_FUEL") {
                    Ok(e) => Some(e),
                    Err(_e) => None,
                },
            )),
            mdi_obd_engine_coolant_level: Some(Base64::from(
                match doc.get_document("MDI_OBD_ENGINE_COOLANT_LEVEL") {
                    Ok(e) => Some(e),
                    Err(_e) => None,
                },
            )),
            mdi_obd_engine_coolant_temp: Some(Base64::from(
                match doc.get_document("MDI_OBD_ENGINE_COOLANT_TEMP") {
                    Ok(e) => Some(e),
                    Err(_e) => None,
                },
            )),
            mdi_obd_engine_coolant_pressure: Some(Base64::from(
                match doc.get_document("MDI_OBD_ENGINE_COOLANT_PRESSURE") {
                    Ok(e) => Some(e),
                    Err(_e) => None,
                },
            )),
            mdi_obd_fuel_delivery_pressure: Some(Base64::from(
                match doc.get_document("MDI_OBD_FUEL_DELIVERY_PRESSURE") {
                    Ok(e) => Some(e),
                    Err(_e) => None,
                },
            )),
            mdi_obd_parking_brake_switch: Some(Base64::from(
                match doc.get_document("MDI_OBD_PARKING_BRAKE_SWITCH") {
                    Ok(e) => Some(e),
                    Err(_e) => None,
                },
            )),
            mdi_obd_brake_application_pressure: Some(Base64::from(
                match doc.get_document("MDI_OBD_BRAKE_APPLICATION_PRESSURE") {
                    Ok(e) => Some(e),
                    Err(_e) => None,
                },
            )),
            mdi_obd_brake_pedal_status: Some(Base64::from(
                match doc.get_document("MDI_OBD_BRAKE_PEDAL_STATUS") {
                    Ok(e) => Some(e),
                    Err(_e) => None,
                },
            )),
            mdi_obd_stack_name: Some(Base64::from(match doc.get_document("MDI_OBD_STACK_NAME") {
                Ok(e) => Some(e),
                Err(_e) => None,
            })),
            mdi_obd_brake_pedal_position: Some(Base64::from(
                match doc.get_document("MDI_OBD_BRAKE_PEDAL_POSITION") {
                    Ok(e) => Some(e),
                    Err(_e) => None,
                },
            )),
            mdi_obd_vin_alt: Some(Base64::from(match doc.get_document("MDI_OBD_VIN_ALT") {
                Ok(e) => Some(e),
                Err(_e) => None,
            })),
            mdi_obd_fuel_type: Some(Base64::from(match doc.get_document("MDI_OBD_FUEL_TYPE") {
                Ok(e) => Some(e),
                Err(_e) => None,
            })),
            mdi_obd_monitor_status: Some(Base64::from(
                match doc.get_document("MDI_OBD_MONITOR_STATUS") {
                    Ok(e) => Some(e),
                    Err(_e) => None,
                },
            )),
            mdi_obd_vin_hash: Some(Base64::from(match doc.get_document("MDI_OBD_VIN_HASH") {
                Ok(e) => Some(e),
                Err(_e) => None,
            })),
            mdi_obd_pid_1: Some(Base64::from(match doc.get_document("MDI_OBD_PID_1") {
                Ok(e) => Some(e),
                Err(_e) => None,
            })),
            mdi_obd_pid_2: Some(Base64::from(match doc.get_document("MDI_OBD_PID_2") {
                Ok(e) => Some(e),
                Err(_e) => None,
            })),
            mdi_obd_pid_3: Some(Base64::from(match doc.get_document("MDI_OBD_PID_3") {
                Ok(e) => Some(e),
                Err(_e) => None,
            })),
            mdi_obd_pid_4: Some(Base64::from(match doc.get_document("MDI_OBD_PID_4") {
                Ok(e) => Some(e),
                Err(_e) => None,
            })),
            mdi_obd_pid_5: Some(Base64::from(match doc.get_document("MDI_OBD_PID_5") {
                Ok(e) => Some(e),
                Err(_e) => None,
            })),
            mdi_obd_squish_vin: Some(Base64::from(match doc.get_document("MDI_OBD_SQUISH_VIN") {
                Ok(e) => Some(e),
                Err(_e) => None,
            })),
            mdi_obd_mileage_meters: Some(Base64::from(
                match doc.get_document("MDI_OBD_MILEAGE_METERS") {
                    Ok(e) => Some(e),
                    Err(_e) => None,
                },
            )),
            mdi_obd_speed: Some(Base64::from(match doc.get_document("MDI_OBD_SPEED") {
                Ok(e) => Some(e),
                Err(_e) => None,
            })),
            mdi_obd_rpm: Some(Base64::from(match doc.get_document("MDI_OBD_RPM") {
                Ok(e) => Some(e),
                Err(_e) => None,
            })),
            mdi_obd_fuel: Some(Base64::from(match doc.get_document("MDI_OBD_FUEL") {
                Ok(e) => Some(e),
                Err(_e) => None,
            })),
            mdi_obd_vin: Some(Base64::from(match doc.get_document("MDI_OBD_VIN") {
                Ok(e) => Some(e),
                Err(_e) => None,
            })),
            mdi_obd_mileage: Some(Base64::from(match doc.get_document("MDI_OBD_MILEAGE") {
                Ok(e) => Some(e),
                Err(_e) => None,
            })),
            mdi_obd_hev_engine_mode: Some(Base64::from(
                match doc.get_document("MDI_OBD_HEV_ENGINE_MODE") {
                    Ok(e) => Some(e),
                    Err(_e) => None,
                },
            )),
            mdi_obd_hev_engine_combustion_mode_time: Some(Base64::from(
                match doc.get_document("MDI_OBD_HEV_ENGINE_COMBUSTION_MODE_TIME") {
                    Ok(e) => Some(e),
                    Err(_e) => None,
                },
            )),
            mdi_obd_hev_engine_electric_mode_time: Some(Base64::from(
                match doc.get_document("MDI_OBD_HEV_ENGINE_ELECTRIC_MODE_TIME") {
                    Ok(e) => Some(e),
                    Err(_e) => None,
                },
            )),
            mdi_obd_hev_charging_state: Some(Base64::from(
                match doc.get_document("MDI_OBD_HEV_CHARGING_STATE") {
                    Ok(e) => Some(e),
                    Err(_e) => None,
                },
            )),
            mdi_obd_hev_battery_voltage: Some(Base64::from(
                match doc.get_document("MDI_OBD_HEV_BATTERY_VOLTAGE") {
                    Ok(e) => Some(e),
                    Err(_e) => None,
                },
            )),
            mdi_obd_hev_battery_current: Some(Base64::from(
                match doc.get_document("MDI_OBD_HEV_BATTERY_CURRENT") {
                    Ok(e) => Some(e),
                    Err(_e) => None,
                },
            )),
            mdi_obd_hev_battery_state_of_charge: Some(Base64::from(
                match doc.get_document("MDI_OBD_HEV_BATTERY_STATE_OF_CHARGE") {
                    Ok(e) => Some(e),
                    Err(_e) => None,
                },
            )),
            mdi_obd_hev_state_supported: Some(Base64::from(
                match doc.get_document("MDI_OBD_HEV_STATE_SUPPORTED") {
                    Ok(e) => Some(e),
                    Err(_e) => None,
                },
            )),
            mdi_obd_co2_emissions: Some(Base64::from(
                match doc.get_document("MDI_OBD_CO2_EMISSIONS") {
                    Ok(e) => Some(e),
                    Err(_e) => None,
                },
            )),
            obd_supported_pids_00: Some(Base64::from(
                match doc.get_document("OBD_SUPPORTED_PIDS_00") {
                    Ok(e) => Some(e),
                    Err(_e) => None,
                },
            )),
            mdi_obd_distance_since_dtc_cleared: Some(Base64::from(
                match doc.get_document("MDI_OBD_DISTANCE_SINCE_DTC_CLEARED") {
                    Ok(e) => Some(e),
                    Err(_e) => None,
                },
            )),
            mdi_obd_time_since_dtc_cleared: Some(Base64::from(
                match doc.get_document("MDI_OBD_TIME_SINCE_DTC_CLEARED") {
                    Ok(e) => Some(e),
                    Err(_e) => None,
                },
            )),
            mdi_ext_volt_snapshot: Some(Base64::from(
                match doc.get_document("MDI_EXT_VOLT_SNAPSHOT") {
                    Ok(e) => Some(e),
                    Err(_e) => None,
                },
            )),
            mdi_obd_ambient_air_temperature: Some(Base64::from(
                match doc.get_document("MDI_OBD_AMBIENT_AIR_TEMPERATURE") {
                    Ok(e) => Some(e),
                    Err(_e) => None,
                },
            )),
            mdi_obd_barometric_pressure: Some(Base64::from(
                match doc.get_document("MDI_OBD_BAROMETRIC_PRESSURE") {
                    Ok(e) => Some(e),
                    Err(_e) => None,
                },
            )),
        };
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
