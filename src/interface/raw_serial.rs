use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct RawSerial {
    #[serde(default = "RawSerial::default_path")]
    pub tty: String,

    #[serde(default = "RawSerial::default_socket")]
    pub unix_socket: String,
}

impl Default for RawSerial {
    fn default() -> RawSerial {
        RawSerial {
            tty: String::from(""),
            unix_socket: String::from(""),
        }
    }
}

impl RawSerial {
    fn default_path() -> String {
        "/dev/ttyUSB0".to_string()
    }

    fn default_socket() -> String {
        "/tmp/wtestcontroller_socket".to_string()
    }
}
