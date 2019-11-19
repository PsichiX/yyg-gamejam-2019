use std::path::PathBuf;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct ImageDesc {
    #[serde(default)]
    pub x: usize,
    #[serde(default)]
    pub y: usize,
    #[serde(default)]
    pub w: usize,
    #[serde(default)]
    pub h: usize,
    #[serde(default)]
    pub ox: i32,
    #[serde(default)]
    pub oy: i32,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct CartridgeManifest {
    #[serde(default = "CartridgeManifest::default_images_atlas_path")]
    pub images_atlas_path: PathBuf,
    #[serde(default)]
    pub images: Vec<ImageDesc>,
    #[serde(default = "CartridgeManifest::default_code_entry_path")]
    pub code_entry_path: PathBuf,
}

impl CartridgeManifest {
    fn default_images_atlas_path() -> PathBuf {
        "atlas.png".into()
    }

    fn default_code_entry_path() -> PathBuf {
        "main.kj".into()
    }
}
