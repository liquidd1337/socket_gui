use rand::Rng;
use std::fmt;

#[derive(Default)]
pub struct SmartSocket {
    pub name: String,
    status: bool,
    voltage: f32,
}

impl SmartSocket {
    pub fn new(name: String) -> SmartSocket {
        Self {
            name,
            status: false,
            voltage: 0.0,
        }
    }

    pub fn socket_on(&mut self) {
        self.status = true;
        self.voltage = 220.0 + rand::thread_rng().gen_range(-10.0..=-10.0);
    }

    pub fn socket_off(&mut self) {
        self.status = false;
        self.voltage = 0.0;
    }

    pub fn get_display_string(&self) -> String {
        format!(
            "\n
            Socket name: {} 
            Status: {}, 
            Current voltage: {} V \n",
            self.name,
            if self.status { "On" } else { "Off" },
            self.voltage,
        )
    }
}

impl fmt::Display for SmartSocket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_display_string())
    }
}
