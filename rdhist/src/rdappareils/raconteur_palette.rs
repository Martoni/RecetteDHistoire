use crate::rdappareils::Appareil;

const NOM_APP: &str = "raconteur_palette";

pub struct RaconteurPalette {
    pub nom: String,
}

impl RaconteurPalette {
    pub fn new() -> RaconteurPalette {
        RaconteurPalette {
            nom: NOM_APP.to_string()
        }
    }
}

impl Appareil for RaconteurPalette {
    fn nom(&self) -> &str {
        &self.nom
    }
}
