//! Controllers for requests starting with `/api/v0/course-material/courses`.

use std::{collections::HashMap, net::IpAddr, path::Path};

use actix_http::header;
use chrono::Utc;
use futures::{future::OptionFuture, FutureExt};
use headless_lms_utils::ip_to_country::IpToCountryMapper;
use isbot::Bots;
use models::{
    chapters::ChapterWithStatus,
    course_instances::CourseInstance,
    course_modules::CourseModule,
    courses,
    courses::Course,
    feedback,
    feedback::NewFeedback,
    glossary::Term,
    material_references::MaterialReference,
    page_visit_datum::NewPageVisitDatum,
    page_visit_datum_daily_visit_hashing_keys::{
        generate_anonymous_identifier, GenerateAnonymousIdentifierInput,
    },
    pages::{CoursePageWithUserData, Page, PageSearchRequest, PageSearchResult, PageVisibility},
    proposed_page_edits::{self, NewProposedPageEdits},
    user_course_settings::UserCourseSettings,
};

use crate::{
    domain::authorization::{authorize_access_to_course_material, skip_authorize},
    prelude::*,
};

/**
GET `/api/v0/course-material/courses/:course_id` - Get course.
*/
#[generated_doc]
#[instrument(skip(pool))]
async fn get_course(
    course_id: web::Path<Uuid>,
    pool: web::Data<PgPool>,
    user: AuthUser,
) -> ControllerResult<web::Json<Course>> {
    let mut conn = pool.acquire().await?;
    let course = models::courses::get_course(&mut conn, *course_id).await?;
    let token = skip_authorize()?;
    token.authorized_ok(web::Json(course))
}

/**
GET `/:course_slug/page-by-path/...` - Returns a course page by path

If the page has moved and there's a redirection, this will still return the moved page but the field `was_redirected` will indicate that the redirection happened. The new path can be found in the page object. The frontend is supposed to update the url of the page to the new location without reloading the page.

# Example
GET /api/v0/course-material/courses/introduction-to-everything/page-by-path//part-2/hello-world
*/
#[generated_doc]
#[instrument(skip(pool, ip_to_country_mapper, req))]
async fn get_course_page_by_path(
    params: web::Path<(String, String)>,
    pool: web::Data<PgPool>,
    user: Option<AuthUser>,
    ip_to_country_mapper: web::Data<IpToCountryMapper>,
    req: HttpRequest,
) -> ControllerResult<web::Json<CoursePageWithUserData>> {
    let mut conn = pool.acquire().await?;

    let (course_slug, raw_page_path) = params.into_inner();
    let path = if raw_page_path.starts_with('/') {
        raw_page_path
    } else {
        format!("/{}", raw_page_path)
    };

    let page_with_user_data = models::pages::get_page_with_user_data_by_path(
        &mut conn,
        user.as_ref().map(|u| u.id),
        &course_slug,
        &path,
    )
    .await?;

    let token = authorize_access_to_course_material(
        &mut conn,
        user.map(|u| u.id),
        page_with_user_data.page.course_id.ok_or_else(|| {
            ControllerError::new(
                ControllerErrorType::NotFound,
                "Course not found".to_string(),
                None,
            )
        })?,
    )
    .await?;

    let temp_request_information =
        derive_information_from_requester(req, ip_to_country_mapper).await?;

    let RequestInformation {
        ip,
        referrer,
        utm_tags,
        country,
        user_agent,
        has_bot_user_agent,
        browser_admits_its_a_bot,
        browser,
        browser_version,
        operating_system,
        operating_system_version,
        device_type,
    } = temp_request_information.data;

    let course_or_exam_id = page_with_user_data
        .page
        .course_id
        .unwrap_or_else(|| page_with_user_data.page.exam_id.unwrap_or_else(Uuid::nil));
    let anonymous_identifier = generate_anonymous_identifier(
        &mut conn,
        GenerateAnonymousIdentifierInput {
            user_agent,
            ip_address: ip.map(|ip| ip.to_string()).unwrap_or_default(),
            course_id: course_or_exam_id,
        },
    )
    .await?;

    models::page_visit_datum::insert(
        &mut conn,
        NewPageVisitDatum {
            course_id: page_with_user_data.page.course_id,
            page_id: page_with_user_data.page.id,
            country,
            browser,
            browser_version,
            operating_system,
            operating_system_version,
            device_type,
            referrer,
            is_bot: has_bot_user_agent || browser_admits_its_a_bot,
            utm_tags,
            anonymous_identifier,
            exam_id: page_with_user_data.page.exam_id,
        },
    )
    .await?;

    token.authorized_ok(web::Json(page_with_user_data))
}

