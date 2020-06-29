pub enum StringId {
    HomeTitle
}

pub fn get_string(id: StringId) -> String {
    let raw = match id {
        StringId::HomeTitle => "Campeões do Campeonato Paulista de Futebol"
    };
    return String::from(raw);
}
