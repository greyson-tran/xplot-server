use std::f64::consts::*;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write, stdin, stdout};
use std::mem::transmute;

fn main() {
    let mut serial_interface = USB::new("/dev/tty.usbmodem.XPS1");

    loop {
        stdout().flush().unwrap();

        let mut instruction = String::new();
        stdin()
            .read_line(&mut instruction)
            .expect("macOS Spesific Error");

        let command: Vec<&str> = instruction.trim().split_whitespace().collect();

        if command.len() != 3 {
            continue;
        }; // Ensure Propper Command Formatting

        match command[0] {
            "cntr" => {
                serial_interface.send_instruction((f32::MAX as f64, f32::MAX as f64)); // Enter Configuration Mode
                serial_interface.block_for_next(); // Wait for Synchronize

                let arg_one = match command[1].parse() {
                    Ok(f64) => f64,
                    _ => continue,
                };

                let arg_two = match command[2].parse() {
                    Ok(f64) => f64,
                    _ => continue,
                };

                serial_interface.send_instruction((arg_one, arg_two)); // CONFIG: MoveServo
                serial_interface.block_for_next(); // Wait for Synchronize
            }

            "home" => {
                serial_interface.send_instruction((f32::MAX as f64, f32::MAX as f64)); // Enter Configuration Mode
                serial_interface.block_for_next(); // Wait for Synchronize
                serial_interface.send_instruction((f32::MAX as f64, f32::MAX as f64)); // CONFIG: Home
                serial_interface.block_for_next(); // Wait for Synchronize
            }

            "gt" => {
                let arg_one = match command[1].parse() {
                    Ok(f64) => f64,
                    _ => continue,
                };

                let arg_two = match command[2].parse() {
                    Ok(f64) => f64,
                    _ => continue,
                };

                serial_interface.send_instruction((arg_one, arg_two)); // Send Instruction / Go-To / G1
                serial_interface.block_for_next(); // Wait for Synchronize
            }

            _ => continue,
        };
    }
}

// Grouped Mathematical Operations. Relevant Feature Set only.
struct Math;
impl Math {
    fn acos_rule(a: f64, b: f64, c: f64) -> f64 {
        Self::to_degrees(((a.powf(2.0) + b.powf(2.0) - c.powf(2.0)) / (2.0 * a * b)).acos())
    }

    fn cos_rule(a: f64, b: f64, angle: f64) -> f64 {
        (a.powf(2.0) + b.powf(2.0) - 2.0 * a * b * (Self::to_radians(angle)).cos()).powf(0.5)
    }

    fn to_radians(angle: f64) -> f64 {
        angle / 180.0 * PI
    }

    fn to_degrees(angle: f64) -> f64 {
        angle / PI * 180.0
    }

    fn distance(origin: (f64, f64), destination: (f64, f64)) -> f64 {
        (origin.0 - destination.0) + (origin.1 - destination.1)
    }
}

struct XPS {
    motor_length: f64,
    proximal_length: f64,
    distal_length: f64,
    pen_holder_length: f64,
}

impl XPS {
    fn new(
        motor_length: f64,
        proximal_length: f64,
        distal_length: f64,
        pen_holder_length: f64,
    ) -> Self {
        Self {
            motor_length,
            proximal_length,
            distal_length,
            pen_holder_length,
        }
    }

    fn inverse_kinematics(&self, destination: (f64, f64)) -> (f64, f64) {
        // Angle, Motor Zero / Angle, Motor One
        // Angle Notation / Motor Enum
        // Alpha: MotorLine / Shortest-Path-First
        // Beta: Shortest-Path-First / Proximal
        // Gamma: Proximal / Distal

        let gamma_zero = {
            let motor_zero_pen = Math::distance((0.0, 0.0), destination);
            Math::acos_rule(
                self.proximal_length,
                self.distal_length + self.pen_holder_length,
                motor_zero_pen,
            )
        };

        let gamma_one = {
            let motor_one_pen = Math::distance((self.motor_length, 0.0), destination);
            Math::acos_rule(
                self.proximal_length,
                self.distal_length + self.pen_holder_length,
                motor_one_pen,
            )
        };

        let motor_zero_common =
            Math::cos_rule(self.proximal_length, self.distal_length, gamma_zero);
        let motor_one_common = Math::cos_rule(self.proximal_length, self.distal_length, gamma_one);

        let beta_zero =
            Math::acos_rule(self.proximal_length, motor_zero_common, self.distal_length);
        let beta_one = Math::acos_rule(self.proximal_length, motor_one_common, self.distal_length);

        let alpha_zero = Math::acos_rule(self.motor_length, motor_zero_common, motor_one_common);
        let alpha_one = Math::acos_rule(self.motor_length, motor_one_common, motor_zero_common);

        (alpha_zero + beta_zero, alpha_one + beta_one)
    }
}

struct USB<'a> {
    path: &'a str,
    port: File,
}

impl<'a> USB<'a> {
    fn new(path: &'a str) -> Self {
        Self {
            path: path,
            port: OpenOptions::new()
                .read(true)
                .write(true)
                .open(path)
                .expect("macOS Spesific Error / Device Not Detected"),
        }
    }

    fn send_instruction(&mut self, double_precision_angles: (f64, f64)) {
        let single_precision_angles = (
            double_precision_angles.0 as f32,
            double_precision_angles.1 as f32,
        );

        let command: [u8; 8] = unsafe {
            transmute([
                single_precision_angles.0.to_le_bytes(),
                single_precision_angles.1.to_le_bytes(),
            ])
        };

        self.port
            .write_all(&command)
            .expect("Interface Failure in Forward Direction");
    }

    fn block_for_next(&mut self) {
        let mut responce = [0u8; 8];

        match self.port.read_exact(&mut responce) {
            Ok(()) if &responce == b"INSTRCTN" => return,
            Ok(()) => panic!("Bad Traffic on Interface"),
            Err(_) => panic!("Interface Failure in Backward Direction"),
        }
    }
}
