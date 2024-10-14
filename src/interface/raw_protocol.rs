use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub enum DataBits {
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum FlowControl {
    None,
    Hardware,
    Software,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Parity {
    None,
    Odd,
    Even,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum StopBits {
    One = 1,
    Two = 2,
}

#[derive(Deserialize, Serialize)]
pub struct RawProtocol {
    #[serde(default = "RawProtocol::default_baud_rate")]
    pub baud_rate: u32,
    #[serde(default = "RawProtocol::default_data_bits")]
    pub data_bits: DataBits,
    #[serde(default = "RawProtocol::default_flow_control")]
    pub flow_control: FlowControl,
    #[serde(default = "RawProtocol::default_parity")]
    pub parity: Parity,
    #[serde(default = "RawProtocol::default_timeout")]
    pub timeout: u64,
    #[serde(default = "RawProtocol::default_stop_bits")]
    pub stop_bits: StopBits,
}

impl Default for RawProtocol {
    fn default() -> Self {
        RawProtocol {
            baud_rate: RawProtocol::default_baud_rate(),
            data_bits: RawProtocol::default_data_bits(),
            flow_control: RawProtocol::default_flow_control(),
            parity: RawProtocol::default_parity(),
            timeout: RawProtocol::default_timeout(),
            stop_bits: RawProtocol::default_stop_bits(),
        }
    }
}

impl RawProtocol {
    fn default_baud_rate() -> u32 {
        let d = 9600;
        log::debug!("RawProtocol -> using default baud rate: {}", d);
        d
    }
    fn default_data_bits() -> DataBits {
        let d = DataBits::Eight;
        log::debug!("RawProtocol -> using default data bits: {:?}", d);
        d
    }
    fn default_flow_control() -> FlowControl {
        let d = FlowControl::None;
        log::debug!("RawProtocol -> using default flow control: {:?}", d);
        d
    }
    fn default_parity() -> Parity {
        let d = Parity::Even;
        log::debug!("RawProtocol -> using default parity: {:?}", d);
        d
    }
    fn default_timeout() -> u64 {
        let d = 30;
        log::debug!("RawProtocol -> using default timeout: {}", d);
        d
    }
    fn default_stop_bits() -> StopBits {
        let d = StopBits::One;
        log::debug!("RawProtocol -> using default timeout: {:?}", d);
        d
    }
}
