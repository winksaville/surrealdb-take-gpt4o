use surrealdb::engine::local::Mem;
use surrealdb::Surreal;
use surrealdb::Response;
use surrealdb::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
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

    // Deserialize the results directly into Vec<Person>
    let persons: Vec<Person> = response.take(0)?;
    for person in persons {
        println!("Person: {:?}", person);
    }

    Ok(())
}
