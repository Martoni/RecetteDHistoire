/// Gestion du lecteur de cdrom
/// Les programmes cdparanoia et cd-discid sont requis.
use std::error::Error;
use std::process::Command;
use std::str;

/// Lit le numéro unique du CD se trouvant dans le lecteur
pub fn get_discid() -> Result<String, Box<dyn Error>> {
    let _cmdret = Command::new("cd-discid")
                            .output()
                            .expect("La commande cd-discid a échouée");

    Ok(str::replace(str::from_utf8(&_cmdret.stdout)?,"\n",""))
}

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
