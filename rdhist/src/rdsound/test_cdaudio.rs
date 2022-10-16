/// Test à lancer avec :
/// $ RUST_BACKTRACE=1 cargo test -- --nocapture
///     ^                               ^
///     |     pour avoir les println! --|
///     |- pour avoir la trace en cas d'erreur

#[allow(dead_code)]
const DISCID_SAMPLE: &str = "30027f05 5 150 2422 15072 29219 44022 641";
#[allow(dead_code)]
const TOC_PANTHÈRE: &str = "cdparanoia III release 10.2 (September 11, 2008)

 

Table of contents (audio tracks only):
track        length               begin        copy pre ch
===========================================================
  1.     2272 [00:30.22]        0 [00:00.00]    no   no  2
  2.    12650 [02:48.50]     2272 [00:30.22]    no   no  2
  3.    14147 [03:08.47]    14922 [03:18.72]    no   no  2
  4.    14803 [03:17.28]    29069 [06:27.44]    no   no  2
  5.     4069 [00:54.19]    43872 [09:44.72]    no   no  2
TOTAL   47941 [10:39.16]    (audio only)
 
";

#[cfg(test)]
mod test_disc_id {
    use crate::rdsound::cdaudio::*;
    use super::{DISCID_SAMPLE};

    /// les tests suivant fonctionne avec le cd
    /// de mes premiers j'aime lire titré
    /// Tiens bon, petite panthère !
    /// numéro 234 de Février 2022

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

    #[test]
    fn test_equality(){
        let discid1 = DiscId::from_string(String::from(DISCID_SAMPLE))
                        .expect("Impossible de récupérer le discid du cd");
        let discid2 = DiscId::from_string(String::from(DISCID_SAMPLE))
                        .expect("Impossible de récupérer le discid du cd");
        assert!(discid1 == discid2);
    }
}

#[cfg(test)]
mod test_table_of_content {
    use crate::rdsound::cdaudio::*;
    use super::{TOC_PANTHÈRE};

    #[test]
    fn test_get_toc() {
       let stoc = get_toc().expect("get_toc a échoué");
       assert!(stoc == TOC_PANTHÈRE);
    }

    #[test]
    fn test_parse_toc() {
        let ptoc = parse_toc(5, TOC_PANTHÈRE).expect("Problème de parse");
        println!("{:?}", ptoc);
        assert!(true);
    }

    #[test]
    fn test_in_drive() {
        let _toc = CdToc::in_drive();
        assert!(true);
    }
}

#[cfg(test)]
mod test_wav_conversion {
    use crate::rdsound::cdaudio::*;

    #[test]
    fn test_wave2mp3(){
        let _ret = wav2mp3("/tmp/track01.cdda.wav".to_string(),
                          "/tmp".to_string(),
                          "track01.mp3".to_string())
                          .expect("wave2mp3 error");
        assert!(true);
    }
}
