
use postgres::{Client, NoTls};

struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>
}

fn main() {
    let mut conn = Client::connect("postgres://mrabouuj:QvhCQM5jwdFFh7kHdzPW2nN2uI7fxNYS@castor.db.elephantsql.com/mrabouuj", NoTls).unwrap();
           

    conn.query("CREATE TABLE pes (
                    id              SERIAL PRIMARY KEY,
                    name            VARCHAR NOT NULL,
                    data            BYTEA
                  )", &[]).unwrap();
    let me = Person {
        id: 0,
        name: "Steven".to_owned(),
        data: None
    };
    conn.query("INSERT INTO pes (name, data) VALUES ($1, $2)",
                 &[&me.name, &me.data]).unwrap();

    for row in &conn.query("SELECT id, name, data FROM pes", &[]).unwrap() {
        let person = Person {
            id: row.get(0),
            name: row.get(1),
            data: row.get(2)
        };
        println!("Found person {} with {}", person.name,person.id);
    }
}
