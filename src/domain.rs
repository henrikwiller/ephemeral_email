use std::fmt::{self, Display, Formatter};

/// The email domain to use for the temporary email address.
///
/// This is an enum with all possible domains. To use a custom domain, use the `Custom` variant.
///
/// # Example
/// ```
/// use ephemeral_email::Domain;
///
/// let domain = Domain::TenMinMailDe;
/// assert_eq!("10minmail.de", domain.to_string());
///
/// let domain: Domain = "10minmail.de".into();
/// assert_eq!(Domain::TenMinMailDe, domain);
///
/// let domain = Domain::Custom("example.com".to_string());
/// assert_eq!("example.com", domain.to_string());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Domain {
    /// A custom domain, represented by any valid domain string.
    /// This variant is used when the domain is not one of the predefined options.
    /// Some providers support custom domains or subdomains.
    Custom(String),
    /// The 10minmail.de domain.
    TenMinMailDe,
    /// The 10minutenmail.xyz domain.
    TenMinutenMailXyz,
    /// The existiert.net domain.
    ExistiertNet,
    /// The fliegender.fish domain.
    FliegenderFish,
    /// The jaga.email domain.
    JagaEmail,
    /// The mdz.email domain.
    MdzEmail,
    /// The muellmail.com domain.
    MuellMailCom,
    /// The muellemail.com domain.
    MuelleMailCom,
    /// The muell.monster domain.
    MuellMonster,
    /// The muell.icu domain.
    MuellIcu,
    /// The muell.io domain.
    MuellIo,
    /// The muell.xyz domain.
    MuellXyz,
    /// The magspam.net domain.
    MagSpamNet,
    /// The fukaru.com domain.
    FukaruCom,
    /// The oida.icu domain.
    OidaIcu,
    /// The papierkorb.me domain.
    PapierkorbMe,
    /// The spam.care domain.
    SpamCare,
    /// The tonne.to domain.
    TonneTo,
    /// The ultra.fyi domain.
    UltraFyi,
    /// The wegwerfemail.de domain.
    WegwerfEmailDe,
    /// The dsgvo.party domain.
    DsgvoParty,
    /// The knickerbockerban.de domain.
    KnickerbockerbanDe,
    /// The lambsauce.de domain.
    LambsauceDe,
    /// The ramenmail.de domain.
    RamenMailDe,
    /// The ji5.de domain.
    Ji5De,
    /// The ji6.de domain.
    Ji6De,
    /// The ji7.de domain.
    Ji7De,
    /// The sudern.de domain.
    SudernDe,
    /// The hihi.lol domain.
    HihiLol,
    /// The kein.date domain.
    KeinDate,
    /// The holio.day domain.
    HolioDay,
    /// The corn.holio.day domain.
    CornHolioDay,
    /// The bung.holio.day domain.
    BungHolioDay,
    /// The stacys.mom domain.
    StacysMom,
    /// The edny.net domain.
    EdnyNet,
    /// The filesaved.org domain.
    FileSavedOrg,
    /// The terriblecoffee.org domain.
    TerribleCoffeeOrg,
    /// The underseagolf.com domain.
    UnderseaGolfCom,
    /// The jailbreakeverything.com domain.
    JailBreakEverythingCom,
    /// The awesome47.com domain.
    Awesome47,
    /// The expiredtoaster.org domain.
    ExpiredToasterOrg,
    /// The undeadbank.com domain.
    UndeadBankCom,
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
            Domain::TerribleCoffeeOrg,
            Domain::UnderseaGolfCom,
            Domain::JailBreakEverythingCom,
            Domain::Awesome47,
            Domain::ExpiredToasterOrg,
            Domain::UndeadBankCom,
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
            Domain::TerribleCoffeeOrg => write!(f, "terriblecoffee.org"),
            Domain::UnderseaGolfCom => write!(f, "underseagolf.com"),
            Domain::JailBreakEverythingCom => write!(f, "jailbreakeverything.com"),
            Domain::Awesome47 => write!(f, "awesome47.com"),
            Domain::ExpiredToasterOrg => write!(f, "expiredtoaster.org"),
            Domain::UndeadBankCom => write!(f, "undeadbank.com"),
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
