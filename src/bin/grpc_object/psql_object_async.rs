pub mod psql_object_async {
    use tokio_postgres::{Client, NoTls, Error};
    use postgres::types::ToSql;
    use api::api::db_url;
    use crate::muscle_exercises_object::{MuscleGroup, Exercise};

    pub mod muscle_exercises_object {
        tonic::include_proto!("muscle_exercises_object");
    }

    pub async fn db_connection() -> Result<Client, Error> {
        let (client, connection) =
            tokio_postgres::connect(db_url().as_str(), NoTls).await?;
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });
        Ok(client)
    }

    pub async fn query_muscle_groups(query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Vec<MuscleGroup>, Error> {
        let client = db_connection().await.unwrap();
        let mut result = Vec::new();
        for row in client.query(query, params).await? {
            let id: i64 = row.get(0);
            let name: String = row.get(1);
            println!("Groups, Id: {}, Name: {}", id, name);
            result.push(MuscleGroup { id, name,});
        }
        Ok(result)
    }

    pub async fn query_exercises(query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<Vec<Exercise>, Error> {
        let client = db_connection().await.unwrap();
        let mut result = Vec::new();
        for row in client.query(query, params).await? {
            let id: i64 = row.get(0);
            let name: String = row.get(1);
            println!("Groups, Id: {}, Name: {}", id, name);
            result.push(Exercise { id, name,});
        }
        Ok(result)
    }

    pub async fn get_muscle_groups() -> Result<Vec<MuscleGroup>, Error> {
        let result = query_muscle_groups("SELECT id, name FROM muscle_groups", &[]).await.unwrap();
        Ok(result)
    }

    pub async fn get_exercises() -> Result<Vec<Exercise>, Error> {
        let result = query_exercises("SELECT id, name FROM exercises", &[]).await.unwrap();
        Ok(result)
    }

    pub async fn get_exercises_for_muscle_group(id: &i64) -> Result<Vec<Exercise>, Error> {
        let result = query_exercises("SELECT e.id, e.name FROM exercises AS e \
                JOIN muscle_group_exercise AS m on e.id = m.exercise_id \
                WHERE m.muscle_group_id = $1 AND m.worked = true", &[id]).await.unwrap();
        Ok(result)
    }

    pub async fn get_muscle_groups_for_exercise(id: &i64) -> Result<Vec<MuscleGroup>, Error> {
        let result = query_muscle_groups("SELECT m.id, m.name FROM muscle_groups AS m \
                JOIN muscle_group_exercise AS me on m.id = me.muscle_group_id \
                WHERE me.exercise_id = $1 AND me.worked = true", &[id]).await.unwrap();
        Ok(result)
    }
}