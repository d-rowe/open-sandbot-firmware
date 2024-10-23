use embassy_rp::{gpio, PeripheralRef};
use gpio::{AnyPin, Level, Output};

pub struct Stepper<'a> {
    step_pin: Output<'a, AnyPin>,
    dir_pin: Output<'a, AnyPin>,
    pub current_step: i64,
}

impl Stepper<'_> {
    pub fn new<'a>(
        step_pin: PeripheralRef<'static, AnyPin>,
        dir_pin: PeripheralRef<'static, AnyPin>,
    ) -> Stepper<'a> {
        Stepper {
            step_pin: create_output(step_pin),
            dir_pin: create_output(dir_pin),
            current_step: 0,
        }
    }

    pub fn step(&mut self, is_forward: bool) {
        if is_forward {
            self.dir_pin.set_high();
            self.current_step += 1;
        } else {
            self.dir_pin.set_low();
            self.current_step -= 1;
        }

        self.step_pin.toggle();
    }
}

fn create_output(pin: PeripheralRef<'static, AnyPin>) -> Output<'static, AnyPin> {
    Output::new(pin, Level::Low)
}
