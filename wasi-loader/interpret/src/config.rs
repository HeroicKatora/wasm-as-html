use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub input: Input,
    pub output: Output,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Input {
    pub env: Vec<String>,
    pub root: Option<FsInMode>,
    /// Identifies the name of the data section to use.
    ///
    /// The usual name is `wah_polyglot_stage2_data`, the default name for the corresponding
    /// packer.
    pub data_section: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FsInMode {
    Unzip,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Output {
    pub stdin: Option<FdOutMode>,
    pub stdout: Option<FdOutMode>,
    pub stderr: Option<FdOutMode>,
    pub root: Option<FsOutMode>,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FdOutMode {
    Data,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FsOutMode {
    Tree,
}
