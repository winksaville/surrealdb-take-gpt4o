use surrealdb::engine::local::Mem;
use surrealdb::Surreal;
use surrealdb::sql::Value;
use surrealdb::Response;
use surrealdb::Result;

#[derive(serde::Serialize)]
struct Person {
    name: String,
    age: i64,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Connect to the database
    let db = Surreal::new::<Mem>(()).await?;

    // Select a namespace and database
    db.use_ns("namespace").use_db("database").await?;

    // Add several people to the database
    let people = vec![
        Person { name: "Alice".to_string(), age: 30 },
        Person { name: "Bob".to_string(), age: 25 },
        Person { name: "Charlie".to_string(), age: 35 },
    ];

    for person in people {
        let query = format!(
            "CREATE person CONTENT {{ name: '{}', age: {} }};",
            person.name, person.age
        );
        db.query(&query).await?;
    }

    // Perform a query with multiple results
    let query = "SELECT * FROM person;";
    let mut response: Response = db.query(query).await?;
    // dbg!(&response);

    // Access the first result
    if let Ok(Value::Array(results)) = response.take(0) {
        for result in results {
            println!("Result: {:?}", result);
        }
    }

    Ok(())
}
