//! Database query helpers placeholder.

/// Placeholder query to fetch a user by id.
pub fn get_user_by_id_query(_id: i64) -> &'static str {
    "SELECT id, line_id, name FROM users WHERE id = $1"
}
