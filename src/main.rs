use surrealdb::Surreal;
use surrealdb::opt::auth::Root;
use surrealdb::sql::Value;
use surrealdb::Response;
use surrealdb::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Connect to the database
    let db = Surreal::new("localhost:8000").await?;

    // Sign in with root credentials
    db.signin(Root {
        username: "root",
        password: "password",
    }).await?;

    // Select a namespace and database
    db.use_ns("namespace").use_db("database").await?;

    // Perform a query with multiple results
    let query = "SELECT * FROM table_name;";
    let response: Response = db.query(query).await?;

    // Access the first result
    if let Some(Value::Array(results)) = response.take(0) {
        for result in results {
            println!("First result: {:?}", result);
        }
    }

    // Access the second result, if it exists
    if let Some(Value::Array(results)) = response.take(1) {
        for result in results {
            println!("Second result: {:?}", result);
        }
    }

    Ok(())
}
