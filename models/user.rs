use chrono::{NaiveDateTime, Utc};
use diesel::{Insertable, Queryable, RunQueryDsl, ExpressionMethods};
use diesel::pg::PgConnection;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;

table! {
    polls (id) {
        id -> Integer,
        title -> Varchar,
        description -> Text,
        start_time -> Timestamp,
        end_time -> Timestamp,
    }
}

table! {
    votes (id) {
        id -> Integer,
        poll_id -> Integer,
        user_id -> Integer,
        choice -> Varchar,
        timestamp -> Timestamp,
    }
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Poll {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "polls"]
pub struct NewPoll<'a> {
    pub title: &'a str,
    pub description: &'a str,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Vote {
    pub id: i32,
    pub poll_id: i32,
    pub user_id: i32,
    pub choice: String,
    pub timestamp: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "votes"]
pub struct NewVote<'a> {
    pub poll_id: i32,
    pub user_id: i32,
    pub choice: &'a str,
}

pub fn create_poll<'a>(conn: &PgConnection, title: &'a str, description: &'a str, start_time: NaiveDateTime, end_time: NaiveDateTime) -> Poll {
    use schema::polls;

    let new_poll = NewPoll {
        title,
        description,
        start_time,
        end_time,
    };

    diesel::insert_into(polls::table)
        .values(&new_poll)
        .get_result(conn)
        .expect("Error creating new poll")
}

pub fn create_vote<'a>(conn: &PgConnection, poll_id: i32, user_id: i32, choice: &'a str) -> Vote {
    use schema::votes;

    let new_vote = NewVote {
        poll_id,
        user_id,
        choice,
    };

    diesel::insert_into(votes::table)
        .values(&new_vote)
        .get_result(conn)
        .expect("Error casting vote")
}