struct RequestInformation {
    ip: Option<IpAddr>,
    user_agent: String,
    referrer: Option<String>,
    utm_tags: Option<serde_json::Value>,
    country: Option<String>,
    has_bot_user_agent: bool,
    browser_admits_its_a_bot: bool,
    browser: Option<String>,
    browser_version: Option<String>,
    operating_system: Option<String>,
    operating_system_version: Option<String>,
    device_type: Option<String>,
}

/// Used in get_course_page_by_path for path for anonymous visitor counts
async fn derive_information_from_requester(
    req: HttpRequest,
    ip_to_country_mapper: web::Data<IpToCountryMapper>,
) -> ControllerResult<RequestInformation> {
    let headers = req.headers();
    let user_agent = headers.get(header::USER_AGENT);
    let bots = Bots::default();
    let has_bot_user_agent = user_agent
        .and_then(|ua| ua.to_str().ok())
        .map(|ua| bots.is_bot(ua))
        .unwrap_or(true);
    // If this header is not set, the requester is considered a bot
    let header_totally_not_a_bot = headers.get("totally-not-a-bot");
    let browser_admits_its_a_bot = header_totally_not_a_bot.is_none();
    if has_bot_user_agent || browser_admits_its_a_bot {
        warn!(
            ?has_bot_user_agent,
            ?browser_admits_its_a_bot,
            ?user_agent,
            ?header_totally_not_a_bot,
            "The requester is a bot"
        )
    }

    let user_agent_parser = woothee::parser::Parser::new();
    let parsed_user_agent = user_agent
        .and_then(|ua| ua.to_str().ok())
        .and_then(|ua| user_agent_parser.parse(ua));

    let ip: Option<IpAddr> = req
        .connection_info()
        .realip_remote_addr()
        .and_then(|ip| ip.parse::<IpAddr>().ok());

    let country = ip
        .and_then(|ip| ip_to_country_mapper.map_ip_to_country(&ip))
        .map(|c| c.to_string());

    let utm_tags = headers
        .get("utm-tags")
        .and_then(|utms| utms.to_str().ok())
        .and_then(|utms| serde_json::to_value(utms).ok());
    let referrer = headers
        .get("Orignal-Referrer")
        .and_then(|r| r.to_str().ok())
        .map(|r| r.to_string());

    let browser = parsed_user_agent.as_ref().map(|ua| ua.name.to_string());
    let browser_version = parsed_user_agent.as_ref().map(|ua| ua.version.to_string());
    let operating_system = parsed_user_agent.as_ref().map(|ua| ua.os.to_string());
    let operating_system_version = parsed_user_agent
        .as_ref()
        .map(|ua| ua.os_version.to_string());
    let device_type = parsed_user_agent.as_ref().map(|ua| ua.category.to_string());
    let token = skip_authorize()?;
    token.authorized_ok(RequestInformation {
        ip,
        user_agent: user_agent
            .and_then(|ua| ua.to_str().ok())
            .unwrap_or_default()
            .to_string(),
        referrer,
        utm_tags,
        country,
        has_bot_user_agent,
        browser_admits_its_a_bot,
        browser,
        browser_version,
        operating_system,
        operating_system_version,
        device_type,
    })
}

/**
GET `/api/v0/course-material/courses/:course_id/current-instance` - Returns the instance of a course for the current user, if there is one.
*/
#[generated_doc]
#[instrument(skip(pool))]
async fn get_current_course_instance(
    pool: web::Data<PgPool>,
    course_id: web::Path<Uuid>,
    user: Option<AuthUser>,
) -> ControllerResult<web::Json<Option<CourseInstance>>> {
    let mut conn = pool.acquire().await?;
    if let Some(user) = user {
        let instance = models::course_instances::current_course_instance_of_user(
            &mut conn, user.id, *course_id,
        )
        .await?;
        let token = skip_authorize()?;
        token.authorized_ok(web::Json(instance))
    } else {
        Err(ControllerError::new(
            ControllerErrorType::NotFound,
            "User not found".to_string(),
            None,
        ))
    }
}

