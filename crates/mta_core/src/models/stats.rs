use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct SystemStats {
    pub total_users: i64,
    pub total_rooms: i64,
    pub mage_sheets_public: i64,
    pub mage_sheets_private: i64,
    pub gm_sheets_public: i64,
    pub gm_sheets_private: i64,
}

impl SystemStats {
    pub fn total_mage(&self) -> i64 {
        self.mage_sheets_public + self.mage_sheets_private
    }

    pub fn total_gm(&self) -> i64 {
        self.gm_sheets_public + self.gm_sheets_private
    }

    pub fn total_sheets(&self) -> i64 {
        self.total_mage() + self.total_gm()
    }

    pub fn total_public(&self) -> i64 {
        self.mage_sheets_public + self.gm_sheets_public
    }

    pub fn total_private(&self) -> i64 {
        self.mage_sheets_private + self.gm_sheets_private
    }
}
