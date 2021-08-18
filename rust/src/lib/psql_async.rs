pub mod psql_async {
    use tokio_postgres::{Client, NoTls, Error};
    use postgres::types::ToSql;
    use crate::config::config;

    pub async fn db_connection() -> Result<Client, Error> {
        let (client, connection) =
            tokio_postgres::connect(config::db_url().as_str(), NoTls).await?;
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });
        Ok(client)
    }

    pub async fn query(query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<String, Error> {
        let client = db_connection().await.unwrap();
        let mut result = json::array![];
        for row in client.query(query, params).await? {
            let id: i64 = row.get(0);
            let name: String = row.get(1);
            println!("Groups, Id: {}, Name: {}", id, name);
            let mut data = json::JsonValue::new_object();
            data["Id"] = json::JsonValue::String(id.to_string());
            data["Name"] = json::JsonValue::String(name);
            match result.push(data) {
                Err(e) => println!("{}", e.to_string()),
                _ => {}
            }
        }
        Ok(json::stringify(result))
    }

    pub async fn get_muscle_groups() -> Result<String, Error> {
        let result = query("SELECT id, name FROM muscle_groups", &[]).await.unwrap();
        Ok(result)
    }

    pub async fn get_exercises() -> Result<String, Error> {
        let result = query("SELECT id, name FROM exercises", &[]).await.unwrap();
        Ok(result)
    }

    pub async fn get_exercises_for_muscle_group(id: &i64) -> Result<String, Error> {
        let result = query("SELECT e.id, e.name FROM exercises AS e \
                JOIN muscle_group_exercise AS m on e.id = m.exercise_id \
                WHERE m.muscle_group_id = $1 AND m.worked = true", &[id]).await.unwrap();
        Ok(result)
    }

    pub async fn get_muscle_groups_for_exercise(id: &i64) -> Result<String, Error> {
        let result = query("SELECT m.id, m.name FROM muscle_groups AS m \
                JOIN muscle_group_exercise AS me on m.id = me.muscle_group_id \
                WHERE me.exercise_id = $1 AND me.worked = true", &[id]).await.unwrap();
        Ok(result)
    }

    pub async fn get_data(link: &str, id: &i64) -> String {
        let result = match link {
            "muscle_groups" => get_muscle_groups().await.unwrap(),
            "exercises" => get_exercises().await.unwrap(),
            "exercises_for_muscle_group" => get_exercises_for_muscle_group(id).await.unwrap(),
            "muscle_groups_for_exercise" => get_muscle_groups_for_exercise(id).await.unwrap(),
            _ => "".to_string(),
        };
        return result;
    }
}