/**
GET `/api/v0/course-material/courses/:course_id/course-instances` - Returns all course instances for given course id.
*/
#[generated_doc]
async fn get_course_instances(
    pool: web::Data<PgPool>,
    course_id: web::Path<Uuid>,
) -> ControllerResult<web::Json<Vec<CourseInstance>>> {
    let mut conn = pool.acquire().await?;
    let instances =
        models::course_instances::get_course_instances_for_course(&mut conn, *course_id).await?;
    let token = skip_authorize()?;
    token.authorized_ok(web::Json(instances))
}

/**
GET `/api/v0/course-material/courses/:course_id/pages` - Returns a list of public pages on a course.

Since anyone can access this endpoint, any unlisted pages are omited from these results.
*/
#[generated_doc]
#[instrument(skip(pool))]
async fn get_public_course_pages(
    course_id: web::Path<Uuid>,
    pool: web::Data<PgPool>,
) -> ControllerResult<web::Json<Vec<Page>>> {
    let mut conn = pool.acquire().await?;
    let pages: Vec<Page> = models::pages::get_all_by_course_id_and_visibility(
        &mut conn,
        *course_id,
        PageVisibility::Public,
    )
    .await?;
    let token = skip_authorize()?;
    token.authorized_ok(web::Json(pages))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "ts_rs", derive(TS))]
pub struct ChaptersWithStatus {
    pub is_previewable: bool,
    pub modules: Vec<CourseMaterialCourseModule>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "ts_rs", derive(TS))]
pub struct CourseMaterialCourseModule {
    pub chapters: Vec<ChapterWithStatus>,
    pub id: Uuid,
    pub is_default: bool,
    pub name: Option<String>,
    pub order_number: i32,
}

/**
GET `/api/v0/course-material/courses/:course_id/chapters` - Returns a list of chapters in a course.
*/
#[generated_doc]
#[instrument(skip(pool, file_store, app_conf))]
async fn get_chapters(
    course_id: web::Path<Uuid>,
    user: Option<AuthUser>,
    pool: web::Data<PgPool>,
    file_store: web::Data<dyn FileStore>,
    app_conf: web::Data<ApplicationConfiguration>,
) -> ControllerResult<web::Json<ChaptersWithStatus>> {
    let mut conn = pool.acquire().await?;
    let is_previewable = OptionFuture::from(user.map(|u| {
        authorize(&mut conn, Act::Teach, Some(u.id), Res::Course(*course_id)).map(|r| r.ok())
    }))
    .await
    .is_some();
    let token = skip_authorize()?;
    let course_modules = models::course_modules::get_by_course_id(&mut conn, *course_id).await?;
    let chapters = models::chapters::course_chapters(&mut conn, *course_id)
        .await?
        .into_iter()
        .map(|chapter| {
            let chapter_image_url = chapter
                .chapter_image_path
                .as_ref()
                .map(|path| file_store.get_download_url(Path::new(&path), &app_conf));
            ChapterWithStatus::from_database_chapter_timestamp_and_image_url(
                chapter,
                Utc::now(),
                chapter_image_url,
            )
        })
        .collect();
    let modules = collect_course_modules(course_modules, chapters)?.data;
    token.authorized_ok(web::Json(ChaptersWithStatus {
        is_previewable,
        modules,
    }))
}

/// Combines course modules and chapters, consuming them.
fn collect_course_modules(
    course_modules: Vec<CourseModule>,
    chapters: Vec<ChapterWithStatus>,
) -> ControllerResult<Vec<CourseMaterialCourseModule>> {
    let mut course_modules: HashMap<Uuid, CourseMaterialCourseModule> = course_modules
        .into_iter()
        .map(|course_module| {
            (
                course_module.id,
                CourseMaterialCourseModule {
                    chapters: vec![],
                    id: course_module.id,
                    is_default: course_module.name.is_none(),
                    name: course_module.name,
                    order_number: course_module.order_number,
                },
            )
        })
        .collect();
    for chapter in chapters {
        course_modules
            .get_mut(&chapter.course_module_id)
            .ok_or_else(|| {
                ControllerError::new(
                    ControllerErrorType::InternalServerError,
                    "Module data mismatch.".to_string(),
                    None,
                )
            })?
            .chapters
            .push(chapter);
    }
    let token = skip_authorize()?;
    token.authorized_ok(course_modules.into_values().collect())
}

