use freedesktop_entry_parser::Entry;
use lazy_static::lazy_static;
use rustbreak::{deser::Bincode, MemoryDatabase};
use serde::{Deserialize, Serialize};
use findex::AppInfo;

lazy_static! {
    pub static ref DB: MemoryDatabase<Vec<DBAppInfo>, Bincode> =
        MemoryDatabase::memory(Vec::new()).unwrap();
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DBAppInfo {
    pub name: String,
    pub exec: String,
    pub icon: String,
    pub desktop_file: String,
}

impl DBAppInfo {
    pub(crate) fn from_freedesktop_entry(entry: &Entry, path: &str) -> Result<Self, String> {
        let parameter_regex = regex::Regex::new("%.").unwrap();
        let section = entry.section("Desktop Entry");
        let name = match section.attr("Name") {
            Some(n) => n.to_string(),
            None => {
                return Err("Cannot find 'Name' field".to_string())
            }
        };
        let icon = section.attr("Icon").unwrap_or("applications-other").to_string();
        let exec = match section.attr("Exec") {
            Some(e) => parameter_regex.replace_all(e, "").to_string(),
            None => return Err("Cannot find 'Exec' field".to_string()),
        };

        Ok(Self {
            name,
            exec,
            icon,
            desktop_file: path.to_string()
        })
    }

    pub(crate) fn to_appinfo(&self, total_score: f64) -> AppInfo {
        AppInfo {
            name: self.name.clone(),
            exec: self.exec.clone(),
            icon: self.icon.clone(),
            total_score,
        }
    }
}