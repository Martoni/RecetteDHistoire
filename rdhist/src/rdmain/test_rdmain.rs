// test du module rdmain.sr

#[cfg(test)]
mod test_rdmain {
    use crate::rdmain::*;
 
    #[test]
    fn test_get_list_files_recettes() {
        let cfg = RdMainConfig::new()
                    .expect("Impossible de créer une configuration RdMainConfig");
        let recettelist = cfg.get_list_files_recettes()
                              .expect("Impossible de lire la liste des recettes");
        println!("{:?}", recettelist);
        assert!(true);
    }

    #[test]
    fn test_list_noeuds() {
        let cfg = RdMainConfig::new()
                    .expect("Impossible de créer une configuration RdMainConfig");
        let liste_noeuds = cfg.list_noeuds()
                              .expect("Impossible de lire la liste des nœuds");
        println!("Liste de nœuds trouvée: {:?}", liste_noeuds);
        assert!(true);
    } 
}