/**
GET `/api/v0/course-material/courses/:course_id/user-settings` - Returns user settings for the current course.
*/
#[generated_doc]
#[instrument(skip(pool))]
async fn get_user_course_settings(
    pool: web::Data<PgPool>,
    course_id: web::Path<Uuid>,
    user: Option<AuthUser>,
) -> ControllerResult<web::Json<Option<UserCourseSettings>>> {
    let mut conn = pool.acquire().await?;
    if let Some(user) = user {
        let settings = models::user_course_settings::get_user_course_settings_by_course_id(
            &mut conn, user.id, *course_id,
        )
        .await?;
        let token = skip_authorize()?;
        token.authorized_ok(web::Json(settings))
    } else {
        Err(ControllerError::new(
            ControllerErrorType::NotFound,
            "User not found".to_string(),
            None,
        ))
    }
}

/**
POST `/api/v0/course-material/courses/:course_id/search-pages-with-phrase` - Returns a list of pages given a search query.

Provided words are supposed to appear right after each other in the source document.

# Example

Request:

```http
POST /api/v0/course-material/courses/1a68e8b0-d151-4c0e-9307-bb154e9d2be1/search-pages-with-phrase HTTP/1.1
Content-Type: application/json

{
  "query": "Everything"
}
```
*/
#[generated_doc]
#[instrument(skip(pool))]
async fn search_pages_with_phrase(
    course_id: web::Path<Uuid>,
    payload: web::Json<PageSearchRequest>,
    pool: web::Data<PgPool>,
) -> ControllerResult<web::Json<Vec<PageSearchResult>>> {
    let mut conn = pool.acquire().await?;
    let res =
        models::pages::get_page_search_results_for_phrase(&mut conn, *course_id, &payload).await?;
    let token = skip_authorize()?;
    token.authorized_ok(web::Json(res))
}

/**
POST `/api/v0/course-material/courses/:course_id/search-pages-with-words` - Returns a list of pages given a search query.

Provided words can appear in any order in the source document.

# Example

Request:

```http
POST /api/v0/course-material/courses/1a68e8b0-d151-4c0e-9307-bb154e9d2be1/search-pages-with-words HTTP/1.1
Content-Type: application/json

{
  "query": "Everything"
}
```
*/
#[generated_doc]
#[instrument(skip(pool))]
async fn search_pages_with_words(
    course_id: web::Path<Uuid>,
    payload: web::Json<PageSearchRequest>,
    pool: web::Data<PgPool>,
) -> ControllerResult<web::Json<Vec<PageSearchResult>>> {
    let mut conn = pool.acquire().await?;
    let res =
        models::pages::get_page_search_results_for_words(&mut conn, *course_id, &payload).await?;
    let token = skip_authorize()?;
    token.authorized_ok(web::Json(res))
}

/**
POST `/api/v0/course-material/courses/:course_id/feedback` - Creates new feedback.
*/
#[generated_doc]
pub async fn feedback(
    course_id: web::Path<Uuid>,
    new_feedback: web::Json<Vec<NewFeedback>>,
    pool: web::Data<PgPool>,
    user: Option<AuthUser>,
) -> ControllerResult<web::Json<Vec<Uuid>>> {
    let mut conn = pool.acquire().await?;
    let fs = new_feedback.into_inner();
    let user_id = user.as_ref().map(|u| u.id);

    // validate
    for f in &fs {
        if f.feedback_given.len() > 1000 {
            return Err(ControllerError::new(
                ControllerErrorType::BadRequest,
                "Feedback given too long: max 1000".to_string(),
                None,
            ));
        }
        if f.related_blocks.len() > 100 {
            return Err(ControllerError::new(
                ControllerErrorType::BadRequest,
                "Too many related blocks: max 100".to_string(),
                None,
            ));
        }
        for block in &f.related_blocks {
            if block.text.as_ref().map(|t| t.len()).unwrap_or_default() > 10000 {
                return Err(ControllerError::new(
                    ControllerErrorType::BadRequest,
                    "Block text too long: max 10000".to_string(),
                    None,
                ));
            }
        }
    }

    let mut tx = conn.begin().await?;
    let mut ids = vec![];
    for f in fs {
        let id = feedback::insert(&mut tx, PKeyPolicy::Generate, user_id, *course_id, f).await?;
        ids.push(id);
    }
    tx.commit().await?;
    let token = skip_authorize()?;
    token.authorized_ok(web::Json(ids))
}

