use serde::{Deserialize, Serialize};

// TODO: add Custom type
#[derive(Clone, Default, Deserialize, Serialize)]
pub(crate) enum DownloadPathKind {
    Downloads,
    #[default]
    Relative,
}

#[derive(Clone, Default, Deserialize, Serialize)]
pub(crate) struct Account {
    pub(crate) name: String,
    pub(crate) cookie: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub(crate) struct Config {
    pub(crate) accounts: Vec<Account>,
    pub(crate) download_path_type: Option<DownloadPathKind>,
}

impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            accounts: Vec::new(),
            download_path_type: Some(DownloadPathKind::default()),
        }
    }
}
