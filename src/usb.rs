use serialport::SerialPort;

pub struct USB {
    port: Box<dyn SerialPort>,
}

impl USB {
    pub fn new() -> Self {
        Self {
            port: serialport::new("/dev/tty.usbmodemXPSF1", 9600)
                .timeout(std::time::Duration::from_secs(1))
                .open()
                .expect("A macOS Spesific Error Has Occured / Failed to Start Interface"),
        }
    }

    pub fn send_instruction(&mut self, command: [u8; 8]) {
        self.port
            .write_all(&command)
            .expect("A macOS Spesific Error Has Occured / Failed to Send Instruction");
    }

    pub fn block_for_next(&mut self) {
        let mut responce = [0u8; 8];

        match self.port.read_exact(&mut responce) {
            Ok(()) if &responce == b"INSTRCTN" => return,
            Ok(()) => panic!("Bad Traffic on Interface"),
            Err(_) => panic!("Interface Failure in Backward Direction"),
        }
    }
}
