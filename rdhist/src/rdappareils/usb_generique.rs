use crate::rdappareils::Appareil;

const NOM_APP: &str = "usb_generique"; 


pub struct UsbGenerique {
     pub nom: String,
}

impl UsbGenerique {
    pub fn new() -> UsbGenerique {
        UsbGenerique {
            nom: NOM_APP.to_string()
        }
    }
}

impl Appareil for UsbGenerique {
    fn nom(&self) -> &str {
        &self.nom
    }
}
