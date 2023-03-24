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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Fields>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "UPPERCASE", deserialize = "UPPERCASE"))]
pub struct Fields {
    //---------------Generic Data--------------------------
    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub gprmc_valid: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub gps_speed: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub gps_dir: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub gps_altitude: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub dio_ignition: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub batt: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub gprs_header: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub rssi: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub mdi_ext_batt_present: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub odo_partial_km: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub odo_full: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub dio_alarm: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub driver_id: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub dio_in_tor: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub gps_hdop: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub gps_pdop: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub gps_vdop: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub batt_temp: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub case_temp: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub modem_network_operator: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub gps_average_pdop_status: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub mdi_last_valid_gps_latitude: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub mdi_last_valid_gps_longitude: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub area_list: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub gps_fixed_sat_num: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub mvt_state: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub boot_reason: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub shutdown_type_and_reason: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub batt_volt: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub tx_kbytes: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub rx_kbytes: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub batt_warmup: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub batt_charging: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub dio_out_tor: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub modem_sim_iccid: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub modem_sim_imsi: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub serial_ppp_state: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub board_id: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub event: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub mdi_ext_batt_low: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub mdi_ext_batt_voltage: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub odo_partial_meter: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub odo_full_meter: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub mdi_zone_country: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub mdi_zone_state: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub mdi_vehicle_state_mvt: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub mdi_gps_antenna: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub mdi_dio_ain1: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub mdi_fuel_type: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub mdi_timezone: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub mdi_night_and_day: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub mdi_stat_global_trip_distance: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub mdi_stat_global_trip_fuel_consumed: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub mdi_serial_number: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub mdi_software_version: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub mdi_unplug_duration: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub mdi_unplug_timestamp: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub mdi_unplug_sporadic: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub mdi_replug_timestamp: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub mdi_unplug_count: Base64,

    //---------------OBD Data--------------------------
    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub obd_connected_protocol: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub obd_fuel_level_ratio: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub mdi_obd_fuel_level: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub mdi_obd_engine_load: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub mdi_obd_fuel_pressure: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub mdi_obd_throttle_position: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub mdi_obd_engine_oil_temp: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub mdi_obd_transmission_oil_temp: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub mdi_obd_engine_oil_pressure: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub mdi_obd_cruise_control: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub mdi_obd_state_water_in_fuel: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub mdi_obd_engine_coolant_level: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub mdi_obd_engine_coolant_temp: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub mdi_obd_engine_coolant_pressure: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub mdi_obd_fuel_delivery_pressure: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub mdi_obd_parking_brake_switch: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub mdi_obd_brake_application_pressure: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub mdi_obd_brake_pedal_status: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub mdi_obd_stack_name: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub mdi_obd_brake_pedal_position: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub mdi_obd_vin_alt: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub mdi_obd_fuel_type: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub mdi_obd_monitor_status: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub mdi_obd_vin_hash: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub mdi_obd_pid_1: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub mdi_obd_pid_2: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub mdi_obd_pid_3: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub mdi_obd_pid_4: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub mdi_obd_pid_5: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub mdi_obd_squish_vin: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub mdi_obd_mileage_meters: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub mdi_obd_speed: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub mdi_obd_rpm: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub mdi_obd_fuel: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub mdi_obd_vin: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub mdi_obd_mileage: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub mdi_obd_hev_engine_mode: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub mdi_obd_hev_engine_combustion_mode_time: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub mdi_obd_hev_engine_electric_mode_time: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub mdi_obd_hev_charging_state: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub mdi_obd_hev_battery_voltage: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub mdi_obd_hev_battery_current: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub mdi_obd_hev_battery_state_of_charge: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub mdi_obd_hev_state_supported: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub mdi_obd_co2_emissions: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub obd_supported_pids_00: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub mdi_obd_distance_since_dtc_cleared: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub mdi_obd_time_since_dtc_cleared: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub mdi_ext_volt_snapshot: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub mdi_obd_ambient_air_temperature: Base64,

