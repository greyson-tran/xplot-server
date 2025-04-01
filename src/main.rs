use std::f64::consts::*;

fn main() {
    println!("Hello, World!");
}
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

struct G1 {
    destination: (f64, f64),
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
