/// Gestion du lecteur de cdrom
/// Les programmes cdparanoia et cd-discid sont requis.
use std::error::Error;
use std::process::Command;

/// Lit le numéro unique du CD se trouvant dans le lecteur
pub fn get_discid() -> Result<String, Box<dyn Error>> {
    let _cmdret = Command::new("cd-discid")
                            .output()
                            .expect("La commande cd-discid a échouée");
    let ret = _cmdret.stdout;
    println!("{:?}", ret);
    Ok("C'est bon !".to_string())
}
