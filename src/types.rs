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

#[derive(Debug, PartialEq, Clone)]
pub struct ClassDetails {
    pub thresholds: Vec<usize>,
}

pub fn animasquad_details() -> ClassDetails {
    ClassDetails {
        thresholds: vec![3, 5, 7, 10],
    }
}

pub fn boombots_details() -> ClassDetails {
    ClassDetails {
        thresholds: vec![2, 4, 7],
    }
}

pub fn cyberboss_details() -> ClassDetails {
    ClassDetails {
        thresholds: vec![2, 3, 4],
    }
}

pub fn cypher_details() -> ClassDetails {
    ClassDetails {
        thresholds: vec![3, 4, 5],
    }
}

pub fn divinicorp_details() -> ClassDetails {
    ClassDetails {
        thresholds: vec![1, 2, 3, 4, 5, 6, 7],
    }
}

pub fn exotech_details() -> ClassDetails {
    ClassDetails {
        thresholds: vec![3, 5, 7, 10],
    }
}

pub fn godofthenet_details() -> ClassDetails {
    ClassDetails {
        thresholds: vec![1],
    }
}

pub fn goldenox_details() -> ClassDetails {
    ClassDetails {
        thresholds: vec![2, 4, 6],
    }
}

pub fn nitro_details() -> ClassDetails {
    ClassDetails {
        thresholds: vec![3, 4],
    }
}

pub fn overlord_details() -> ClassDetails {
    ClassDetails {
        thresholds: vec![1],
    }
}

pub fn soulkiller_details() -> ClassDetails {
    ClassDetails {
        thresholds: vec![1],
    }
}

pub fn streetdemon_details() -> ClassDetails {
    ClassDetails {
        thresholds: vec![3, 5, 7, 10],
    }
}

pub fn syndicate_details() -> ClassDetails {
    ClassDetails {
        thresholds: vec![3, 5, 7],
    }
}

pub fn virus_details() -> ClassDetails {
    ClassDetails {
        thresholds: vec![1],
    }
}

// Class detail functions
pub fn amp_details() -> ClassDetails {
    ClassDetails {
        thresholds: vec![2, 3, 4, 5],
    }
}

pub fn bastion_details() -> ClassDetails {
    ClassDetails {
        thresholds: vec![2, 4, 6],
    }
}

pub fn bruiser_details() -> ClassDetails {
    ClassDetails {
        thresholds: vec![2, 4, 6],
    }
}

pub fn dynamo_details() -> ClassDetails {
    ClassDetails {
        thresholds: vec![2, 3, 4],
    }
}

pub fn executioner_details() -> ClassDetails {
    ClassDetails {
        thresholds: vec![2, 3, 4, 5],
    }
}

pub fn marksman_details() -> ClassDetails {
    ClassDetails {
        thresholds: vec![2, 4],
    }
}

pub fn rapidfire_details() -> ClassDetails {
    ClassDetails {
        thresholds: vec![2, 4, 6],
    }
}

pub fn slayer_details() -> ClassDetails {
    ClassDetails {
        thresholds: vec![2, 4, 6],
    }
}

pub fn strategist_details() -> ClassDetails {
    ClassDetails {
        thresholds: vec![2, 3, 4, 5],
    }
}

pub fn techie_details() -> ClassDetails {
    ClassDetails {
        thresholds: vec![2, 4, 6, 8],
    }
}

pub fn vanguard_details() -> ClassDetails {
    ClassDetails {
        thresholds: vec![2, 4, 6],
    }
}
