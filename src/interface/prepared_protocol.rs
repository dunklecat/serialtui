use derive_builder::Builder;

#[derive(Builder, Debug)]
pub struct PreparedProtocol {
    pub path: String,
    pub baud_rate: u32,
    pub data_bits: serialport::DataBits,
    pub flow_control: serialport::FlowControl,
    pub parity: serialport::Parity,
    pub timeout: std::time::Duration,
    pub stop_bits: serialport::StopBits,
}

impl PreparedProtocol {
    pub fn builder() -> PreparedProtocolBuilder {
        PreparedProtocolBuilder::create_empty()
    }
}

impl PreparedProtocolBuilder {
    pub fn build_from_raw(
        raw_protocol: super::raw_protocol::RawProtocol,
        tty_path: String,
    ) -> PreparedProtocol {
        let path = tty_path;
        let baud_rate = raw_protocol.baud_rate;

        let data_bits = match raw_protocol.data_bits {
            super::raw_protocol::DataBits::Five => serialport::DataBits::Five,
            super::raw_protocol::DataBits::Six => serialport::DataBits::Six,
            super::raw_protocol::DataBits::Seven => serialport::DataBits::Seven,
            super::raw_protocol::DataBits::Eight => serialport::DataBits::Eight,
        };

        let flow_control = match raw_protocol.flow_control {
            super::raw_protocol::FlowControl::None => serialport::FlowControl::None,
            super::raw_protocol::FlowControl::Hardware => serialport::FlowControl::Hardware,
            super::raw_protocol::FlowControl::Software => serialport::FlowControl::Software,
        };

        let parity = match raw_protocol.parity {
            super::raw_protocol::Parity::None => serialport::Parity::None,
            super::raw_protocol::Parity::Odd => serialport::Parity::Odd,
            super::raw_protocol::Parity::Even => serialport::Parity::Even,
        };

        let timeout = std::time::Duration::from_secs(raw_protocol.timeout);

        let stop_bits = match raw_protocol.stop_bits {
            super::raw_protocol::StopBits::One => serialport::StopBits::One,
            super::raw_protocol::StopBits::Two => serialport::StopBits::Two,
        };

        PreparedProtocolBuilder::create_empty()
            .path(path)
            .baud_rate(baud_rate)
            .data_bits(data_bits)
            .flow_control(flow_control)
            .parity(parity)
            .timeout(timeout)
            .stop_bits(stop_bits)
            .build()
            .unwrap()
    }
}
