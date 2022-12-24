use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    #[serde(rename = "userId")]
    user_id: i32,
    id: Option<i32>,
    title: String,
    completed: bool,
}

#[tokio::main]

// RECIEVE JSON
// async fn main() -> Result<(), reqwest::Error> {
//     let todos: Vec<Todo> = reqwest::Client::new()
//         .get("https://jsonplaceholder.typicode.com/todos?userId=1")
//         .send()
//         .await?
//         .json()
//         .await?;

//     println!("{:?}", todos);
//     Ok(())
// }

//SEND JSON
async fn main() -> Result<(), reqwest::Error> {
    let new_todo = Todo {
        user_id: 1,
        id: None,
        title: "hello".to_string(),
        completed: false,
    };

    // Using struct
    let new_todo: Todo = reqwest::Client::new()
        .post("https://jsonplaceholder.typicode.com/todos")
        .json(&new_todo)
        .send()
        .await?
        .json()
        .await?;

    //Arbitrary JSON object
    // let new_todo: serde_json::Value = reqwest::Client::new()
    //     .post("https://jsonplaceholder.typicode.com/todos")
    //     .json(&serde_json::json!({
    //         "userId": 1,
    //         "title": "hello".to_string(),
    //         "completed": false
    //     }))
    //     .send()
    //     .await?
    //     .json()
    //     .await?;

    println!("{:?}", new_todo);

    Ok(())
}
