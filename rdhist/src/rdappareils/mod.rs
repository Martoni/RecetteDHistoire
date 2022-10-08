/* Gestion des appareils compatibles */

pub mod raconteur_palette;
pub mod usb_generique;

use std::error::Error;
use raconteur_palette::RaconteurPalette;
use usb_generique::UsbGenerique;

pub trait Appareil {
    fn nom(&self) -> &str;
}

pub struct RdAppareils {
    pub listapp: Vec<Box<dyn Appareil>>,
}

impl RdAppareils {
    pub fn new() -> Result<RdAppareils, Box<dyn Error>> {
        Ok(RdAppareils {
            listapp: vec![
                Box::new(RaconteurPalette::new()),
                Box::new(UsbGenerique::new()),
            ]
        })
    }
}
