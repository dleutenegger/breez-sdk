use crate::models::NodeState;

use super::{db::SqliteStorage, error::PersistResult};

impl SqliteStorage {
    pub fn get_cached_item(&self, key: String) -> PersistResult<Option<String>> {
        let res = self.get_connection()?.query_row(
            "SELECT value FROM cached_items WHERE key = ?1",
            [key],
            |row| row.get(0),
        );
        Ok(res.ok())
    }

    pub fn update_cached_item(&self, key: String, value: String) -> PersistResult<()> {
        self.get_connection()?.execute(
            "INSERT OR REPLACE INTO cached_items (key, value) VALUES (?1,?2)",
            (key, value),
        )?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn delete_cached_item(&self, key: String) -> PersistResult<()> {
        self.get_connection()?
            .execute("DELETE FROM cached_items WHERE key = ?1", [key])?;
        Ok(())
    }

    pub fn set_node_state(&self, state: &NodeState) -> PersistResult<()> {
        let serialized_state = serde_json::to_string(state)?;
        self.update_cached_item("node_state".to_string(), serialized_state)?;
        Ok(())
    }

    pub fn get_node_state(&self) -> PersistResult<Option<NodeState>> {
        let state_str = self.get_cached_item("node_state".to_string())?;
        Ok(match state_str {
            Some(str) => serde_json::from_str(str.as_str())?,
            None => None,
        })
    }

    pub fn set_last_backup_time(&self, t: u64) -> PersistResult<()> {
        self.update_cached_item("last_backup_time".to_string(), t.to_string())?;
        Ok(())
    }

    pub fn get_last_backup_time(&self) -> PersistResult<Option<u64>> {
        let state_str = self.get_cached_item("last_backup_time".to_string())?;
        Ok(match state_str {
            Some(str) => str.as_str().parse::<u64>().ok(),
            None => None,
        })
    }
    pub fn set_gl_credentials(&self, creds: Vec<u8>) -> PersistResult<()> {
        self.update_cached_item("gl_credentials".to_string(), hex::encode(creds))?;
        Ok(())
    }

    pub fn get_gl_credentials(&self) -> PersistResult<Option<Vec<u8>>> {
        match self.get_cached_item("gl_credentials".to_string())? {
            Some(str) => Ok(Some(hex::decode(str)?)),
            None => Ok(None),
        }
    }

    pub fn set_static_backup(&self, backup: Vec<String>) -> PersistResult<()> {
        let serialized_state = serde_json::to_string(&backup)?;
        self.update_cached_item("static_backup".to_string(), serialized_state)?;
        Ok(())
    }

    pub fn get_static_backup(&self) -> PersistResult<Option<Vec<String>>> {
        let backup_str = self.get_cached_item("static_backup".to_string())?;
        Ok(match backup_str {
            Some(str) => serde_json::from_str(str.as_str())?,
            None => None,
        })
    }
}

#[test]
fn test_cached_items() {
    use crate::persist::test_utils;

    let storage = SqliteStorage::new(test_utils::create_test_sql_dir());

    storage.init().unwrap();
    storage
        .update_cached_item("key1".to_string(), "val1".to_string())
        .unwrap();
    let item_value = storage.get_cached_item("key1".to_string()).unwrap();
    assert_eq!(item_value, Some("val1".to_string()));

    storage.delete_cached_item("key1".to_string()).unwrap();
    let item_value = storage.get_cached_item("key1".to_string()).unwrap();
    assert_eq!(item_value, None);
}
