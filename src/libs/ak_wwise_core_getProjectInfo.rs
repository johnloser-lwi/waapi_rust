use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Directory {
    pub cache: String,
    pub commands: String,
    pub originals: String,
    pub properties: String,
    pub root: String,
    pub soundBankOutputRoot: String,
}

#[derive(Serialize, Deserialize)]
pub struct Language {
    pub id: String,
    pub name: String,
    pub shortId: u64,
}

#[derive(Serialize, Deserialize)]
pub struct Platform {
    pub baseDisplayName: String,
    pub baseName: String,
    pub copiedMediaPath: String,
    pub id: String,
    pub name: String,
    pub soundBankPath: String,
}

#[derive(Serialize, Deserialize)]
pub struct Conversion {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Result {
    pub currentLanguageId: String,
    pub currentPlatformId: String,
    pub defaultConversion: Conversion,
    pub directories: Directory,
    pub displayTitle: String,
    pub id: String,
    pub isDirty: bool,
    pub languages: Vec<Language>,
    pub name: String,
    pub path: String,
    pub platforms: Vec<Platform>,
    pub referenceLanguageId: String,
}