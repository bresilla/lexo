#![allow(dead_code)]

extern crate getset;
use getset::{CopyGetters, Getters, MutGetters, Setters};

#[derive(Serialize, Deserialize, Debug, Clone, CopyGetters, Getters, MutGetters, Setters)]
#[getset(get = "pub", set = "pub", get_mut = "pub")]
pub struct SCHEME {
    #[serde(skip)]
    library: Option<String>,
    #[serde(skip)]
    config: Option<String>,
}

impl SCHEME {
    pub fn init() -> Self {
        Self {
            library: None,
            config: None,
        }
    }
    pub fn modi(&mut self, new: &SCHEME) -> &Self {
        if let Some(value) = new.library() { self.library = Some(value.clone()); }
        if let Some(value) = new.config() { self.config = Some(value.clone()); }
        self
    }
}
