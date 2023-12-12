use crate::test_database::TestDatabase;

mod test_database;

fn main() {
    // let db = TestDatabase::new("Mytest.db").unwrap();
    // println!("Database name: {}", db.get_database_name());
    // println!("Tables count: {}", db.get_tables_count().unwrap());
    // println!(
    //     "Items count: {}",
    //     db.get_items_count("users").unwrap_or_else(|_| {
    //         println!("No such table or the table name is incorrect.");
    //         0
    //     })
    // );

    // // Add test table 'users'
    // let fields = vec![("id", "INTEGER"), ("name", "TEXT"), ("age", "INTEGER")];
    // db.create_table_if_not_existed("users", &fields).unwrap();
    // println!("Items count: {}", db.get_items_count("users").unwrap());

    // let user = test_database::User {
    //     id: 1,
    //     name: "Alice".to_string(),
    //     age: 20,
    // };
    // db.add_item("users", &user).unwrap();
    // println!("Items count: {}", db.get_items_count("users").unwrap());
}
