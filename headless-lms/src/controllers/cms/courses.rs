//! Controllers for requests starting with `/api/v0/cms/cms/courses`.
use crate::{
    controllers::ApplicationResult,
    models::{
        courses::{Course, CourseUpdate, NewCourse},
        pages::Page,
    },
};
use actix_web::web::ServiceConfig;
use actix_web::web::{self, Json};
use sqlx::PgPool;
use std::str::FromStr;
use uuid::Uuid;

/**
GET `/api/v0/cms/courses` - Returns a list of all courses.

# Example
```json
[
    {
        "id": "a90c39f8-5d23-461f-8375-0b05a55d7ac1",
        "slug": "introduction-to-programming",
        "created_at": "2021-03-05T22:26:59.067294",
        "updated_at": "2021-03-05T22:26:59.067294",
        "name": "Introduction to Programming",
        "organization_id": "c6fbb0fe-b418-4156-8319-fc761d482dcb",
        "deleted": false
    }
]
```
 */
async fn get_all_courses(pool: web::Data<PgPool>) -> ApplicationResult<Json<Vec<Course>>> {
    let courses = crate::models::courses::all_courses(pool.get_ref()).await?;
    Ok(Json(courses))
}

/**
POST `/api/v0/cms/courses` - Create a new course.
# Example

Request:
```http
POST /api/v0/cms/courses HTTP/1.1
Content-Type: application/json

{
    "name": "Introduction to introduction",
    "slug": "introduction-to-introduction",
    "organization_id": "1b89e57e-8b57-42f2-9fed-c7a6736e3eec"
}
```

Response:
```json
{
    "id": "ab4541d8-6db4-4561-bdb2-45f35b2544a1",
    "slug": "introduction-to-introduction",
    "created_at": "2021-04-21T18:34:21.795388",
    "updated_at": "2021-04-21T18:34:21.795388",
    "name": "Introduction to introduction",
    "organization_id": "1b89e57e-8b57-42f2-9fed-c7a6736e3eec",
    "deleted": false
}
```
*/
async fn post_new_course(
    pool: web::Data<PgPool>,
    payload: web::Json<NewCourse>,
) -> ApplicationResult<Json<Course>> {
    let new_course = payload.0;
    let course = crate::models::courses::insert_course(&pool, new_course).await?;
    Ok(Json(course))
}

/**
POST `/api/v0/cms/courses/:course_id` - Update course.
# Example

Request:
```http
PUT /api/v0/cms/courses/ab4541d8-6db4-4561-bdb2-45f35b2544a1 HTTP/1.1
Content-Type: application/json

{
    "name": "Introduction to Introduction"
}

```

Response:
```json
{
    "id": "ab4541d8-6db4-4561-bdb2-45f35b2544a1",
    "slug": "introduction-to-introduction",
    "created_at": "2021-04-21T18:34:21.795388",
    "updated_at": "2021-04-21T18:49:21.398638",
    "name": "Introduction to Introduction",
    "organization_id": "1b89e57e-8b57-42f2-9fed-c7a6736e3eec",
    "deleted": false
}
```
*/
async fn update_course(
    payload: web::Json<CourseUpdate>,
    request_course_id: web::Path<String>,
    pool: web::Data<PgPool>,
) -> ApplicationResult<Json<Course>> {
    let course_id = Uuid::from_str(&request_course_id)?;

    let course_update = payload.0;
    let course =
        crate::models::courses::update_course(pool.get_ref(), course_id, course_update).await?;
    Ok(Json(course))
}

/**
DELETE `/api/v0/cms/courses/:course_id` - Delete a course.
# Example

```json
{
    "id": "ab4541d8-6db4-4561-bdb2-45f35b2544a1",
    "slug": "introduction-to-introduction",
    "created_at": "2021-04-21T18:34:21.795388",
    "updated_at": "2021-04-21T18:49:21.398638",
    "name": "Introduction to Introduction",
    "organization_id": "1b89e57e-8b57-42f2-9fed-c7a6736e3eec",
    "deleted": true
}
```
*/
async fn delete_course(
    request_course_id: web::Path<String>,
    pool: web::Data<PgPool>,
) -> ApplicationResult<Json<Course>> {
    let course_id = Uuid::from_str(&request_course_id)?;

    let course = crate::models::courses::delete_course(pool.get_ref(), course_id).await?;
    Ok(Json(course))
}

/**
GET `/api/v0/cms/courses/:course_id/pages` - Returns a list of pages in a course.
# Example
```json
[
    {
        "id": "86ac4f0a-ccca-464e-89f4-ed58969b1103",
        "created_at": "2021-03-05T22:50:47.920120",
        "updated_at": "2021-03-05T22:50:47.920120",
        "course_id": "a90c39f8-5d23-461f-8375-0b05a55d7ac1",
        "content": [
            {
                "id": "55be197d-4145-444a-bc1f-ee1091c47ad9"
            }
        ],
        "url_path": "/part-1/01-loops-and-variables",
        "title": "Loops and Variables",
        "deleted": false
    }
]
```
*/
async fn get_course_pages(
    request_course_id: web::Path<String>,
    pool: web::Data<PgPool>,
) -> ApplicationResult<Json<Vec<Page>>> {
    let course_id = Uuid::from_str(&request_course_id)?;

    let pages: Vec<Page> = crate::models::pages::course_pages(pool.get_ref(), course_id).await?;
    Ok(Json(pages))
}

/**
Add a route for each controller in this module.

The name starts with an underline in order to appear before other functions in the module documentation.

We add the routes by calling the route method instead of using the route annotations because this method preserves the function signatures for documentation.
*/
pub fn _add_courses_routes(cfg: &mut ServiceConfig) {
    cfg.route("", web::get().to(get_all_courses))
        .route("", web::post().to(post_new_course))
        .route("/{course_id}", web::put().to(update_course))
        .route("/{course_id}", web::delete().to(delete_course))
        .route("/{course_id}/pages", web::get().to(get_course_pages));
}
