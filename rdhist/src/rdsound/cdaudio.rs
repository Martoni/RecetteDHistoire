/// Gestion du lecteur de cdrom
/// Les programmes cdparanoia et cd-discid sont requis.
use std::error::Error;
use std::process::Command;
use std::time::Duration;
use std::str;


const CMD_CDDISCID: &str = "cd-discid";
const CMD_CDPARANOIA: &str = "cdparanoia";

/// Lit le numéro unique du CD se trouvant dans le lecteur
pub fn get_discid() -> Result<String, Box<dyn Error>> {
    let _cmdret = Command::new(CMD_CDDISCID)
                        .output()
                        .expect(&format!("La commande {CMD_CDDISCID} a échoué"));

    Ok(str::replace(str::from_utf8(&_cmdret.stdout)?,"\n",""))
}

pub fn get_toc() -> Result<String, Box<dyn Error>> {
    let _cmdret = Command::new(CMD_CDPARANOIA)
                        .args(["-Q"])
                        .output()
                        .expect(&format!("La commande {CMD_CDPARANOIA} a échoué"));

    // La table des matières est affichée sur le canal d'erreur
    // dans cdparanoia !
    let sret = _cmdret.stderr;

    Ok(str::from_utf8(&sret)?.to_string())
}

pub fn parse_duration(raw_duration: &str) -> Result<Duration, Box<dyn Error>> {
    let min  = raw_duration[1..3].parse::<u64>()?;
    let sec  = raw_duration[4..6].parse::<u64>()?;
    let dsec = raw_duration[7..9].parse::<u64>()?;
    Ok(Duration::from_secs(min*60)
        + Duration::from_secs(sec)
        + Duration::from_millis(dsec*10))
}

pub fn parse_toc(ntrack: u8, raw_toc: &str) -> Result<Vec<Duration>, Box<dyn Error>> {
    let mut iter_toc = raw_toc.lines();
    let mut line = " ";
    while line.chars().nth(1) != Some('=') {
        line = iter_toc.next().unwrap();
    }
    let mut tracks = Vec::<Duration>::new();
    for _track in 1..=ntrack {
        line = iter_toc.next().unwrap();
        let vec_track: Vec<&str> = line.split_whitespace().collect();
        tracks.push(parse_duration(vec_track[2])?);
    }
    Ok(tracks)
}

///---------------------
/// Table Of Content
///---------------------

pub struct CdToc {
    pub ntracks: u8,
    pub tracks: Vec<Duration>,
}

impl CdToc {
    pub fn in_drive() -> Result<CdToc, Box<dyn Error>> {
        // Récupération du discid pour avoir le nombre de pistes
        let discid = DiscId::in_drive()?;
        // lecture de la table des matières «brute»
        let raw_toc = get_toc()?;
        let ntracks = discid.tracks_number()?;
        Ok(CdToc {
            ntracks: ntracks,
            tracks: parse_toc(ntracks, &raw_toc)?,
        })
    }
}

///----------------------
/// DiscID

/// Structure de «stockage» du discid
pub struct DiscId {
    _cddiscid: String,
}

impl PartialEq for DiscId {
    fn eq(&self, other: &Self) -> bool {
        self._cddiscid == other._cddiscid
    }
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
