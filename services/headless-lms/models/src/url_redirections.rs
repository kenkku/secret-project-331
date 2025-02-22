use crate::prelude::*;

pub async fn insert(
    conn: &mut PgConnection,
    pkey_policy: PKeyPolicy<Uuid>,
    destination_page_id: Uuid,
    old_url_path: &str,
    course_id: Uuid,
) -> ModelResult<Uuid> {
    let res = sqlx::query!(
        "
INSERT INTO url_redirections (id, destination_page_id, old_url_path, course_id)
VALUES ($1, $2, $3, $4)
RETURNING id
        ",
        pkey_policy.into_uuid(),
        destination_page_id,
        old_url_path,
        course_id,
    )
    .fetch_one(conn)
    .await?;
    Ok(res.id)
}
