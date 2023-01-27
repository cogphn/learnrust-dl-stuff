use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Root {
    pub feeds: Vec<Feed>,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Feed {
    pub name: String,
    pub url: String,
    pub outfile_name: String,
    pub format: String,
    pub cooldown: i32
}

