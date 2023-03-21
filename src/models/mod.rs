// models/mod.rs

use bson::Document;
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
    pub loc: Option<[f64; 2]>,
    pub location: Option<[f64; 2]>,
    pub connection_id: i64,
    pub index: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Fields>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

// Only Generic Fields
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "UPPERCASE", deserialize = "UPPERCASE"))]
pub struct Fields {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gprmc_valid: Option<Base64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gps_speed: Option<Base64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gps_dir: Option<Base64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gps_altitude: Option<Base64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dio_ignition: Option<Base64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub batt: Option<Base64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gprs_header: Option<Base64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rssi: Option<Base64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mdi_ext_batt_present: Option<Base64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub odo_partial_km: Option<Base64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub odo_full: Option<Base64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dio_alarm: Option<Base64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver_id: Option<Base64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dio_in_tor: Option<Base64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gps_hdop: Option<Base64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gps_pdop: Option<Base64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gps_vdop: Option<Base64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub batt_temp: Option<Base64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub case_temp: Option<Base64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub modem_network_operator: Option<Base64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gps_average_pdop_status: Option<Base64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mdi_last_valid_gps_latitude: Option<Base64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mdi_last_valid_gps_longitude: Option<Base64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub area_list: Option<Base64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gps_fixed_sat_num: Option<Base64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mvt_state: Option<Base64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub boot_reason: Option<Base64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shutdown_type_and_reason: Option<Base64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub batt_volt: Option<Base64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_kbytes: Option<Base64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rx_kbytes: Option<Base64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub batt_warmup: Option<Base64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub batt_charging: Option<Base64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dio_out_tor: Option<Base64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub modem_sim_iccid: Option<Base64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub modem_sim_imsi: Option<Base64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_ppp_state: Option<Base64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub board_id: Option<Base64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<Base64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mdi_ext_batt_low: Option<Base64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mdi_ext_batt_voltage: Option<Base64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub odo_partial_meter: Option<Base64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub odo_full_meter: Option<Base64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mdi_zone_country: Option<Base64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mdi_zone_state: Option<Base64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mdi_vehicle_state_mvt: Option<Base64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mdi_gps_antenna: Option<Base64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mdi_dio_ain1: Option<Base64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mdi_fuel_type: Option<Base64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mdi_timezone: Option<Base64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mdi_night_and_day: Option<Base64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mdi_stat_global_trip_distance: Option<Base64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mdi_stat_global_trip_fuel_consumed: Option<Base64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mdi_serial_number: Option<Base64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mdi_software_version: Option<Base64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mdi_unplug_duration: Option<Base64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mdi_unplug_timestamp: Option<Base64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mdi_unplug_sporadic: Option<Base64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mdi_replug_timestamp: Option<Base64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mdi_unplug_count: Option<Base64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Base64 {
    #[serde(skip_serializing_if = "Option::is_none")]
    b64_value: Option<String>,
}

impl From<&Document> for Fields {
    fn from(doc: &Document) -> Fields {
        let field = Fields {
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
            })),
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
