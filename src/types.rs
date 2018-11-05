pub type ApiStore = std::collections::HashMap<String, Vec<Entry>>;
// type ProjectID = usize;
// type Tag = String;

// type ProjectName = String;
// type EntryName = String;

#[derive(Hash, PartialEq)]
pub struct ApiKey(pub String);

#[derive(Serialize, Deserialize)]
pub struct Project {}

#[derive(Serialize, Deserialize)]
pub struct Entry {}



