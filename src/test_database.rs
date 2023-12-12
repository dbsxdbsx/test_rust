use std::fs;

use anyhow::{Context, Result};
use rusqlite::types::{ToSqlOutput, Value};
use rusqlite::Connection;
use rusqlite::{params, ToSql};
use serde::{Deserialize, Serialize};
use tokio;
pub struct TestDatabase {
    conn: Connection,
    db_name: String,
}

impl TestDatabase {
    pub fn new(db_name: &str) -> Result<Self> {
        let conn = Connection::open(db_name).context("Failed to open the database")?;
        Ok(Self {
            conn,
            db_name: db_name.to_string(),
        })
    }

    pub fn disconnect(&mut self) -> Result<(), anyhow::Error> {
        let conn = std::mem::replace(&mut self.conn, Connection::open_in_memory().unwrap());
        conn.close().map_err(|(_, e)| anyhow::Error::new(e))
    }

    pub fn get_database_name(&self) -> &str {
        &self.db_name
    }

    pub fn get_tables_count(&self) -> Result<i64> {
        self.conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table'",
                params![],
                |row| row.get(0),
            )
            .map_err(|e| anyhow::Error::new(e))
    }

    pub fn get_items_count(&self, table_name: &str) -> Result<i64> {
        let query = format!("SELECT COUNT(*) FROM {}", table_name);
        self.conn
            .query_row(&query, params![], |row| row.get(0))
            .map_err(|e| anyhow::Error::new(e))
    }

    pub fn create_table(&self, table_name: &str, fields: &Vec<(&str, &str)>) -> Result<()> {
        let fields_str = fields
            .iter()
            .map(|(name, ty)| format!("{} {}", name, ty))
            .collect::<Vec<String>>()
            .join(", ");
        let query = format!("CREATE TABLE {} ({})", table_name, fields_str);
        self.conn
            .execute(&query, params![])
            .map_err(|e| anyhow::Error::new(e))
            .map(|_| ())
    }

    pub fn create_table_if_not_existed(
        &self,
        table_name: &str,
        fields: &Vec<(&str, &str)>,
    ) -> Result<()> {
        let fields_str = fields
            .iter()
            .map(|(name, ty)| format!("{} {}", name, ty))
            .collect::<Vec<String>>()
            .join(", ");
        let query = format!("CREATE TABLE IF NOT EXISTS {} ({})", table_name, fields_str);
        self.conn
            .execute(&query, params![])
            .map_err(|e| anyhow::Error::new(e))
            .map(|_| ())
    }
    // TODO: 这个方法有问题，暂时不用
    // pub fn add_item<T: ToSql>(&self, table_name: &str, item: &T, columns: &[&str]) -> Result<()> {
    //     let placeholders = std::iter::repeat("?")
    //         .take(columns.len())
    //         .collect::<Vec<_>>()
    //         .join(", ");
    //     let query = format!(
    //         "INSERT INTO {} ({}) VALUES ({})",
    //         table_name,
    //         columns.join(", "),
    //         placeholders
    //     );
    //     self.conn
    //         .execute(&query, params![item.to_sql()?])
    //         .map_err(|e| anyhow::Error::new(e))
    //         .map(|_| ())
    // }

    pub fn add_item<T: Serialize>(&self, table_name: &str, item: &T) -> Result<()> {
        let serialized_item = serde_json::to_value(item)?;
        let mut fields = Vec::new();
        let mut values = Vec::new();
        let mut placeholders = Vec::new();

        if let Some(map) = serialized_item.as_object() {
            for (key, value) in map {
                fields.push(key.as_str());
                placeholders.push("?".to_string());
                // 检查字段的类型
                if value.is_string() {
                    // 如果字段是字符串，直接使用字段的值
                    values.push(value.as_str().unwrap().to_string());
                } else {
                    // 否则，使用序列化后的值
                    values.push(value.to_string());
                }
            }
        }

        let query = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            table_name,
            fields.iter().map(|s| *s).collect::<Vec<&str>>().join(", "),
            placeholders.join(", ")
        );

        self.conn
            .execute(
                &query,
                values
                    .iter()
                    .map(|v| v as &dyn ToSql)
                    .collect::<Vec<_>>()
                    .as_slice(),
            )
            .map_err(|e| anyhow::Error::new(e))
            .map(|_| ())
    }

    pub fn delete_table(&self, table_name: &str) -> Result<()> {
        let query = format!("DROP TABLE IF EXISTS {}", table_name);
        self.conn
            .execute(&query, params![])
            .map_err(|e| anyhow::Error::new(e))
            .map(|_| ())
    }

    pub fn delete_table_data(&self, table_name: &str) -> Result<()> {
        let query = format!("DELETE FROM {}", table_name);
        self.conn
            .execute(&query, params![])
            .map_err(|e| anyhow::Error::new(e))
            .map(|_| ())
    }
    // TODO: use MeiliSearch instead?
    // pub fn find_item_by_name(&self, table_name: &str, name: &str) -> Result<Vec<User>> {
    //     let mut stmt = self
    //         .conn
    //         .prepare(&format!("SELECT * FROM {} WHERE name = ?", table_name))?;
    //     let rows = stmt.query_map(params![name], |row| {
    //         Ok(User {
    //             id: row.get(0)?,
    //             name: row.get(1)?,
    //             age: row.get(2)?,
    //             nick_names: row.get(3)?,
    //         })
    //     })?;

    //     let mut items = Vec::new();
    //     for item_result in rows {
    //         items.push(item_result?);
    //     }
    //     Ok(items)
    // }
}
impl Drop for TestDatabase {
    fn drop(&mut self) {
        let conn = std::mem::replace(&mut self.conn, Connection::open_in_memory().unwrap());
        conn.close().unwrap();
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub age: i32,
    nick_names: Vec<String>,
}
pub struct Controller {
    db: TestDatabase,
    client: meilisearch_sdk::Client,
}

impl Controller {
    pub fn new(db_name: &str, meilisearch_url: &str) -> Result<Self> {
        let db = TestDatabase::new(db_name)?;
        let client = meilisearch_sdk::Client::new(meilisearch_url, "masterKey".into());

        Ok(Self { db, client })
    }

