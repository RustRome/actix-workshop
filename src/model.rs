use diesel;
use diesel::prelude::*;
use diesel::{QueryResult, SqliteConnection};
use serde::{Deserialize, Serialize};

mod schema {

    table! {

        contacts {
            id -> Integer,
            name -> Text,
            email -> Text,
        }

    }
}

pub use self::schema::*;
use schema::contacts::dsl::{contacts as all_contacts, email};

#[table_name = "contacts"]
#[derive(Deserialize, Insertable, Debug)]
pub struct NewContact {
    name: String,
    email: String,
}

#[derive(Serialize, Queryable, Debug)]
pub struct Contact {
    id: i32,
    name: String,
    email: String,
}

impl Contact {
    pub fn create(contact: NewContact, conn: &SqliteConnection) -> QueryResult<Contact> {
        diesel::insert_into(contacts::table)
            .values(&contact)
            .execute(conn)?;

        all_contacts
            .filter(email.eq(contact.email))
            .get_result(conn)
    }

    pub fn get_contact(id: i32, conn: &SqliteConnection) -> QueryResult<Option<Contact>> {
        all_contacts.find(id).get_result(conn).optional()
    }

    pub fn list(conn: &SqliteConnection) -> QueryResult<Vec<Contact>> {
        all_contacts.order(contacts::id.desc()).load(conn)
    }

    pub fn delete(id: i32, conn: &SqliteConnection) -> QueryResult<usize> {
        diesel::delete(all_contacts.find(id)).execute(conn)
    }
}
