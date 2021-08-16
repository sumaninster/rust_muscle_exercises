pub mod psql {
    use postgres::{Client, NoTls, Error};
    use postgres::types::ToSql;
    use crate::config::config;

    pub struct Db;

    impl Db {
        pub fn db_connection(&self) -> Result<Client, Error> {
            let client = Client::connect(config::db_url().as_str(), NoTls)?;
            Ok(client)
        }

        pub fn query(&self, query: &str, params: &[&(dyn ToSql + Sync)]) -> Result<String, Error> {
            let mut client = self.db_connection()?;
            let mut result = json::array![];
            for row in client.query(query, params)? {
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

        pub fn get_muscle_groups(&self) -> Result<String, Error> {
            let result = self.query("SELECT id, name FROM muscle_groups", &[])?;
            Ok(result)
        }

        pub fn get_exercises(&self) -> Result<String, Error> {
            let result = self.query("SELECT id, name FROM exercises", &[])?;
            Ok(result)
        }

        pub fn get_exercises_for_muscle_group(&self, id: &i64) -> Result<String, Error> {
            let result = self.query("SELECT e.id, e.name FROM exercises AS e \
                JOIN muscle_group_exercise AS m on e.id = m.exercise_id \
                WHERE m.muscle_group_id = $1 AND m.worked = true", &[id])?;
            Ok(result)
        }

        pub fn get_muscle_groups_for_exercise(&self, id: &i64) -> Result<String, Error> {
            let result = self.query("SELECT m.id, m.name FROM muscle_groups AS m \
                JOIN muscle_group_exercise AS me on m.id = me.muscle_group_id \
                WHERE me.exercise_id = $1 AND me.worked = true", &[id])?;
            Ok(result)
        }

        pub fn get_data(&self, link: &str, id: &i64) -> String {
            let result = match link {
                "muscle_groups" => self.get_muscle_groups(),
                "exercises" => self.get_exercises(),
                "exercises_for_muscle_group" => self.get_exercises_for_muscle_group(id),
                "muscle_groups_for_exercise" => self.get_muscle_groups_for_exercise(id),
                _ => Ok("".to_string())
            };
            let data = match result {
                Ok(data) => data,
                Err(error) => panic!("Problem connecting database: {:?}", error),
            };
            return data;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test1() {
        let db = psql::Db{};
        let result = db.get_exercises_for_muscle_group(&1);

        match result {
            Ok(client) => client,
            Err(error) => panic!("Problem connecting database: {:?}", error),
        };
    }
}