/**
POST `/api/v0/course-material/courses/:course_slug/edit` - Creates a new edit proposal.
*/
#[generated_doc]
async fn propose_edit(
    course_slug: web::Path<String>,
    edits: web::Json<NewProposedPageEdits>,
    pool: web::Data<PgPool>,
    user: Option<AuthUser>,
) -> ControllerResult<web::Json<Uuid>> {
    let mut conn = pool.acquire().await?;
    let course = courses::get_course_by_slug(&mut conn, course_slug.as_str()).await?;
    let (id, _) = proposed_page_edits::insert(
        &mut conn,
        PKeyPolicy::Generate,
        course.id,
        user.map(|u| u.id),
        &edits.into_inner(),
    )
    .await?;
    let token = skip_authorize()?;
    token.authorized_ok(web::Json(id))
}

#[generated_doc]
#[instrument(skip(pool))]
async fn glossary(
    pool: web::Data<PgPool>,
    course_id: web::Path<Uuid>,
) -> ControllerResult<web::Json<Vec<Term>>> {
    let mut conn = pool.acquire().await?;
    let glossary = models::glossary::fetch_for_course(&mut conn, *course_id).await?;
    let token = skip_authorize()?;
    token.authorized_ok(web::Json(glossary))
}

#[generated_doc]
#[instrument(skip(pool))]
async fn get_material_references_by_course_id(
    course_id: web::Path<Uuid>,
    pool: web::Data<PgPool>,
    user: AuthUser,
) -> ControllerResult<web::Json<Vec<MaterialReference>>> {
    let mut conn = pool.acquire().await?;
    let token = authorize(&mut conn, Act::View, Some(user.id), Res::Course(*course_id)).await?;
    let res =
        models::material_references::get_references_by_course_id(&mut conn, *course_id).await?;

    token.authorized_ok(web::Json(res))
}

/**
GET /api/v0/course-material/courses/:course_id/top-level-pages
*/
#[generated_doc]
#[instrument(skip(pool))]
async fn get_public_top_level_pages(
    course_id: web::Path<Uuid>,
    pool: web::Data<PgPool>,
) -> ControllerResult<web::Json<Vec<Page>>> {
    let mut conn = pool.acquire().await?;
    let page = models::pages::get_course_top_level_pages_by_course_id_and_visibility(
        &mut conn,
        *course_id,
        PageVisibility::Public,
    )
    .await?;
    let token = skip_authorize()?;
    token.authorized_ok(web::Json(page))
}

/**
Add a route for each controller in this module.

The name starts with an underline in order to appear before other functions in the module documentation.

We add the routes by calling the route method instead of using the route annotations because this method preserves the function signatures for documentation.
*/
pub fn _add_routes(cfg: &mut ServiceConfig) {
    cfg.route("/{course_id}", web::get().to(get_course))
        .route("/{course_id}/chapters", web::get().to(get_chapters))
        .route(
            "/{course_id}/course-instances",
            web::get().to(get_course_instances),
        )
        .route(
            "/{course_id}/current-instance",
            web::get().to(get_current_course_instance),
        )
        .route("/{course_id}/feedback", web::post().to(feedback))
        .route(
            "/{course_id}/page-by-path/{url_path:.*}",
            web::get().to(get_course_page_by_path),
        )
        .route("/{course_id}/pages", web::get().to(get_public_course_pages))
        .route(
            "/{course_id}/search-pages-with-phrase",
            web::post().to(search_pages_with_phrase),
        )
        .route(
            "/{course_id}/search-pages-with-words",
            web::post().to(search_pages_with_words),
        )
        .route(
            "/{course_id}/user-settings",
            web::get().to(get_user_course_settings),
        )
        .route(
            "/{course_id}/top-level-pages",
            web::get().to(get_public_top_level_pages),
        )
        .route("/{course_id}/propose-edit", web::post().to(propose_edit))
        .route("/{course_id}/glossary", web::get().to(glossary))
        .route(
            "/{course_id}/references",
            web::get().to(get_material_references_by_course_id),
        );
}