    pub async fn insert_items(&mut self, table_name: &str, items: Vec<User>) -> Result<()> {
        for item in &items {
            self.db.add_item(table_name, item)?;
        }
        let index = self.client.index("users");
        index.add_documents(&items, Some("id")).await?;

        Ok(())
    }

    pub async fn search(&self, table_name: &str, query: &str) -> Result<Vec<User>> {
        let search_results = self
            .client
            .index(table_name)
            .search()
            .with_query(query)
            .execute::<User>()
            .await?;

        let results = search_results
            .hits
            .into_iter()
            .map(|hit| hit.result)
            .collect();

        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_operations() {
        let db_name = "test.db";
        let mut db = TestDatabase::new(db_name).unwrap();

        // 创建表
        let fields = vec![
            ("id", "INTEGER"),
            ("name", "TEXT"),
            ("age", "INTEGER"),
            ("nick_names", "TEXT"), // 新增字段
        ];
        db.create_table_if_not_existed("users", &fields).unwrap();
        assert_eq!(db.get_tables_count().unwrap(), 1);

        // 添加数据
        let user = User {
            id: 1,
            name: "Alice".to_string(),
            age: 20,
            nick_names: vec!["nickname1".to_string(), "nickname2".to_string()], // 新增字段
        };
        db.add_item("users", &user).unwrap();
        assert_eq!(db.get_items_count("users").unwrap(), 1);

        // 删除表数据
        db.delete_table_data("users").unwrap();
        assert_eq!(db.get_items_count("users").unwrap(), 0);

        // 删除表
        db.delete_table("users").unwrap();
        assert_eq!(db.get_tables_count().unwrap(), 0);

        // 删除测试数据库文件
        db.disconnect();
        fs::remove_file(db_name).unwrap();
    }

    #[tokio::test]
    async fn test_controller() {

    }
}
