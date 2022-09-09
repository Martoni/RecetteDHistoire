/// Gestion du lecteur de cdrom
/// Les programmes cdparanoia et cd-discid sont requis.
use std::error::Error;
use std::process::Command;
use std::str;

/// Structure de «stockage» du discid
pub struct DiscId {
    pub _cddiscid: String,
}

impl DiscId {
    pub fn in_drive() -> Result<DiscId, Box<dyn Error>> {
        let discid = DiscId {
            _cddiscid: get_discid()?
        };
        Ok(discid)
    }

    pub fn from_string(rawdiscid: String) -> Result<DiscId, Box<dyn Error>> {
        let discid = DiscId {
            _cddiscid: rawdiscid
        };
        Ok(discid)
    }

    pub fn raw(&self) -> &str {
        &self._cddiscid
    }

    pub fn vectorized(&self) -> Vec<&str> {
        let ret: Vec<&str> = self._cddiscid.split(' ').collect();
        ret 
    }

    pub fn hashid(&self) -> Result<&str, Box<dyn Error>> {
        let ret = self.vectorized();
        Ok(ret[0])
    }

    pub fn tracks_number(&self) -> Result<u8, Box<dyn Error>> {
        Ok(self.vectorized()[1].parse::<u8>()?)
    }
}

/// Lit le numéro unique du CD se trouvant dans le lecteur
pub fn get_discid() -> Result<String, Box<dyn Error>> {
    let _cmdret = Command::new("cd-discid")
                            .output()
                            .expect("La commande cd-discid a échouée");

    Ok(str::replace(str::from_utf8(&_cmdret.stdout)?,"\n",""))
}

/// Test à lancer avec :
/// $ RUST_BACKTRACE=1 cargo test -- --nocapture
///     ^                               ^
///     |  pour avoir les println! -----|
///     |- pour avoir la trace en cas d'erreur
#[cfg(test)]
mod tests {
    /// les tests suivant fonctionne avec le cd
    /// de mes premiers j'aime lire titré
    /// Tiens bon, petite panthère ! 
    /// numéro 234 de Février 2022
    const DISCID_SAMPLE: &str = "30027f05 5 150 2422 15072 29219 44022 641";

    #[test]
    fn test_get_discid() {
        use super::*;
        let ret = get_discid().expect("Erreur de lecture du discid");
        assert_eq!(ret, DISCID_SAMPLE);
    }

    #[test]
    fn test_in_drive() {
        use super::*;
        let discid = DiscId::in_drive()
                        .expect("Impossible de récupérer le discid du cd");
        assert_eq!(discid.raw(), DISCID_SAMPLE);
    }

    #[test]
    fn test_hashid() {
        use super::*;
        let discid = DiscId::from_string(String::from(DISCID_SAMPLE))
                        .expect("Impossible de récupérer le discid du cd");
        let hashid = discid.hashid().expect("Impossible d'extraire le hashid");
        assert_eq!(hashid, "30027f05"); 
    }

    #[test]
    fn test_tracks_number() {
        use super::*;
        let discid = DiscId::from_string(String::from(DISCID_SAMPLE))
                        .expect("Impossible de récupérer le discid du cd");
        let tracksnum = discid.tracks_number()
                        .expect("Impossible de décoder le nombre de pistes");
        assert_eq!(tracksnum, 5);
    }
}
