/// Test à lancer avec :
/// $ RUST_BACKTRACE=1 cargo test -- --nocapture
///     ^                               ^
///     |  pour avoir les println! -----|
///     |- pour avoir la trace en cas d'erreur
#[cfg(test)]
mod tests {
    use crate::rdsound::cdaudio::*;

    /// les tests suivant fonctionne avec le cd
    /// de mes premiers j'aime lire titré
    /// Tiens bon, petite panthère ! 
    /// numéro 234 de Février 2022
    const DISCID_SAMPLE: &str = "30027f05 5 150 2422 15072 29219 44022 641";

    #[test]
    fn test_get_discid() {
        let ret = get_discid().expect("Erreur de lecture du discid");
        assert_eq!(ret, DISCID_SAMPLE);
    }

    #[test]
    fn test_in_drive() {
        let discid = DiscId::in_drive()
                        .expect("Impossible de récupérer le discid du cd");
        assert_eq!(discid.raw(), DISCID_SAMPLE);
    }

    #[test]
    fn test_hashid() {
        let discid = DiscId::from_string(String::from(DISCID_SAMPLE))
                        .expect("Impossible de récupérer le discid du cd");
        let hashid = discid.hashid().expect("Impossible d'extraire le hashid");
        assert_eq!(hashid, "30027f05"); 
    }

    #[test]
    fn test_tracks_number() {
        let discid = DiscId::from_string(String::from(DISCID_SAMPLE))
                        .expect("Impossible de récupérer le discid du cd");
        let tracksnum = discid.tracks_number()
                        .expect("Impossible de décoder le nombre de pistes");
        assert_eq!(tracksnum, 5);
    }
}
