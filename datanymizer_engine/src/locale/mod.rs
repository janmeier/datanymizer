use serde::{Deserialize, Serialize};

mod ext_data;
pub use ext_data::ExtData;

mod ru;
pub use ru::RU;

#[allow(clippy::upper_case_acronyms)]
pub type EN = fake::locales::EN;
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub type ZH_TW = fake::locales::ZH_TW;

#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Copy, Serialize, Deserialize, PartialEq, Eq, Hash, Debug, Clone)]
pub enum LocaleConfig {
    EN,
    RU,
    ZH_TW,
}

impl Default for LocaleConfig {
    // We need some method to take default for all tables from config
    fn default() -> Self {
        Self::EN
    }
}

pub trait Localized {
    fn locale(&self) -> Option<LocaleConfig>;
    fn set_locale(&mut self, _l: Option<LocaleConfig>);
}

pub trait LocalizedFaker<V>: Localized {
    fn fake<L: ExtData>(&self, l: L) -> V;

    fn localized_fake(&self) -> V {
        match self.locale().unwrap_or_else(LocaleConfig::default) {
            LocaleConfig::EN => self.fake(EN {}),
            LocaleConfig::RU => self.fake(RU {}),
            LocaleConfig::ZH_TW => self.fake(ZH_TW {}),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialization() {
        let l: LocaleConfig = serde_yaml::from_str("RU").unwrap();
        assert_eq!(l, LocaleConfig::RU);
    }
}
