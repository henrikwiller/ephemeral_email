use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Domain {
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
}

impl Display for Domain {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
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
        }
    }
}
