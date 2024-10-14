
pub struct PreparedSerial {
    pub tty: Option<Box<dyn serialport::SerialPort>>,
    pub unix_socket: String,
}
