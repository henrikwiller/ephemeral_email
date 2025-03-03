use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Domain {
    Custom(String),
    TenMinMailDe,
    TenMinutenMailXyz,
    ExistiertNet,
    FliegenderFish,
    JagaEmail,
    MdzEmail,
    MuellMailCom,
    MuelleMailCom,
    MuellMonster,
    MuellIcu,
    MuellIo,
    MuellXyz,
    MagSpamNet,
    FukaruCom,
    OidaIcu,
    PapierkorbMe,
    SpamCare,
    TonneTo,
    UltraFyi,
    WegwerfEmailDe,
    DsgvoParty,
    KnickerbockerbanDe,
    LambsauceDe,
    RamenMailDe,
    Ji5De,
    Ji6De,
    Ji7De,
    SudernDe,
    HihiLol,
    KeinDate,
    HolioDay,
    CornHolioDay,
    BungHolioDay,
    StacysMom,
    EdnyNet,
    FileSavedOrg,
}

impl Domain {
    pub(crate) fn get_all_domains() -> Vec<Domain> {
        vec![
            Domain::TenMinMailDe,
            Domain::TenMinutenMailXyz,
            Domain::ExistiertNet,
            Domain::FliegenderFish,
            Domain::JagaEmail,
            Domain::MdzEmail,
            Domain::MuellMailCom,
            Domain::MuelleMailCom,
            Domain::MuellMonster,
            Domain::MuellIcu,
            Domain::MuellIo,
            Domain::MuellXyz,
            Domain::MagSpamNet,
            Domain::FukaruCom,
            Domain::OidaIcu,
            Domain::PapierkorbMe,
            Domain::SpamCare,
            Domain::TonneTo,
            Domain::UltraFyi,
            Domain::WegwerfEmailDe,
            Domain::DsgvoParty,
            Domain::KnickerbockerbanDe,
            Domain::LambsauceDe,
            Domain::RamenMailDe,
            Domain::Ji5De,
            Domain::Ji6De,
            Domain::Ji7De,
            Domain::SudernDe,
            Domain::HihiLol,
            Domain::KeinDate,
            Domain::HolioDay,
            Domain::CornHolioDay,
            Domain::BungHolioDay,
            Domain::StacysMom,
            Domain::EdnyNet,
            Domain::FileSavedOrg,
        ]
    }
}

impl Display for Domain {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Domain::Custom(custom) => write!(f, "{}", custom),
            Domain::TenMinMailDe => write!(f, "10minmail.de"),
            Domain::TenMinutenMailXyz => write!(f, "10minutenmail.xyz"),
            Domain::ExistiertNet => write!(f, "existiert.net"),
            Domain::FliegenderFish => write!(f, "fliegender.fish"),
            Domain::JagaEmail => write!(f, "jaga.email"),
            Domain::MdzEmail => write!(f, "mdz.email"),
            Domain::MuellMailCom => write!(f, "muellmail.com"),
            Domain::MuelleMailCom => write!(f, "muellemail.com"),
            Domain::MuellMonster => write!(f, "muell.monster"),
            Domain::MuellIcu => write!(f, "muell.icu"),
            Domain::MuellIo => write!(f, "muell.io"),
            Domain::MuellXyz => write!(f, "muell.xyz"),
            Domain::MagSpamNet => write!(f, "magspam.net"),
            Domain::FukaruCom => write!(f, "fukaru.com"),
            Domain::OidaIcu => write!(f, "oida.icu"),
            Domain::PapierkorbMe => write!(f, "papierkorb.me"),
            Domain::SpamCare => write!(f, "spam.care"),
            Domain::TonneTo => write!(f, "tonne.to"),
            Domain::UltraFyi => write!(f, "ultra.fyi"),
            Domain::WegwerfEmailDe => write!(f, "wegwerfemail.de"),
            Domain::DsgvoParty => write!(f, "dsgvo.party"),
            Domain::KnickerbockerbanDe => write!(f, "knickerbockerban.de"),
            Domain::LambsauceDe => write!(f, "lambsauce.de"),
            Domain::RamenMailDe => write!(f, "ramenmail.de"),
            Domain::Ji5De => write!(f, "ji5.de"),
            Domain::Ji6De => write!(f, "ji6.de"),
            Domain::Ji7De => write!(f, "ji7.de"),
            Domain::SudernDe => write!(f, "sudern.de"),
            Domain::HihiLol => write!(f, "hihi.lol"),
            Domain::KeinDate => write!(f, "kein.date"),
            Domain::HolioDay => write!(f, "holio.day"),
            Domain::CornHolioDay => write!(f, "corn.holio.day"),
            Domain::BungHolioDay => write!(f, "bung.holio.day"),
            Domain::StacysMom => write!(f, "stacys.mom"),
            Domain::EdnyNet => write!(f, "edny.net"),
            Domain::FileSavedOrg => write!(f, "filesaved.org"),
        }
    }
}

impl From<&str> for Domain {
    fn from(value: &str) -> Self {
        if let Some(domain) = Domain::get_all_domains()
            .into_iter()
            .find(|domain| domain.to_string() == value)
        {
            return domain;
        }

        Domain::Custom(value.to_string())
    }
}
