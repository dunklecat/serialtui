use config::ConfigBuilder;
use config::Mode;
use lazy_static::lazy_static;
use prepared_protocol::PreparedProtocol;
use prepared_protocol::PreparedProtocolBuilder;
use prepared_serial::PreparedSerial;
use serialport::SerialPort;
use std::error::Error;
use std::sync::Arc;
use std::sync::Mutex;
use tokio::sync::RwLock;

pub(crate) mod config;
pub(crate) mod prepared_protocol;
pub(crate) mod prepared_serial;
pub(crate) mod raw_protocol;
pub(crate) mod raw_serial;

lazy_static! {
    pub static ref TTY: Arc<Mutex<Option<Box<dyn SerialPort>>>> =
        Arc::new(Mutex::new(Default::default()));
    pub static ref SOCKET: RwLock<String> = RwLock::new(Default::default());
    pub static ref MODE: RwLock<Mode> = RwLock::new(Default::default());
}

pub async fn open_interface(path: &str) -> Result<String, Box<dyn Error>> {
    let config = ConfigBuilder::new().config_path(path).build();
    let prepared_protocol =
        PreparedProtocolBuilder::build_from_raw(config.protocol, config.serial.tty);
    let prepared_serial: PreparedSerial = PreparedSerial {
        tty: Some(build_serialport(prepared_protocol)),
        unix_socket: config.serial.unix_socket,
    };

    *SOCKET.write().await = prepared_serial.unix_socket;
    *TTY.lock().unwrap() = prepared_serial.tty;
    *MODE.write().await = config.mode;

    let _ = std::fs::remove_file(&SOCKET.read().await.as_str());

    let socket = SOCKET.read().await.clone();

    Ok(socket)
}

fn build_serialport(interface: PreparedProtocol) -> Box<dyn SerialPort> {
    log::debug!("Connecting to {}", interface.path);
    log::debug!("{:?}", interface);

    serialport::new(interface.path, interface.baud_rate)
        .data_bits(interface.data_bits)
        .flow_control(interface.flow_control)
        .parity(interface.parity)
        .timeout(interface.timeout)
        .stop_bits(interface.stop_bits)
        .open()
        .unwrap()
}
