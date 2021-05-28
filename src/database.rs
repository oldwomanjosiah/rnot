#![allow(non_snake_case)]
use chrono::{DateTime, NaiveDateTime, Utc};

pub struct Sender {
    /// The primary key of the sender, assigned by SQLITE
    SenderId: i64,

    /// The name of the sending application, as sent by the application
    pub Name: String,

    /// Cached local path to the icon for this application, if it exists
    pub IconPath: Option<String>,

    /// Time since this applicaiton last notified the user
    pub LastNotifiedUTS: NaiveDateTime,
}

impl Sender {

    /// Add a new sender to the database with the following information and return it
    async fn new(name: String, icon_path: Option<String>, last_notified: DateTime<Utc>) -> Sender {
        unimplemented!("Sender::new not yet written");
    }

    async fn get_by_id(id: i64, conn: &mut crate::Connection) -> sqlx::Result<Option<Sender>> {
        sqlx::query_as!(
            Self,
            r#"
            SELECT * FROM Sender WHERE SenderId IS ?
            "#,
            id
        ).fetch_optional(conn).await
    }

    async fn get_by_name(name: &String, conn: &mut crate::Connection) -> sqlx::Result<Option<Sender>> {
        sqlx::query_as!(
            Self,
            r#"
            SELECT * FROM Sender WHERE Name LIKE ?
            "#,
            *name
        ).fetch_optional(conn).await
    }

    fn get_id(&self) -> i64 { self.SenderId }

    /// Update the database with these changes
    async fn commit_changes(&self, conn: &mut crate::Connection) -> sqlx::Result<&Self> {
        sqlx::query!(
            r#"
            UPDATE Sender
            SET
                Name = ?,
                IconPath = ?,
                LastNotifiedUTS = ?
            WHERE
                SenderId is ?;
            "#,
            self.Name, self.IconPath, self.LastNotifiedUTS, self.SenderId,
        ).execute(conn) .await
            .map(|_| self)
    }

    /// Pull changes on this object from the db
    async fn refresh(&mut self, conn: &mut crate::Connection) -> sqlx::Result<&mut Self> {
        match sqlx::query_as!(
            Self,
            r#"
            SELECT * FROM Sender WHERE SenderId = ?;
            "#,
            self.SenderId,
        ).fetch_optional(conn).await {
            Ok(new) => {
                if let Some(new) = new {
                *self = new;
                Ok(self)
                } else {
                    Err(sqlx::Error::RowNotFound)
                }
            },
            Err(e) => Err(e),
        }
    }

    /// Remove a sender from the database
    async fn remove(self, conn: &mut crate::Connection) {
        let _ = sqlx::query!("DELETE FROM Sender WHERE SenderId = ?;", self.SenderId)
            .execute(conn).await;
    }
}

pub struct Notification {
    NotificationId: u32,
    SenderId: u32,

    /// The title or summary of the notification
    Summary: String,

    /// The formatted body of the sent notification
    FormatBody: Option<String>,

    Received: DateTime<Utc>,

    /// The time in seconds that this notification should remain in the
    /// list before being removed, iff it is transient
    Timeout: Option<u32>,
}

pub struct Action {
    // These are unique together
    NotificationId: u32,
    ActionId: u32,

    /// The key to be broadcast if this action is taken by the user
    ActionKey: String,

    /// The display text for this action, formatted like [`Notification::FormatBody`]
    ActionFormatSummary: String,
}