    #[serde(skip_serializing_if = "is_empty::is_empty")]
    pub mdi_obd_barometric_pressure: Base64,
}

#[derive(Debug, Serialize, Deserialize, IsEmpty)]
pub struct Base64 {
    #[serde(skip_serializing_if = "Option::is_none")]
    b64_value: Option<String>,
}

impl From<&Document> for Fields {
    fn from(doc: &Document) -> Fields {
        let field = Fields {
            gprmc_valid: Base64::from(match doc.get_document("gprmc_valid") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),

            gps_speed: Base64::from(match doc.get_document("gps_speed") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            gps_dir: Base64::from(match doc.get_document("gps_dir") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            gps_altitude: Base64::from(match doc.get_document("gps_altitude") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            dio_ignition: Base64::from(match doc.get_document("dio_ignition") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            batt: Base64::from(match doc.get_document("batt") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            gprs_header: Base64::from(match doc.get_document("gprs_header") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            rssi: Base64::from(match doc.get_document("rssi") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            mdi_ext_batt_present: Base64::from(match doc.get_document("mdi_ext_batt_present") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            odo_partial_km: Base64::from(match doc.get_document("odo_partial_km") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            odo_full: Base64::from(match doc.get_document("odo_full") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            dio_alarm: Base64::from(match doc.get_document("dio_alarm") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            driver_id: Base64::from(match doc.get_document("driver_id") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            dio_in_tor: Base64::from(match doc.get_document("dio_in_tor") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            gps_hdop: Base64::from(match doc.get_document("gps_hdop") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            gps_pdop: Base64::from(match doc.get_document("gps_pdop") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            gps_vdop: Base64::from(match doc.get_document("gps_vdop") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            batt_temp: Base64::from(match doc.get_document("batt_temp") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            case_temp: Base64::from(match doc.get_document("case_temp") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            modem_network_operator: Base64::from(
                match doc.get_document("modem_network_operator") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            ),
            gps_average_pdop_status: Base64::from(
                match doc.get_document("gps_average_pdop_status") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            ),
            mdi_last_valid_gps_latitude: Base64::from(
                match doc.get_document("mdi_last_valid_gps_latitude") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            ),
            mdi_last_valid_gps_longitude: Base64::from(
                match doc.get_document("mdi_last_valid_gps_longitude") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            ),
            area_list: Base64::from(match doc.get_document("area_list") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            gps_fixed_sat_num: Base64::from(match doc.get_document("gps_fixed_sat_num") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            mvt_state: Base64::from(match doc.get_document("mvt_state") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            boot_reason: Base64::from(match doc.get_document("boot_reason") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            shutdown_type_and_reason: Base64::from(
                match doc.get_document("shutdown_type_and_reason") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            ),
            batt_volt: Base64::from(match doc.get_document("batt_volt") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            tx_kbytes: Base64::from(match doc.get_document("tx_kbytes") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            rx_kbytes: Base64::from(match doc.get_document("rx_kbytes") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            batt_warmup: Base64::from(match doc.get_document("batt_warmup") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            batt_charging: Base64::from(match doc.get_document("batt_charging") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            dio_out_tor: Base64::from(match doc.get_document("dio_out_tor") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            modem_sim_iccid: Base64::from(match doc.get_document("modem_sim_iccid") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            modem_sim_imsi: Base64::from(match doc.get_document("modem_sim_imsi") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            serial_ppp_state: Base64::from(match doc.get_document("serial_ppp_state") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            board_id: Base64::from(match doc.get_document("board_id") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            event: Base64::from(match doc.get_document("event") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            mdi_ext_batt_low: Base64::from(match doc.get_document("mdi_ext_batt_low") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            mdi_ext_batt_voltage: Base64::from(match doc.get_document("mdi_ext_batt_voltage") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            odo_partial_meter: Base64::from(match doc.get_document("odo_partial_meter") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            odo_full_meter: Base64::from(match doc.get_document("odo_full_meter") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            mdi_zone_country: Base64::from(match doc.get_document("mdi_zone_country") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            mdi_zone_state: Base64::from(match doc.get_document("mdi_zone_state") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            mdi_vehicle_state_mvt: Base64::from(match doc.get_document("mdi_vehicle_state_mvt") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            mdi_gps_antenna: Base64::from(match doc.get_document("mdi_gps_antenna") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            mdi_dio_ain1: Base64::from(match doc.get_document("mdi_dio_ain1") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            mdi_fuel_type: Base64::from(match doc.get_document("mdi_fuel_type") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            mdi_timezone: Base64::from(match doc.get_document("mdi_timezone") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            mdi_night_and_day: Base64::from(match doc.get_document("mdi_night_and_day") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            mdi_stat_global_trip_distance: Base64::from(
                match doc.get_document("mdi_stat_global_trip_distance") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            ),
            mdi_stat_global_trip_fuel_consumed: Base64::from(
                match doc.get_document("mdi_stat_global_trip_fuel_consumed") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            ),
            mdi_serial_number: Base64::from(match doc.get_document("mdi_serial_number") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            mdi_software_version: Base64::from(match doc.get_document("mdi_software_version") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            mdi_unplug_duration: Base64::from(match doc.get_document("mdi_unplug_duration") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            mdi_unplug_timestamp: Base64::from(match doc.get_document("mdi_unplug_timestamp") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            mdi_unplug_sporadic: Base64::from(match doc.get_document("mdi_unplug_sporadic") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            mdi_replug_timestamp: Base64::from(match doc.get_document("mdi_replug_timestamp") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            mdi_unplug_count: Base64::from(match doc.get_document("mdi_unplug_count") {
                Ok(e) => Some(e),
                Err(e) => None,
            }), //------------------OBD Data--------------------------------------
            obd_connected_protocol: Base64::from(
                match doc.get_document("obd_connected_protocol") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            ),
            obd_fuel_level_ratio: Base64::from(match doc.get_document("obd_fuel_level_ratio") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            mdi_obd_fuel_level: Base64::from(match doc.get_document("mdi_obd_fuel_level") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            mdi_obd_engine_load: Base64::from(match doc.get_document("mdi_obd_engine_load") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            mdi_obd_fuel_pressure: Base64::from(match doc.get_document("mdi_obd_fuel_pressure") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            mdi_obd_throttle_position: Base64::from(
                match doc.get_document("mdi_obd_throttle_position") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            ),
            mdi_obd_engine_oil_temp: Base64::from(
                match doc.get_document("mdi_obd_engine_oil_temp") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            ),
            mdi_obd_transmission_oil_temp: Base64::from(
                match doc.get_document("mdi_obd_transmission_oil_temp") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            ),
            mdi_obd_engine_oil_pressure: Base64::from(
                match doc.get_document("mdi_obd_engine_oil_pressure") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            ),
            mdi_obd_cruise_control: Base64::from(
                match doc.get_document("mdi_obd_cruise_control") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            ),
            mdi_obd_state_water_in_fuel: Base64::from(
                match doc.get_document("mdi_obd_state_water_in_fuel") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            ),
            mdi_obd_engine_coolant_level: Base64::from(
                match doc.get_document("mdi_obd_engine_coolant_level") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            ),
            mdi_obd_engine_coolant_temp: Base64::from(
                match doc.get_document("mdi_obd_engine_coolant_temp") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            ),
            mdi_obd_engine_coolant_pressure: Base64::from(
                match doc.get_document("mdi_obd_engine_coolant_pressure") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            ),
            mdi_obd_fuel_delivery_pressure: Base64::from(
                match doc.get_document("mdi_obd_fuel_delivery_pressure") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            ),
            mdi_obd_parking_brake_switch: Base64::from(
                match doc.get_document("mdi_obd_parking_brake_switch") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            ),
            mdi_obd_brake_application_pressure: Base64::from(
                match doc.get_document("mdi_obd_brake_application_pressure") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            ),
            mdi_obd_brake_pedal_status: Base64::from(
                match doc.get_document("mdi_obd_brake_pedal_status") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            ),
            mdi_obd_stack_name: Base64::from(match doc.get_document("mdi_obd_stack_name") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            mdi_obd_brake_pedal_position: Base64::from(
                match doc.get_document("mdi_obd_brake_pedal_position") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            ),
            mdi_obd_vin_alt: Base64::from(match doc.get_document("mdi_obd_vin_alt") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            mdi_obd_fuel_type: Base64::from(match doc.get_document("mdi_obd_fuel_type") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            mdi_obd_monitor_status: Base64::from(
                match doc.get_document("mdi_obd_monitor_status") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            ),
            mdi_obd_vin_hash: Base64::from(match doc.get_document("mdi_obd_vin_hash") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            mdi_obd_pid_1: Base64::from(match doc.get_document("mdi_obd_pid_1") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            mdi_obd_pid_2: Base64::from(match doc.get_document("mdi_obd_pid_2") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            mdi_obd_pid_3: Base64::from(match doc.get_document("mdi_obd_pid_3") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            mdi_obd_pid_4: Base64::from(match doc.get_document("mdi_obd_pid_4") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            mdi_obd_pid_5: Base64::from(match doc.get_document("mdi_obd_pid_5") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            mdi_obd_squish_vin: Base64::from(match doc.get_document("mdi_obd_squish_vin") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            mdi_obd_mileage_meters: Base64::from(
                match doc.get_document("mdi_obd_mileage_meters") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            ),
            mdi_obd_speed: Base64::from(match doc.get_document("mdi_obd_speed") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            mdi_obd_rpm: Base64::from(match doc.get_document("mdi_obd_rpm") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            mdi_obd_fuel: Base64::from(match doc.get_document("mdi_obd_fuel") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            mdi_obd_vin: Base64::from(match doc.get_document("mdi_obd_vin") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            mdi_obd_mileage: Base64::from(match doc.get_document("mdi_obd_mileage") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            mdi_obd_hev_engine_mode: Base64::from(
                match doc.get_document("mdi_obd_hev_engine_mode") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            ),
            mdi_obd_hev_engine_combustion_mode_time: Base64::from(
                match doc.get_document("mdi_obd_hev_engine_combustion_mode_time") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            ),
            mdi_obd_hev_engine_electric_mode_time: Base64::from(
                match doc.get_document("mdi_obd_hev_engine_electric_mode_time") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            ),
            mdi_obd_hev_charging_state: Base64::from(
                match doc.get_document("mdi_obd_hev_charging_state") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            ),
            mdi_obd_hev_battery_voltage: Base64::from(
                match doc.get_document("mdi_obd_hev_battery_voltage") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            ),
            mdi_obd_hev_battery_current: Base64::from(
                match doc.get_document("mdi_obd_hev_battery_current") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            ),
            mdi_obd_hev_battery_state_of_charge: Base64::from(
                match doc.get_document("mdi_obd_hev_battery_state_of_charge") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            ),
            mdi_obd_hev_state_supported: Base64::from(
                match doc.get_document("mdi_obd_hev_state_supported") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            ),
            mdi_obd_co2_emissions: Base64::from(match doc.get_document("mdi_obd_co2_emissions") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            obd_supported_pids_00: Base64::from(match doc.get_document("obd_supported_pids_00") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            mdi_obd_distance_since_dtc_cleared: Base64::from(
                match doc.get_document("mdi_obd_distance_since_dtc_cleared") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            ),
            mdi_obd_time_since_dtc_cleared: Base64::from(
                match doc.get_document("mdi_obd_time_since_dtc_cleared") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            ),
            mdi_ext_volt_snapshot: Base64::from(match doc.get_document("mdi_ext_volt_snapshot") {
                Ok(e) => Some(e),
                Err(e) => None,
            }),
            mdi_obd_ambient_air_temperature: Base64::from(
                match doc.get_document("mdi_obd_ambient_air_temperature") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            ),
            mdi_obd_barometric_pressure: Base64::from(
                match doc.get_document("mdi_obd_barometric_pressure") {
                    Ok(e) => Some(e),
                    Err(e) => None,
                },
            ),
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
