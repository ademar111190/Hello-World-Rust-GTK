pub enum StringId {
    HomeLoading,
    HomeTitle,
    Quit,
}

pub fn get_string(id: StringId) -> String {
    let raw = match id {
        StringId::HomeLoading => "Carregando…",
        StringId::HomeTitle => "Campeões do Campeonato Paulista de Futebol",
        StringId::Quit => "Encerrar",
    };
    return String::from(raw);
}
