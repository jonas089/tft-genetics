#[derive(Debug, PartialEq, Clone)]
pub struct Champion {
    pub name: String,
    pub classes: Vec<Class>,
    pub origion: Origin,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Class {
    AMP,
    BASTION,
    BRUISER,
    DYNAMO,
    EXECUTIONER,
    MARKSMAN,
    RAPIDFIRE,
    SLAYER,
    STRATEGIST,
    TECHIE,
    VANGUARD,
    OVERLORD,
    SOULKILLER,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Origin {
    ANIMASQUAD,
    BOOMBOTS,
    CYBERBOSS,
    CYPHER,
    DIVINICORP,
    EXOTECH,
    GODOFTHENET,
    GOLDENOX,
    NITRO,
    OVERLORD,
    SOULKILLER,
    STREETDEMON,
    SYNCIDATE,
    VIRUS,
}
