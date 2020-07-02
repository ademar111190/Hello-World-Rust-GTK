pub enum StringId {
    HomeLoading,
    HomeTitle,
    Quit,
    StackClub,
    StackYear,
}

pub fn get_string(id: StringId) -> String {
    let raw = match id {
        StringId::HomeLoading => "Carregando…",
        StringId::HomeTitle => "Campeões do Campeonato Paulista de Futebol",
        StringId::Quit => "Encerrar",
        StringId::StackClub => "Por Clube",
        StringId::StackYear => "Por Ano",
    };
    return String::from(raw);
}
