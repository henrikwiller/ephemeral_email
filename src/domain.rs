use std::fmt::{self, Display, Formatter};

/// The email domain to use for the temporary email address.
///
/// This is an enum with all possible domains. To use a custom domain, use the `Custom` variant.
///
/// # Example
/// ```
/// use ephemeral_email::Domain;
///
/// let domain = Domain::FileSavedOrg;
/// assert_eq!("filesaved.org", domain.to_string());
///
/// let domain: Domain = "filesaved.org".into();
/// assert_eq!(Domain::FileSavedOrg, domain);
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

    // Domains from MuellMail
    /// The 10minmail.de domain.
    #[cfg(feature = "use-rquest")]
    TenMinMailDe,
    /// The 10minutenmail.xyz domain.
    #[cfg(feature = "use-rquest")]
    TenMinutenMailXyz,
    /// The existiert.net domain.
    #[cfg(feature = "use-rquest")]
    ExistiertNet,
    /// The fliegender.fish domain.
    #[cfg(feature = "use-rquest")]
    FliegenderFish,
    /// The jaga.email domain.
    #[cfg(feature = "use-rquest")]
    JagaEmail,
    /// The mdz.email domain.
    #[cfg(feature = "use-rquest")]
    MdzEmail,
    /// The muellmail.com domain.
    #[cfg(feature = "use-rquest")]
    MuellMailCom,
    /// The muellemail.com domain.
    #[cfg(feature = "use-rquest")]
    MuelleMailCom,
    /// The muell.monster domain.
    #[cfg(feature = "use-rquest")]
    MuellMonster,
    /// The muell.icu domain.
    #[cfg(feature = "use-rquest")]
    MuellIcu,
    /// The muell.io domain.
    #[cfg(feature = "use-rquest")]
    MuellIo,
    /// The muell.xyz domain.
    #[cfg(feature = "use-rquest")]
    MuellXyz,
    /// The magspam.net domain.
    #[cfg(feature = "use-rquest")]
    MagSpamNet,
    /// The fukaru.com domain.
    #[cfg(feature = "use-rquest")]
    FukaruCom,
    /// The oida.icu domain.
    #[cfg(feature = "use-rquest")]
    OidaIcu,
    /// The papierkorb.me domain.
    #[cfg(feature = "use-rquest")]
    PapierkorbMe,
    /// The spam.care domain.
    #[cfg(feature = "use-rquest")]
    SpamCare,
    /// The tonne.to domain.
    #[cfg(feature = "use-rquest")]
    TonneTo,
    /// The ultra.fyi domain.
    #[cfg(feature = "use-rquest")]
    UltraFyi,
    /// The wegwerfemail.de domain.
    #[cfg(feature = "use-rquest")]
    WegwerfEmailDe,
    /// The dsgvo.party domain.
    #[cfg(feature = "use-rquest")]
    DsgvoParty,
    /// The knickerbockerban.de domain.
    #[cfg(feature = "use-rquest")]
    KnickerbockerbanDe,
    /// The lambsauce.de domain.
    #[cfg(feature = "use-rquest")]
    LambsauceDe,
    /// The ramenmail.de domain.
    #[cfg(feature = "use-rquest")]
    RamenMailDe,
    /// The ji5.de domain.
    #[cfg(feature = "use-rquest")]
    Ji5De,
    /// The ji6.de domain.
    #[cfg(feature = "use-rquest")]
    Ji6De,
    /// The ji7.de domain.
    #[cfg(feature = "use-rquest")]
    Ji7De,
    /// The sudern.de domain.
    #[cfg(feature = "use-rquest")]
    SudernDe,
    /// The hihi.lol domain.
    #[cfg(feature = "use-rquest")]
    HihiLol,
    /// The kein.date domain.
    #[cfg(feature = "use-rquest")]
    KeinDate,
    /// The holio.day domain.
    #[cfg(feature = "use-rquest")]
    HolioDay,
    /// The corn.holio.day domain.
    #[cfg(feature = "use-rquest")]
    CornHolioDay,
    /// The bung.holio.day domain.
    #[cfg(feature = "use-rquest")]
    BungHolioDay,
    /// The stacys.mom domain.
    #[cfg(feature = "use-rquest")]
    StacysMom,

    // Domains from FakeMailNet
    /// The filesaved.org domain.
    FileSavedOrg,

    // Domains from TempMailLol
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
            #[cfg(feature = "use-rquest")]
            Domain::TenMinMailDe,
            #[cfg(feature = "use-rquest")]
            Domain::TenMinutenMailXyz,
            #[cfg(feature = "use-rquest")]
            Domain::ExistiertNet,
            #[cfg(feature = "use-rquest")]
            Domain::FliegenderFish,
            #[cfg(feature = "use-rquest")]
            Domain::JagaEmail,
            #[cfg(feature = "use-rquest")]
            Domain::MdzEmail,
            #[cfg(feature = "use-rquest")]
            Domain::MuellMailCom,
            #[cfg(feature = "use-rquest")]
            Domain::MuelleMailCom,
            #[cfg(feature = "use-rquest")]
            Domain::MuellMonster,
            #[cfg(feature = "use-rquest")]
            Domain::MuellIcu,
            #[cfg(feature = "use-rquest")]
            Domain::MuellIo,
            #[cfg(feature = "use-rquest")]
            Domain::MuellXyz,
            #[cfg(feature = "use-rquest")]
            Domain::MagSpamNet,
            #[cfg(feature = "use-rquest")]
            Domain::FukaruCom,
            #[cfg(feature = "use-rquest")]
            Domain::OidaIcu,
            #[cfg(feature = "use-rquest")]
            Domain::PapierkorbMe,
            #[cfg(feature = "use-rquest")]
            Domain::SpamCare,
            #[cfg(feature = "use-rquest")]
            Domain::TonneTo,
            #[cfg(feature = "use-rquest")]
            Domain::UltraFyi,
            #[cfg(feature = "use-rquest")]
            Domain::WegwerfEmailDe,
            #[cfg(feature = "use-rquest")]
            Domain::DsgvoParty,
            #[cfg(feature = "use-rquest")]
            Domain::KnickerbockerbanDe,
            #[cfg(feature = "use-rquest")]
            Domain::LambsauceDe,
            #[cfg(feature = "use-rquest")]
            Domain::RamenMailDe,
            #[cfg(feature = "use-rquest")]
            Domain::Ji5De,
            #[cfg(feature = "use-rquest")]
            Domain::Ji6De,
            #[cfg(feature = "use-rquest")]
            Domain::Ji7De,
            #[cfg(feature = "use-rquest")]
            Domain::SudernDe,
            #[cfg(feature = "use-rquest")]
            Domain::HihiLol,
            #[cfg(feature = "use-rquest")]
            Domain::KeinDate,
            #[cfg(feature = "use-rquest")]
            Domain::HolioDay,
            #[cfg(feature = "use-rquest")]
            Domain::CornHolioDay,
            #[cfg(feature = "use-rquest")]
            Domain::BungHolioDay,
            #[cfg(feature = "use-rquest")]
            Domain::StacysMom,
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
            #[cfg(feature = "use-rquest")]
            Domain::TenMinMailDe => write!(f, "10minmail.de"),
            #[cfg(feature = "use-rquest")]
            Domain::TenMinutenMailXyz => write!(f, "10minutenmail.xyz"),
            #[cfg(feature = "use-rquest")]
            Domain::ExistiertNet => write!(f, "existiert.net"),
            #[cfg(feature = "use-rquest")]
            Domain::FliegenderFish => write!(f, "fliegender.fish"),
            #[cfg(feature = "use-rquest")]
            Domain::JagaEmail => write!(f, "jaga.email"),
            #[cfg(feature = "use-rquest")]
            Domain::MdzEmail => write!(f, "mdz.email"),
            #[cfg(feature = "use-rquest")]
            Domain::MuellMailCom => write!(f, "muellmail.com"),
            #[cfg(feature = "use-rquest")]
            Domain::MuelleMailCom => write!(f, "muellemail.com"),
            #[cfg(feature = "use-rquest")]
            Domain::MuellMonster => write!(f, "muell.monster"),
            #[cfg(feature = "use-rquest")]
            Domain::MuellIcu => write!(f, "muell.icu"),
            #[cfg(feature = "use-rquest")]
            Domain::MuellIo => write!(f, "muell.io"),
            #[cfg(feature = "use-rquest")]
            Domain::MuellXyz => write!(f, "muell.xyz"),
            #[cfg(feature = "use-rquest")]
            Domain::MagSpamNet => write!(f, "magspam.net"),
            #[cfg(feature = "use-rquest")]
            Domain::FukaruCom => write!(f, "fukaru.com"),
            #[cfg(feature = "use-rquest")]
            Domain::OidaIcu => write!(f, "oida.icu"),
            #[cfg(feature = "use-rquest")]
            Domain::PapierkorbMe => write!(f, "papierkorb.me"),
            #[cfg(feature = "use-rquest")]
            Domain::SpamCare => write!(f, "spam.care"),
            #[cfg(feature = "use-rquest")]
            Domain::TonneTo => write!(f, "tonne.to"),
            #[cfg(feature = "use-rquest")]
            Domain::UltraFyi => write!(f, "ultra.fyi"),
            #[cfg(feature = "use-rquest")]
            Domain::WegwerfEmailDe => write!(f, "wegwerfemail.de"),
            #[cfg(feature = "use-rquest")]
            Domain::DsgvoParty => write!(f, "dsgvo.party"),
            #[cfg(feature = "use-rquest")]
            Domain::KnickerbockerbanDe => write!(f, "knickerbockerban.de"),
            #[cfg(feature = "use-rquest")]
            Domain::LambsauceDe => write!(f, "lambsauce.de"),
            #[cfg(feature = "use-rquest")]
            Domain::RamenMailDe => write!(f, "ramenmail.de"),
            #[cfg(feature = "use-rquest")]
            Domain::Ji5De => write!(f, "ji5.de"),
            #[cfg(feature = "use-rquest")]
            Domain::Ji6De => write!(f, "ji6.de"),
            #[cfg(feature = "use-rquest")]
            Domain::Ji7De => write!(f, "ji7.de"),
            #[cfg(feature = "use-rquest")]
            Domain::SudernDe => write!(f, "sudern.de"),
            #[cfg(feature = "use-rquest")]
            Domain::HihiLol => write!(f, "hihi.lol"),
            #[cfg(feature = "use-rquest")]
            Domain::KeinDate => write!(f, "kein.date"),
            #[cfg(feature = "use-rquest")]
            Domain::HolioDay => write!(f, "holio.day"),
            #[cfg(feature = "use-rquest")]
            Domain::CornHolioDay => write!(f, "corn.holio.day"),
            #[cfg(feature = "use-rquest")]
            Domain::BungHolioDay => write!(f, "bung.holio.day"),
            #[cfg(feature = "use-rquest")]
            Domain::StacysMom => write!(f, "stacys.mom"),
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
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_domain_from_str() {
        assert_eq!(Domain::from("filesaved.org"), Domain::FileSavedOrg);
        assert_eq!(
            Domain::from("terriblecoffee.org"),
            Domain::TerribleCoffeeOrg
        );
        assert_eq!(
            Domain::from("custom.com"),
            Domain::Custom("custom.com".to_string())
        );
    }

    #[test]
    fn test_domain_to_string() {
        assert_eq!(Domain::FileSavedOrg.to_string(), "filesaved.org");
        assert_eq!(Domain::TerribleCoffeeOrg.to_string(), "terriblecoffee.org");
        assert_eq!(
            Domain::Custom("custom.com".to_string()).to_string(),
            "custom.com"
        );
    }
}
