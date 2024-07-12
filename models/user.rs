// Add this to your Cargo.toml
// cached = "0.26.2"

use cached::proc_macro::cached;
use cached::SizedCache;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct PollResult {
    poll_id: i32,
    option: String,
    votes: i32,
}

#[cached(
    type = "SizedCache<i32, Vec<PollResult>>",
    create = "{ SizedCache::with_size(100) }",
    convert = r#"{ poll_id }"#,
    result = true
)]
pub async fn get_poll_results(conn: &PgConnection, poll_id: i32) -> diesel::QueryResult<Vec<PollResult>> {
    use schema::votes;

    let results = votes::table
        .filter(votes::poll_id.eq(poll_id))
        .select((votes::poll_id, votes::choice, diesel::dsl::count(votes::id)))
        .group_by(votes::choice)
        .load::<PollResult>(conn);

    results
}