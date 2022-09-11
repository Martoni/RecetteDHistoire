/// Test du module recette

#[allow(dead_code)]
const RECETTE_FILENAME: &str = "recettes/mes_premiers_j_aime_lire/2022_02_Tiens_bon_petite_panthere.rdhist";

#[cfg(test)]
mod test_recette {
    use crate::rdmain::*;
    use super::*;

    #[test]
    fn test_parse_recette() {
        let rdmain = RdMainConfig::new()
            .expect("Impossible de récupérer la configuration");
        let recette = Recette::new(format!("{}/{}", &rdmain.path, RECETTE_FILENAME))
            .expect("Impossible de créer la recette");
        assert!(true);
    }
}
