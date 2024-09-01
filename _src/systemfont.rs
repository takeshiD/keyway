use std::collections::HashSet;
pub fn get_fontfamily_list() -> Vec<String> {
    let mut db = fontdb::Database::new();
    db.load_system_fonts();
    let mut families = HashSet::new();
    for info in db.faces() {
        for (family, lang) in info.families.iter() {
            if *lang == fontdb::Language::English_UnitedStates {
                families.insert(family.clone());
            }
        }
    }
    let mut families = families.into_iter().collect::<Vec<_>>();
    families.sort();
    families
}
