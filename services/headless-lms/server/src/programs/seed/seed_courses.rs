use std::sync::Arc;

use crate::domain::models_requests::{self, JwtKey};
use crate::programs::seed::seed_helpers::{
    create_best_exercise, create_page, example_exercise_flexible, paragraph, quizzes_exercise,
    submit_and_grade,
};
use anyhow::Result;
use chrono::{TimeZone, Utc};

use headless_lms_models::pages::PageUpdateArgs;
use headless_lms_models::{
    chapters,
    chapters::NewChapter,
    course_instance_enrollments,
    course_instance_enrollments::NewCourseInstanceEnrollment,
    course_instances::{self, NewCourseInstance},
    course_modules::{self, NewCourseModule},
    courses::NewCourse,
    feedback,
    feedback::{FeedbackBlock, NewFeedback},
    glossary, library,
    library::content_management::CreateNewCourseFixedIds,
    page_history::HistoryChangeReason,
    pages::CmsPageUpdate,
    pages::{self, NewCoursePage},
    proposed_block_edits::NewProposedBlockEdit,
    proposed_page_edits,
    proposed_page_edits::NewProposedPageEdits,
    url_redirections, PKeyPolicy,
};
use headless_lms_utils::{attributes, document_schema_processor::GutenbergBlock};

use sqlx::{Pool, Postgres};
use tracing::info;
use uuid::Uuid;

use super::seed_helpers::heading;

#[allow(clippy::too_many_arguments)]
pub async fn seed_sample_course(
    db_pool: &Pool<Postgres>,
    org: Uuid,
    course_id: Uuid,
    course_name: &str,
    course_slug: &str,
    admin: Uuid,
    student: Uuid,
    users: &[Uuid],
    jwt_key: Arc<JwtKey>,
) -> Result<Uuid> {
    let spec_fetcher = models_requests::make_spec_fetcher(Arc::clone(&jwt_key));
    info!("inserting sample course {}", course_name);
    let mut conn = db_pool.acquire().await?;
    let new_course = NewCourse {
        name: course_name.to_string(),
        organization_id: org,
        slug: course_slug.to_string(),
        language_code: "en-US".to_string(),
        teacher_in_charge_name: "admin".to_string(),
        teacher_in_charge_email: "admin@example.com".to_string(),
        description: "Sample course.".to_string(),
        is_draft: false,
        is_test_mode: false,
    };
    let (course, _front_page, default_instance, default_module) =
        library::content_management::create_new_course(
            &mut conn,
            PKeyPolicy::Fixed(CreateNewCourseFixedIds {
                course_id,
                default_course_instance_id: Uuid::new_v5(
                    &course_id,
                    b"7344f1c8-b7ce-4c7d-ade2-5f39997bd454",
                ),
            }),
            new_course,
            admin,
            &spec_fetcher,
            models_requests::fetch_service_info,
        )
        .await?;
    course_instances::insert(
        &mut conn,
        PKeyPolicy::Fixed(Uuid::new_v5(
            &course_id,
            b"67f077b4-0562-47ae-a2b9-db2f08f168a9",
        )),
        NewCourseInstance {
            course_id: course.id,
            name: Some("Non-default instance"),
            description: Some("This is a non-default instance"),
            support_email: Some("contact@example.com"),
            teacher_in_charge_name: "admin",
            teacher_in_charge_email: "admin@example.com",
            opening_time: None,
            closing_time: None,
        },
    )
    .await?;

    // chapters and pages

    let new_chapter = NewChapter {
        chapter_number: 1,
        course_id: course.id,
        front_page_id: None,
        name: "The Basics".to_string(),
        color: None,
        opens_at: None,
        deadline: Some(Utc.with_ymd_and_hms(2025, 1, 1, 23, 59, 59).unwrap()),
        course_module_id: Some(default_module.id),
    };
    let (chapter_1, _front_page_1) = library::content_management::create_new_chapter(
        &mut conn,
        PKeyPolicy::Fixed((
            Uuid::new_v5(&course_id, b"bfc557e1-0f8e-4f10-8e21-d7d8ffe50a3a"),
            Uuid::new_v5(&course_id, b"b1e392db-482a-494e-9cbb-c87bbc70e340"),
        )),
        &new_chapter,
        admin,
        &spec_fetcher,
        models_requests::fetch_service_info,
    )
    .await?;
    chapters::set_opens_at(&mut conn, chapter_1.id, Utc::now()).await?;
    let new_chapter = NewChapter {
        chapter_number: 2,
        course_id: course.id,
        front_page_id: None,
        name: "The intermediaries".to_string(),
        color: None,
        opens_at: None,
        deadline: None,
        course_module_id: Some(default_module.id),
    };
    let (chapter_2, _front_page_2) = library::content_management::create_new_chapter(
        &mut conn,
        PKeyPolicy::Fixed((
            Uuid::new_v5(&course_id, b"8d699f05-4318-47f7-b020-b2084128f746"),
            Uuid::new_v5(&course_id, b"9734cb59-4c3c-467d-91e8-f4281baccfe5"),
        )),
        &new_chapter,
        admin,
        &spec_fetcher,
        models_requests::fetch_service_info,
    )
    .await?;
    chapters::set_opens_at(
        &mut conn,
        chapter_2.id,
        Utc::now() + chrono::Duration::minutes(10),
    )
    .await?;
    let new_chapter = NewChapter {
        chapter_number: 3,
        course_id: course.id,
        front_page_id: None,
        name: "Advanced studies".to_string(),
        color: None,
        opens_at: None,
        deadline: None,
        course_module_id: Some(default_module.id),
    };
    let (chapter_3, _front_page_3) = library::content_management::create_new_chapter(
        &mut conn,
        PKeyPolicy::Fixed((
            Uuid::new_v5(&course_id, b"791eada6-5299-41e9-b39c-da4f3c564814"),
            Uuid::new_v5(&course_id, b"22cb6a59-9d9d-4a0b-945b-11a6f2f8d6ef"),
        )),
        &new_chapter,
        admin,
        &spec_fetcher,
        models_requests::fetch_service_info,
    )
    .await?;
    chapters::set_opens_at(
        &mut conn,
        chapter_3.id,
        Utc::now() + chrono::Duration::minutes(20),
    )
    .await?;
    let new_chapter = NewChapter {
        chapter_number: 4,
        course_id: course.id,
        front_page_id: None,
        name: "Forbidden magicks".to_string(),
        color: None,
        opens_at: None,
        deadline: None,
        course_module_id: Some(default_module.id),
    };
    let (chapter_4, _front_page_4) = library::content_management::create_new_chapter(
        &mut conn,
        PKeyPolicy::Fixed((
            Uuid::new_v5(&course_id, b"07f8ceea-d41e-4dcb-9e4b-f600d3894e7f"),
            Uuid::new_v5(&course_id, b"cd7a35b7-8f16-4e86-bef2-b730943ec15b"),
        )),
        &new_chapter,
        admin,
        &spec_fetcher,
        models_requests::fetch_service_info,
    )
    .await?;
    chapters::set_opens_at(
        &mut conn,
        chapter_4.id,
        Utc::now() + (chrono::Duration::days(365) * 100),
    )
    .await?;

    tracing::info!("inserting modules");
    let second_module = course_modules::insert(
        &mut conn,
        PKeyPolicy::Generate,
        &NewCourseModule::new(course.id, Some("Another module".to_string()), 1),
    )
    .await?;
    let new_chapter = NewChapter {
        chapter_number: 5,
        course_id: course.id,
        front_page_id: None,
        name: "Another chapter".to_string(),
        color: None,
        opens_at: None,
        deadline: None,
        course_module_id: Some(second_module.id),
    };
    let (_m1_chapter_1, _m1c1_front_page) = library::content_management::create_new_chapter(
        &mut conn,
        PKeyPolicy::Fixed((
            Uuid::new_v5(&course_id, b"c9003113-b69b-4ee7-8b13-e16397f1a3ea"),
            Uuid::new_v5(&course_id, b"f95aa0bc-93d0-4d83-acde-64682f5e8f66"),
        )),
        &new_chapter,
        admin,
        &spec_fetcher,
        models_requests::fetch_service_info,
    )
    .await?;
    let new_chapter = NewChapter {
        chapter_number: 6,
        course_id: course.id,
        front_page_id: None,
        name: "Another another chapter".to_string(),
        color: None,
        opens_at: None,
        deadline: None,
        course_module_id: Some(second_module.id),
    };
    let (_m1_chapter_2, _m1c2_front_page) = library::content_management::create_new_chapter(
        &mut conn,
        PKeyPolicy::Fixed((
            Uuid::new_v5(&course_id, b"4989533a-7888-424c-963c-d8007d820fca"),
            Uuid::new_v5(&course_id, b"e68b9d5b-fa2e-4a94-a1da-5d69f29dcb63"),
        )),
        &new_chapter,
        admin,
        &spec_fetcher,
        models_requests::fetch_service_info,
    )
    .await?;
    let module = course_modules::insert(
        &mut conn,
        PKeyPolicy::Generate,
        &NewCourseModule::new(course.id, Some("Bonus module".to_string()), 2),
    )
    .await?;
    let new_chapter = NewChapter {
        chapter_number: 7,
        course_id: course.id,
        front_page_id: None,
        name: "Bonus chapter".to_string(),
        color: None,
        opens_at: None,
        deadline: None,
        course_module_id: Some(module.id),
    };
    let (_m2_chapter_1, _m2c1_front_page) = library::content_management::create_new_chapter(
        &mut conn,
        PKeyPolicy::Fixed((
            Uuid::new_v5(&course_id, b"26b52b2f-8b02-4be8-b341-6e956ff3ca86"),
            Uuid::new_v5(&course_id, b"0512fb7c-cb3f-4111-b663-e2fa7714939f"),
        )),
        &new_chapter,
        admin,
        &spec_fetcher,
        models_requests::fetch_service_info,
    )
    .await?;
    let new_chapter = NewChapter {
        chapter_number: 8,
        course_id: course.id,
        front_page_id: None,
        name: "Another bonus chapter".to_string(),
        color: None,
        opens_at: None,
        deadline: None,
        course_module_id: Some(module.id),
    };
    let (_m2_chapter_2, _m2c2_front_page) = library::content_management::create_new_chapter(
        &mut conn,
        PKeyPolicy::Fixed((
            Uuid::new_v5(&course_id, b"4e48b13a-9740-4d4f-9f60-8176649901b9"),
            Uuid::new_v5(&course_id, b"bc6569fe-52d2-4590-aa3a-8ae80e961db8"),
        )),
        &new_chapter,
        admin,
        &spec_fetcher,
        models_requests::fetch_service_info,
    )
    .await?;

    let welcome_page = NewCoursePage::new(
        course.id,
        1,
        "/welcome",
        "Welcome to Introduction to Everything",
    );
    let (_page, _) = pages::insert_course_page(&mut conn, &welcome_page, admin).await?;
    let hidden_page = welcome_page
        .followed_by("/hidden", "Hidden Page")
        .set_hidden(true)
        .set_content(vec![GutenbergBlock::paragraph(
            "You found the secret of the project 331!",
        )]);
    let (_page, _) = pages::insert_course_page(&mut conn, &hidden_page, admin).await?;

    info!("sample exercises");
    let block_id_1 = Uuid::new_v5(&course_id, b"af3b467a-f5db-42ad-9b21-f42ca316b3c6");
    let block_id_2 = Uuid::new_v5(&course_id, b"465f1f95-22a1-43e1-b4a3-7d18e525dc12");
    let block_id_3 = Uuid::new_v5(&course_id, b"46aad5a8-71bd-49cd-8d86-3368ee8bb7ac");
    let block_id_4 = Uuid::new_v5(&course_id, b"09b327a8-8e65-437e-9678-554fc4d98dd4");
    let block_id_5 = Uuid::new_v5(&course_id, b"834648cc-72d9-42d1-bed7-cc6a2e186ae6");
    let block_id_6 = Uuid::new_v5(&course_id, b"223a4718-5287-49ff-853e-a67f4612c629");
    let exercise_1_id = Uuid::new_v5(&course_id, b"cfb950a7-db4e-49e4-8ec4-d7a32b691b08");
    let exercise_1_slide_1_id = Uuid::new_v5(&course_id, b"182c4128-c4e4-40c9-bc5a-1265bfd3654c");
    let exercise_1_slide_1_task_1_id =
        Uuid::new_v5(&course_id, b"f73dab3b-3549-422d-8377-ece1972e5576");
    let exercise_1_slide_1_task_1_spec_1_id =
        Uuid::new_v5(&course_id, b"5f6b7850-5034-4cef-9dcf-e3fd4831067f");
    let exercise_1_slide_1_task_1_spec_2_id =
        Uuid::new_v5(&course_id, b"c713bbfc-86bf-4877-bd39-53afaf4444b5");
    let exercise_1_slide_1_task_1_spec_3_id =
        Uuid::new_v5(&course_id, b"4027d508-4fad-422e-bb7f-15c613a02cc6");

    let (exercise_block_1, exercise_1, slide_1, task_1) = create_best_exercise(
        exercise_1_id,
        exercise_1_slide_1_id,
        exercise_1_slide_1_task_1_id,
        block_id_2,
        block_id_3,
        exercise_1_slide_1_task_1_spec_1_id,
        exercise_1_slide_1_task_1_spec_2_id,
        exercise_1_slide_1_task_1_spec_3_id,
    );
    let page_c1_1 = create_page(
        &mut conn,
        course.id,
        admin,
        Some(chapter_1.id),
        CmsPageUpdate {
            url_path: "/chapter-1/page-1".to_string(),
            title: "Page One".to_string(),
            chapter_id: Some(chapter_1.id),
            exercises: vec![exercise_1],
            exercise_slides: vec![slide_1],
            exercise_tasks: vec![task_1],
            content: serde_json::json!([
                paragraph("Everything is a big topic.", block_id_1),
                exercise_block_1,
                paragraph("So big, that we need many paragraphs.", block_id_4),
                paragraph("Like this.", block_id_5),
                paragraph(&"At vero eos et accusamus et iusto odio dignissimos ducimus qui blanditiis praesentium voluptatum deleniti atque corrupti quos dolores et quas molestias excepturi sint occaecati cupiditate non provident, similique sunt in culpa qui officia deserunt mollitia animi, id est laborum et dolorum fuga. Et harum quidem rerum facilis est et expedita distinctio. Nam libero tempore, cum soluta nobis est eligendi optio cumque nihil impedit quo minus id quod maxime placeat facere possimus, omnis voluptas assumenda est, omnis dolor repellendus. Temporibus autem quibusdam et aut officiis debitis aut rerum necessitatibus saepe eveniet ut et voluptates repudiandae sint et molestiae non recusandae. Itaque earum rerum hic tenetur a sapiente delectus, ut aut reiciendis voluptatibus maiores alias consequatur aut perferendis doloribus asperiores repellat. ".repeat(4), block_id_6),
            ]),
        },
        Arc::clone(&jwt_key),
    )
    .await?;

    let exercise_2_id = Uuid::new_v5(&course_id, b"36e7f0c2-e663-4382-a503-081866cfe7d0");
    let exercise_2_slide_1_id = Uuid::new_v5(&course_id, b"0d85864d-a20d-4d65-9ace-9b4d377f38e8");
    let exercise_2_slide_1_task_1_id =
        Uuid::new_v5(&course_id, b"e7fca192-2161-4ab8-8533-8c41dbaa2d69");
    let exercise_2_slide_1_task_1_spec_1_id =
        Uuid::new_v5(&course_id, b"5898293f-2d41-43b1-9e44-92d487196ade");
    let exercise_2_slide_1_task_1_spec_2_id =
        Uuid::new_v5(&course_id, b"93d27d79-f9a1-44ab-839f-484accc67e32");
    let exercise_2_slide_1_task_1_spec_3_id =
        Uuid::new_v5(&course_id, b"81ec2df2-a5fd-4d7d-b85f-0c304e8d2030");
    let exercise_3_id = Uuid::new_v5(&course_id, b"64d273eb-628f-4d43-a11a-e69ebe244942");
    let exercise_3_slide_1_id = Uuid::new_v5(&course_id, b"5441c7c0-60f1-4058-8223-7090c9cac7cb");
    let exercise_3_slide_1_task_1_id =
        Uuid::new_v5(&course_id, b"114caac5-006a-4afb-9806-785154263c11");
    let exercise_3_slide_1_task_1_spec_1_id =
        Uuid::new_v5(&course_id, b"28ea3062-bd6a-45f5-9844-03174e00a0a8");
    let exercise_3_slide_1_task_1_spec_2_id =
        Uuid::new_v5(&course_id, b"1982f566-2d6a-485d-acb0-65d8b8864c7e");
    let exercise_3_slide_1_task_1_spec_3_id =
        Uuid::new_v5(&course_id, b"01ec5329-2cf6-4d0f-92b2-d388360fb402");
    let exercise_4_id = Uuid::new_v5(&course_id, b"029688ec-c7be-4cb3-8928-85cfd6551083");
    let exercise_4_slide_1_id = Uuid::new_v5(&course_id, b"ab8a314b-ac03-497b-8ade-3d8512ed00c9");
    let exercise_4_slide_1_task_1_id =
        Uuid::new_v5(&course_id, b"382fffce-f177-47d0-a5c0-cc8906d34c49");
    let exercise_4_slide_1_task_1_spec_1_id =
        Uuid::new_v5(&course_id, b"4bae54a3-d67c-428b-8996-290f70ae08fa");
    let exercise_4_slide_1_task_1_spec_2_id =
        Uuid::new_v5(&course_id, b"c3f257c0-bdc2-4d81-99ff-a71c76fe670a");
    let exercise_4_slide_1_task_1_spec_3_id =
        Uuid::new_v5(&course_id, b"fca5a8ba-50e0-4375-8d4b-9d02762d908c");
    let (exercise_block_2, exercise_2, slide_2, task_2) = create_best_exercise(
        exercise_2_id,
        exercise_2_slide_1_id,
        exercise_2_slide_1_task_1_id,
        Uuid::new_v5(&course_id, b"2dbb4649-bcac-47ab-a817-ca17dcd70378"),
        Uuid::new_v5(&course_id, b"c0986981-c8ae-4c0b-b558-1163a16760ec"),
        exercise_2_slide_1_task_1_spec_1_id,
        exercise_2_slide_1_task_1_spec_2_id,
        exercise_2_slide_1_task_1_spec_3_id,
    );
    let (exercise_block_3, exercise_3, slide_3, task_3) = create_best_exercise(
        exercise_3_id,
        exercise_3_slide_1_id,
        exercise_3_slide_1_task_1_id,
        Uuid::new_v5(&course_id, b"fb26489d-ca49-4f76-a1c2-f759ed3146c0"),
        Uuid::new_v5(&course_id, b"c0986981-c8ae-4c0b-b558-1163a16760ec"),
        exercise_3_slide_1_task_1_spec_1_id,
        exercise_3_slide_1_task_1_spec_2_id,
        exercise_3_slide_1_task_1_spec_3_id,
    );
    let (exercise_block_4, exercise_4, slide_4, task_4_1) = create_best_exercise(
        exercise_4_id,
        exercise_4_slide_1_id,
        exercise_4_slide_1_task_1_id,
        Uuid::new_v5(&course_id, b"334593ad-8ba5-4589-b1f7-b159e754bdc5"),
        Uuid::new_v5(&course_id, b"389e80bd-5f91-40c7-94ff-7dda1eeb96fb"),
        exercise_4_slide_1_task_1_spec_1_id,
        exercise_4_slide_1_task_1_spec_2_id,
        exercise_4_slide_1_task_1_spec_3_id,
    );

    let page2_id = create_page(
        &mut conn,
        course.id,
        admin,
        Some(chapter_1.id),
        CmsPageUpdate {
            url_path: "/chapter-1/page-2".to_string(),
            title: "Page 2".to_string(),
            chapter_id: Some(chapter_1.id),
            exercises: vec![exercise_2, exercise_3, exercise_4],
            exercise_slides: vec![slide_2, slide_3, slide_4],
            exercise_tasks: vec![task_2, task_3, task_4_1],
            content: serde_json::json!([
                paragraph(
                    "First chapters second page.",
                    Uuid::new_v5(&course_id, b"9faf5a2d-f60d-4a70-af3d-0e7e3d6fe273"),
                ),
                exercise_block_2,
                exercise_block_3,
                exercise_block_4,
            ]),
        },
        Arc::clone(&jwt_key),
    )
    .await?;

    url_redirections::insert(
        &mut conn,
        PKeyPolicy::Generate,
        page2_id,
        "/old-url",
        course.id,
    )
    .await?;

    let (
        quizzes_exercise_block_1,
        quizzes_exercise_1,
        quizzes_exercise_slide_1,
        quizzes_exercise_task_1,
    ) = quizzes_exercise(
        "Best quizzes exercise".to_string(),
        Uuid::new_v5(&course_id, b"a6ee42d0-2200-43b7-9981-620753a9b5c0"),
        Uuid::new_v5(&course_id, b"8d01d9b3-87d1-4e24-bee2-2726d3853ec6"),
        Uuid::new_v5(&course_id, b"00dd984d-8651-404e-80b8-30fae9cf32ed"),
        Uuid::new_v5(&course_id, b"a66c2552-8123-4287-bd8b-b49a29204870"),
        Uuid::new_v5(&course_id, b"f6f63ff0-c119-4141-922b-bc04cbfa0a31"),
        true,
        serde_json::json!({
            "id": "a2704a2b-fe3d-4945-a007-5593e4b81195",
            "body": "very hard",
            "open": "2021-12-17T07:15:33.479Z",
            "part": 0,
            "items": [{
                "id": "c449acf6-094e-494e-aef4-f5dfa51729ae",
                "body": "",
                "type": "essay",
                "multi": false,
                "multipleChoiceMultipleOptionsGradingPolicy": "default",
                "order": 0,
                "title": "write an essay",
                "quizId": "a2704a2b-fe3d-4945-a007-5593e4b81195",
                "options": [],
                "maxValue": null,
                "maxWords": 500,
                "minValue": null,
                "minWords": 10,
                "createdAt": "2021-12-17T07:16:23.202Z",
                "direction": "row",
                "updatedAt": "2021-12-17T07:16:23.202Z",
                "formatRegex": null,
                "validityRegex": null,
                "failureMessage": null,
                "successMessage": null,
                "allAnswersCorrect": false,
                "sharedOptionFeedbackMessage": null,
                "usesSharedOptionFeedbackMessage": false
            }],
            "title": "Pretty good exercise",
            "tries": 1,
            "points": 2,
            "section": 0,
            "courseId": "1dbd4a71-5f4c-49c9-b8a0-2e65fb8c4e0c",
            "deadline": "2025-12-17T07:15:33.479Z",
            "createdAt": "2021-12-17T07:15:33.479Z",
            "updatedAt": "2021-12-17T07:15:33.479Z",
            "autoReject": false,
            "autoConfirm": true,
            "triesLimited": true,
            "submitMessage": "This is an extra submit message from the teacher.",
            "excludedFromScore": true,
            "grantPointsPolicy": "grant_whenever_possible",
            "awardPointsEvenIfWrong": false}),
        Some(Utc.with_ymd_and_hms(2125, 1, 1, 23, 59, 59).unwrap()),
    );

    let (
        quizzes_exercise_block_2,
        quizzes_exercise_2,
        quizzes_exercise_slide_2,
        quizzes_exercise_task_2,
    ) = quizzes_exercise(
        "Best quizzes exercise".to_string(),
        Uuid::new_v5(&course_id, b"949b548f-a87f-4dc6-aafc-9f1e1abe34a7"),
        Uuid::new_v5(&course_id, b"39c36d3f-017e-4c36-a97e-908e25b3678b"),
        Uuid::new_v5(&course_id, b"8ae8971c-95dd-4d8c-b38f-152ad89c6b20"),
        Uuid::new_v5(&course_id, b"d05b1d9b-f270-4e5e-baeb-a904ea29dc90"),
        Uuid::new_v5(&course_id, b"1057f91c-9dac-4364-9d6a-fa416abc540b"),
        false,
        serde_json::json!({
            "id": "1e2bb795-1736-4b37-ae44-b16ca59b4e4f",
            "body": "very hard",
            "open": "2021-12-17T07:15:33.479Z",
            "part": 0,
            "items": [{
                "id": "d81a81f2-5e44-48c5-ab6d-f724af8a23f2",
                "body": "",
                "type": "open",
                "multi": false,
                "multipleChoiceMultipleOptionsGradingPolicy": "default",
                "order": 0,
                "title": "When you started studying at the uni? Give the date in yyyy-mm-dd format.",
                "quizId": "690c69e2-9275-4cfa-aba4-63ac917e59f6",
                "options": [],
                "maxValue": null,
                "maxWords": null,
                "minValue": null,
                "minWords": null,
                "createdAt": "2021-12-17T07:16:23.202Z",
                "direction": "row",
                "updatedAt": "2021-12-17T07:16:23.202Z",
                "formatRegex": null,
                "validityRegex": r#"^\d{4}\-(0[1-9]|1[012])\-(0[1-9]|[12][0-9]|3[01])$"#.to_string(),
                "failureMessage": "Oh no! Your answer is not in yyyy-mm-dd format :(".to_string(),
                "successMessage": "Gongrats! your answer is in yyyy-mm-dd format!".to_string(),
                "allAnswersCorrect": false,
                "sharedOptionFeedbackMessage": null,
                "usesSharedOptionFeedbackMessage": false
            }],
            "title": "Pretty good exercise",
            "tries": 1,
            "points": 2,
            "section": 0,
            "courseId": "39c7879a-e61f-474a-8f18-7fc476ccc3a0",
            "deadline": "2021-12-17T07:15:33.479Z",
            "createdAt": "2021-12-17T07:15:33.479Z",
            "updatedAt": "2021-12-17T07:15:33.479Z",
            "autoReject": false,
            "autoConfirm": true,
            "randomizeOptions": false,
            "triesLimited": true,
            "submitMessage": "This is an extra submit message from the teacher.",
            "excludedFromScore": true,
            "grantPointsPolicy": "grant_whenever_possible",
            "awardPointsEvenIfWrong": false}),
        Some(Utc.with_ymd_and_hms(2125, 1, 1, 23, 59, 59).unwrap()),
    );

    let (
        quizzes_exercise_block_3,
        quizzes_exercise_3,
        quizzes_exercise_slide_3,
        quizzes_exercise_task_3,
    ) = quizzes_exercise(
        "Best quizzes exercise".to_string(),
        Uuid::new_v5(&course_id, b"9bcf634d-584c-4fef-892c-3c0e97dab1d5"),
        Uuid::new_v5(&course_id, b"984457f6-bc9b-4604-b54c-80fb4adfab76"),
        Uuid::new_v5(&course_id, b"e4230b3a-1db8-49c4-9554-1f96f7f3d015"),
        Uuid::new_v5(&course_id, b"52939561-af36-4ab6-bffa-be97e94d3314"),
        Uuid::new_v5(&course_id, b"8845b17e-2320-4384-97f8-24e42457cb5e"),
        false,
        serde_json::json!({
            "id": "f1f0520e-3037-409c-b52d-163ad0bc5c59",
            "body": "very hard",
            "open": "2021-12-17T07:15:33.479Z",
            "part": 0,
            "items": [{
                "id": "f8cff916-da28-40ab-9e8b-f523e661ddb6",
                "body": "",
                "type": "multiple-choice-dropdown",
                "multi": false,
                "multipleChoiceMultipleOptionsGradingPolicy": "default",
                "order": 0,
                "title": "Choose the right answer from given options.",
                "quizId": "f1f0520e-3037-409c-b52d-163ad0bc5c59",
                "options": [{
                    "id": "86a2d838-04aa-4b1c-8115-2c15ed19e7b3",
                    "body": null,
                    "order": 1,
                    "title": "The right answer",
                    "quizItemId": "f8cff916-da28-40ab-9e8b-f523e661ddb6",
                    "correct":true,
                    "messageAfterSubmissionWhenSelected": "You chose wisely...",
                    "additionalCorrectnessExplanationOnModelSolution": null,
                },
                {
                    "id": "fef8cd36-04ab-48f2-861c-51769ccad52f",
                    "body": null,
                    "order": 2,
                    "title": "The Wright answer",
                    "quizItemId": "f8cff916-da28-40ab-9e8b-f523e661ddb6",
                    "correct":false,
                    "messageAfterSubmissionWhenSelected": "You chose poorly...",
                    "additionalCorrectnessExplanationOnModelSolution": null,
                }],
                "maxValue": null,
                "maxWords": null,
                "minValue": null,
                "minWords": null,
                "createdAt": "2021-12-17T07:16:23.202Z",
                "direction": "row",
                "updatedAt": "2021-12-17T07:16:23.202Z",
                "formatRegex": null,
                "validityRegex": null,
                "messageAfterSubmissionWhenSelected": null,
                "additionalCorrectnessExplanationOnModelSolution": null,
                "allAnswersCorrect": false,
                "sharedOptionFeedbackMessage": null,
                "usesSharedOptionFeedbackMessage": false
            }],
            "title": "Pretty good exercise",
            "tries": 1,
            "points": 2,
            "section": 0,
            "courseId": "39c7879a-e61f-474a-8f18-7fc476ccc3a0",
            "deadline": "2021-12-17T07:15:33.479Z",
            "createdAt": "2021-12-17T07:15:33.479Z",
            "updatedAt": "2021-12-17T07:15:33.479Z",
            "autoReject": false,
            "autoConfirm": true,
            "randomizeOptions": false,
            "triesLimited": true,
            "submitMessage": "This is an extra submit message from the teacher.",
            "excludedFromScore": true,
            "grantPointsPolicy": "grant_whenever_possible",
            "awardPointsEvenIfWrong": false}),
        Some(Utc.with_ymd_and_hms(2125, 1, 1, 23, 59, 59).unwrap()),
    );

    let (
        quizzes_exercise_block_4,
        quizzes_exercise_4,
        quizzes_exercise_slide_4,
        quizzes_exercise_task_4,
    ) = quizzes_exercise(
        "Best quizzes exercise".to_string(),
        Uuid::new_v5(&course_id, b"854a4e05-6575-4d27-8feb-6ee01f662d8a"),
        Uuid::new_v5(&course_id, b"6a8e65be-f5cd-4c87-b4f9-9522cb37bbcb"),
        Uuid::new_v5(&course_id, b"b5e1e7e87-0678-4296-acf7-a8ac926ff94b"),
        Uuid::new_v5(&course_id, b"50e26d7f-f11f-4a8a-990d-fb17c3371d1d"),
        Uuid::new_v5(&course_id, b"7ca39a36-2dcd-4521-bbf6-bfc5849874e3"),
        false,
        serde_json::json!({
            "id": "1e2bb795-1736-4b37-ae44-b16ca59b4e4f",
            "body": "very hard",
            "open": "2021-12-17T07:15:33.479Z",
            "part": 0,
            "items": [{
                "id": "d30bec57-4011-4ac4-b676-79fe766d6424",
                "body": null,
                "type": "clickable-multiple-choice",
                "multi": true,
                "multipleChoiceMultipleOptionsGradingPolicy": "default",
                "order": 0,
                "title": "Pick all the programming languages from below",
                "quizId": "1e2bb795-1736-4b37-ae44-b16ca59b4e4f",
                "options": [
                    {
                        "id": "55a63887-a896-425c-91ae-2f85032c3d58",
                        "body": "Java",
                        "order": 1,
                        "title": null,
                        "quizItemId": "f8cff916-da28-40ab-9e8b-f523e661ddb6",
                        "correct":true,
                        "messageAfterSubmissionWhenSelected": "Java is a programming language",
                        "additionalCorrectnessExplanationOnModelSolution": null
                    },
                    {
                        "id": "534bf512-014e-47b6-a67b-8bea6dc65177",
                        "body": "Erlang",
                        "order": 2,
                        "title": null,
                        "quizItemId": "f8cff916-da28-40ab-9e8b-f523e661ddb6",
                        "correct":true,
                        "messageAfterSubmissionWhenSelected": "Erlang is a programming language",
                        "additionalCorrectnessExplanationOnModelSolution": null,
                    },
                    {
                        "id": "ea4e0bb4-f84e-4048-be2f-f819a391396f",
                        "body": "Jupiter",
                        "order": 3,
                        "title": null,
                        "quizItemId": "f8cff916-da28-40ab-9e8b-f523e661ddb6",
                        "correct":false,
                        "messageAfterSubmissionWhenSelected": "Jupiter is not a programming language",
                        "additionalCorrectnessExplanationOnModelSolution": null
                    },
                    {
                        "id": "b851f1b3-ae90-46bd-8f10-fd6d968695ef",
                        "body": "Rust",
                        "order": 4,
                        "title": null,
                        "quizItemId": "f8cff916-da28-40ab-9e8b-f523e661ddb6",
                        "correct":true,
                        "messageAfterSubmissionWhenSelected": "Rust is a programming language",
                        "additionalCorrectnessExplanationOnModelSolution": null
                    },
                    {
                        "id": "8107ae39-96aa-4f54-aa78-1a33362a19c1",
                        "body": "AC",
                        "order": 5,
                        "title": null,
                        "quizItemId": "f8cff916-da28-40ab-9e8b-f523e661ddb6",
                        "correct":false,
                        "messageAfterSubmissionWhenSelected": "AC is not a programming language",
                        "additionalCorrectnessExplanationOnModelSolution": null
                    },
                ],
                "allAnswersCorrect": false,
                "sharedOptionFeedbackMessage": null,
                "usesSharedOptionFeedbackMessage": false
            }],
            "title": "Pretty good exercise",
            "tries": 1,
            "points": 2,
            "section": 0,
            "courseId": "39c7879a-e61f-474a-8f18-7fc476ccc3a0",
            "deadline": "2021-12-17T07:15:33.479Z",
            "createdAt": "2021-12-17T07:15:33.479Z",
            "updatedAt": "2021-12-17T07:15:33.479Z",
            "autoReject": false,
            "autoConfirm": true,
            "randomizeOptions": false,
            "triesLimited": true,
            "submitMessage": "This is an extra submit message from the teacher.",
            "excludedFromScore": true,
            "grantPointsPolicy": "grant_whenever_possible",
            "awardPointsEvenIfWrong": false}),
        Some(Utc.with_ymd_and_hms(2125, 1, 1, 23, 59, 59).unwrap()),
    );

    let (
        quizzes_exercise_block_5,
        quizzes_exercise_5,
        quizzes_exercise_slide_5,
        quizzes_exercise_task_5,
    ) = quizzes_exercise(
        "Best quizzes exercise".to_string(),
        Uuid::new_v5(&course.id, b"981623c8-baa3-4d14-bb8a-963e167da9ca"),
        Uuid::new_v5(&course.id, b"b1a6d7e4-00b2-43fb-bf39-863f4ef49d09"),
        Uuid::new_v5(&course.id, b"1a2f2c9f-9552-440e-8dd3-1e3703bd0fab"),
        Uuid::new_v5(&course.id, b"6b568812-f752-4d9f-a60a-48257822d21e"),
        Uuid::new_v5(&course.id, b"b2f7d8d5-f3c0-4cac-8eb7-89a7b88c2236"),
        false,
        serde_json::json!({
          "autoConfirm": true,
          "randomizeOptions": false,
          "autoReject": false,
          "awardPointsEvenIfWrong": false,
          "body": "",
          "courseId": "29b09b7e-337f-4074-b14b-6109427a52f6",
          "createdAt": "2022-05-04T09:03:06.271Z",
          "deadline": "2022-05-04T09:03:06.271Z",
          "excludedFromScore": true,
          "grantPointsPolicy": "grant_whenever_possible",
          "id": "72c3bb44-1695-4ea0-af3e-f2280c726551",
          "items": [
            {
              "allAnswersCorrect": false,
              "body": "",
              "createdAt": "2022-05-04T09:03:09.167Z",
              "direction": "column",
              "failureMessage": null,
              "formatRegex": null,
              "id": "105270c8-e94a-40ec-a159-8fe38f116bb4",
              "maxValue": null,
              "maxWords": null,
              "minValue": null,
              "minWords": null,
              "multi": false,
              "optionCells": null,
              "options": [],
              "order": 0,
              "quizId": "72c3bb44-1695-4ea0-af3e-f2280c726551",
              "sharedOptionFeedbackMessage": null,
              "successMessage": null,
              "timelineItems": [
                {
                  "correctEventId": "59e30264-fb11-4e44-a91e-1c5cf80fd977",
                  "correctEventName": "Finland joins  the European Union",
                  "id": "c40fc487-9cb9-4007-80d3-8ffd7a8dc799",
                  "year": "1995"
                },
                {
                  "correctEventId": "0ee17a8e-6d51-4620-b355-90815462543f",
                  "correctEventName": "Finland switches their currency to Euro",
                  "id": "d63fd98e-b73c-47cf-a634-9046249c78e4",
                  "year": "2002"
                },
                {
                  "correctEventId": "0a59d2d3-6cf6-4b91-b1bd-873eefde78ac",
                  "correctEventName": "Finland joins the Economic and Monetary Union of the European Union",
                  "id": "50d7641c-382e-4805-95d8-e873c462bc48",
                  "year": "1998"
                }
              ],
              "title": "",
              "type": "timeline",
              "updatedAt": "2022-05-04T09:03:09.167Z",
              "usesSharedOptionFeedbackMessage": false,
              "validityRegex": null
            }
          ],
          "open": "2022-05-04T09:03:06.271Z",
          "part": 0,
          "points": 0,
          "section": 0,
          "submitMessage": "",
          "title": "",
          "tries": 1,
          "triesLimited": true,
          "updatedAt": "2022-05-04T09:03:06.271Z"
        }),
        Some(Utc.with_ymd_and_hms(2125, 1, 1, 23, 59, 59).unwrap()),
    );

    let (
        quizzes_exercise_block_6,
        quizzes_exercise_6,
        quizzes_exercise_slide_6,
        quizzes_exercise_task_6,
    ) = quizzes_exercise(
        "Multiple choice with feedback".to_string(),
        Uuid::new_v5(&course.id, b"f7fa3a08-e287-44de-aea8-32133af89d31"),
        Uuid::new_v5(&course.id, b"31820133-579a-4d9f-8b0c-2120f76d1390"),
        Uuid::new_v5(&course.id, b"55f929c7-30ab-441d-a0ad-6cd115857b3b"),
        Uuid::new_v5(&course.id, b"d7a91d07-9bd9-449c-9862-fbacb0b402b0"),
        Uuid::new_v5(&course.id, b"664ea614-4af4-4ad0-9855-eae1881568e6"),
        false,
        serde_json::from_str(include_str!(
            "../../assets/quizzes-multiple-choice-feedback.json"
        ))?,
        Some(Utc.with_ymd_and_hms(2125, 1, 1, 23, 59, 59).unwrap()),
    );

    let (
        quizzes_exercise_block_7,
        quizzes_exercise_7,
        quizzes_exercise_slide_7,
        quizzes_exercise_task_7,
    ) = quizzes_exercise(
        "Scale".to_string(),
        Uuid::new_v5(&course.id, b"212132eb-b108-4027-b312-2275cf0b7473"),
        Uuid::new_v5(&course.id, b"6172a36a-b65d-463c-81d0-7f7fce07615c"),
        Uuid::new_v5(&course.id, b"0dcfc4ca-c2f7-40b0-8654-14c6893a1fd9"),
        Uuid::new_v5(&course.id, b"b64d7bd2-a216-494e-a23c-7a975fb1a415"),
        Uuid::new_v5(&course.id, b"05fa1188-4653-4904-bf1c-a93363225841"),
        false,
        serde_json::from_str(include_str!("../../assets/scale.json"))?,
        Some(Utc.with_ymd_and_hms(2125, 1, 1, 23, 59, 59).unwrap()),
    );

    let (
        quizzes_exercise_block_8,
        quizzes_exercise_8,
        quizzes_exercise_slide_8,
        quizzes_exercise_task_8,
    ) = quizzes_exercise(
        "Vector exercise".to_string(),
        Uuid::new_v5(&course.id, b"80373dc3-ceba-45b4-a114-161d60228c0c"),
        Uuid::new_v5(&course.id, b"08f0da90-9080-4cdd-adc7-66173cd5b833"),
        Uuid::new_v5(&course.id, b"ea24c875-1a3c-403e-8272-b1249a475c89"),
        Uuid::new_v5(&course.id, b"38ed716f-5d4f-4ddd-9f5a-700ef124b934"),
        Uuid::new_v5(&course.id, b"0c271345-6934-4489-8164-2cc4dc8974bb"),
        false,
        serde_json::from_str(include_str!("../../assets/vector-exercise.json"))?,
        None,
    );

    let page_3 = create_page(
        &mut conn,
        course.id,
        admin,
        Some(chapter_1.id),
        CmsPageUpdate {
            url_path: "/chapter-1/page-3".to_string(),
            title: "Page 3".to_string(),
            chapter_id: Some(chapter_1.id),
            exercises: vec![quizzes_exercise_1],
            exercise_slides: vec![quizzes_exercise_slide_1],
            exercise_tasks: vec![quizzes_exercise_task_1],
            content: serde_json::json!([
                paragraph(
                    "First chapters essay page.",
                    Uuid::new_v5(&course_id, b"6e4ab83a-2ae8-4bd2-a6ea-0e0d1eeabe23")
                ),
                quizzes_exercise_block_1,
            ]),
        },
        Arc::clone(&jwt_key),
    )
    .await?;

    create_page(
        &mut conn,
        course.id,
        admin,
        Some(chapter_1.id),
        CmsPageUpdate {
            url_path: "/chapter-1/page-4".to_string(),
            title: "Page 4".to_string(),
            chapter_id: Some(chapter_1.id),
            exercises: vec![quizzes_exercise_2],
            exercise_slides: vec![quizzes_exercise_slide_2],
            exercise_tasks: vec![quizzes_exercise_task_2],
            content: serde_json::json!([
                paragraph(
                    "First chapters open page.",
                    Uuid::new_v5(&course_id, b"771b9c61-dbc9-4266-a980-dadc853455c9")
                ),
                quizzes_exercise_block_2
            ]),
        },
        Arc::clone(&jwt_key),
    )
    .await?;

    create_page(
        &mut conn,
        course.id,
        admin,
        Some(chapter_1.id),
        CmsPageUpdate {
            url_path: "/chapter-1/page-5".to_string(),
            title: "Page 5".to_string(),
            chapter_id: Some(chapter_1.id),
            exercises: vec![quizzes_exercise_3],
            exercise_slides: vec![quizzes_exercise_slide_3],
            exercise_tasks: vec![quizzes_exercise_task_3],
            content: serde_json::json!([
                paragraph(
                    "First chapters multiple-choice-dropdown page",
                    Uuid::new_v5(&course_id, b"7af470e7-cc4f-411e-ad5d-c137e353f7c3")
                ),
                quizzes_exercise_block_3
            ]),
        },
        Arc::clone(&jwt_key),
    )
    .await?;

    create_page(
        &mut conn,
        course.id,
        admin,
        Some(chapter_1.id),
        CmsPageUpdate {
            url_path: "/chapter-1/page-6".to_string(),
            title: "Page 6".to_string(),
            chapter_id: Some(chapter_1.id),
            exercises: vec![quizzes_exercise_4],
            exercise_slides: vec![quizzes_exercise_slide_4],
            exercise_tasks: vec![quizzes_exercise_task_4],
            content: serde_json::json!([
                paragraph(
                    "First chapters multiple-choice clickable page.",
                    Uuid::new_v5(&course_id, b"6b7775c3-b46e-41e5-a730-0a2c2f0ba148")
                ),
                quizzes_exercise_block_4
            ]),
        },
        Arc::clone(&jwt_key),
    )
    .await?;

    create_page(
        &mut conn,
        course.id,
        admin,
        Some(chapter_1.id),
        CmsPageUpdate {
            url_path: "/chapter-1/the-timeline".to_string(),
            title: "The timeline".to_string(),
            chapter_id: Some(chapter_2.id),
            exercises: vec![quizzes_exercise_5],
            exercise_slides: vec![quizzes_exercise_slide_5],
            exercise_tasks: vec![quizzes_exercise_task_5],
            content: serde_json::json!([
                paragraph(
                    "Best page",
                    Uuid::new_v5(&course.id, b"891de1ca-f3a9-506f-a268-3477ea4fdd27")
                ),
                quizzes_exercise_block_5,
            ]),
        },
        Arc::clone(&jwt_key),
    )
    .await?;

    create_page(
        &mut conn,
        course.id,
        admin,
        Some(chapter_1.id),
        CmsPageUpdate {
            url_path: "/chapter-1/scale".to_string(),
            title: "scale".to_string(),
            chapter_id: Some(chapter_1.id),
            exercises: vec![quizzes_exercise_7],
            exercise_slides: vec![quizzes_exercise_slide_7],
            exercise_tasks: vec![quizzes_exercise_task_7],
            content: serde_json::json!([
                paragraph(
                    "The page for the scale execise.",
                    Uuid::new_v5(&course_id, b"53f68082-c417-4d38-99ad-40b6a30b2da4")
                ),
                quizzes_exercise_block_7
            ]),
        },
        Arc::clone(&jwt_key),
    )
    .await?;

    create_page(
        &mut conn,
        course.id,
        admin,
        Some(chapter_1.id),
        CmsPageUpdate {
            url_path: "/chapter-1/the-multiple-choice-with-feedback".to_string(),
            title: "Multiple choice with feedback".to_string(),
            chapter_id: Some(chapter_1.id),
            exercises: vec![quizzes_exercise_6],
            exercise_slides: vec![quizzes_exercise_slide_6],
            exercise_tasks: vec![quizzes_exercise_task_6],
            content: serde_json::json!([
                paragraph(
                    "Something about rust and feedback.",
                    Uuid::new_v5(&course_id, b"cbb87878-5af1-4c01-b343-97bf668b8034")
                ),
                quizzes_exercise_block_6
            ]),
        },
        Arc::clone(&jwt_key),
    )
    .await?;

    create_page(
        &mut conn,
        course.id,
        admin,
        Some(chapter_1.id),
        CmsPageUpdate {
            url_path: "/chapter-1/vector".to_string(),
            title: "Vector".to_string(),
            chapter_id: Some(chapter_1.id),
            exercises: vec![quizzes_exercise_8],
            exercise_slides: vec![quizzes_exercise_slide_8],
            exercise_tasks: vec![quizzes_exercise_task_8],
            content: serde_json::json!([
                paragraph(
                    "This page has a vector exercise composed of three close-ended questions.",
                    Uuid::new_v5(&course_id, b"53f68082-c417-4d38-99ad-40b6a30b2da4")
                ),
                quizzes_exercise_block_8
            ]),
        },
        Arc::clone(&jwt_key),
    )
    .await?;

    let multi_exercise_1_id = Uuid::new_v5(&course_id, b"3abe8579-73f1-4cdf-aba0-3e123fcedaea");
    let multi_exercise_1_slide_1_id =
        Uuid::new_v5(&course_id, b"efc7663c-b0fd-4e21-893a-7b7891191e07");
    let multi_exercise_1_slide_1_task_1_id =
        Uuid::new_v5(&course_id, b"b8833157-aa58-4472-a09b-98406a82ef42");
    let multi_exercise_1_slide_1_task_2_id =
        Uuid::new_v5(&course_id, b"36921424-0a65-4de8-8f92-3be96d695463");
    let multi_exercise_1_slide_1_task_3_id =
        Uuid::new_v5(&course_id, b"4c4bc8e5-7108-4f0d-a3d9-54383aa57269");
    let (multi_exercise_block_1, multi_exercise_1, multi_exercise_1_slides, multi_exercise_1_tasks) =
        example_exercise_flexible(
            multi_exercise_1_id,
            "Multiple task exercise".to_string(),
            vec![(
                multi_exercise_1_slide_1_id,
                vec![
                    (
                        multi_exercise_1_slide_1_task_1_id,
                        "example-exercise".to_string(),
                        serde_json::json!([paragraph(
                            "First question.",
                            Uuid::new_v5(&course_id, b"e972a22b-67ae-4971-b437-70effd5614d4")
                        )]),
                        serde_json::json!([
                            {
                                "name": "Correct",
                                "correct": true,
                                "id": Uuid::new_v5(&course_id, b"0a046287-6b49-405d-ad9e-12f6dc5f9b1d"),
                            },
                            {
                                "name": "Incorrect",
                                "correct": false,
                                "id": Uuid::new_v5(&course_id, b"c202540e-9a3f-4ff4-9703-b9921e9eee8e"),
                            },
                        ]),
                    ),
                    (
                        multi_exercise_1_slide_1_task_2_id,
                        "example-exercise".to_string(),
                        serde_json::json!([paragraph(
                            "Second question.",
                            Uuid::new_v5(&course_id, b"e4895ced-757c-401a-8836-b734b75dff54")
                        )]),
                        serde_json::json!([
                            {
                                "name": "Correct",
                                "correct": true,
                                "id": Uuid::new_v5(&course_id, b"e0c2efa8-ac15-4a3c-94bb-7d5e72e57671"),
                            },
                            {
                                "name": "Incorrect",
                                "correct": false,
                                "id": Uuid::new_v5(&course_id, b"db5cf7d4-b5bb-43f7-931e-e329cc2e95b1"),
                            },
                        ]),
                    ),
                    (
                        multi_exercise_1_slide_1_task_3_id,
                        "example-exercise".to_string(),
                        serde_json::json!([paragraph(
                            "Third question.",
                            Uuid::new_v5(&course_id, b"13b75f4e-b02d-41fa-b5bc-79adf22d9aef")
                        )]),
                        serde_json::json!([
                            {
                                "name": "Correct",
                                "correct": true,
                                "id": Uuid::new_v5(&course_id, b"856defd2-08dd-4632-aaef-ec71cdfd3bca"),
                            },
                            {
                                "name": "Incorrect",
                                "correct": false,
                                "id": Uuid::new_v5(&course_id, b"95ffff70-7dbe-4e39-9480-2a3514e9ea1d"),
                            },
                        ]),
                    ),
                ],
            )],
            Uuid::new_v5(&course_id, b"9e70076a-9137-4d65-989c-0c0951027c53"),
        );
    create_page(
        &mut conn,
        course.id,
        admin,
        Some(chapter_1.id),
        CmsPageUpdate {
            url_path: "/chapter-1/complicated-exercise".to_string(),
            title: "Complicated exercise page".to_string(),
            chapter_id: Some(chapter_1.id),
            exercises: vec![multi_exercise_1],
            exercise_slides: multi_exercise_1_slides,
            exercise_tasks: multi_exercise_1_tasks,
            content: serde_json::json!([
                paragraph(
                    "This page has a complicated exercise.",
                    Uuid::new_v5(&course_id, b"86f1b595-ec82-43a6-954f-c1f8de3d53ac")
                ),
                multi_exercise_block_1
            ]),
        },
        Arc::clone(&jwt_key),
    )
    .await?;

    let exercise_5_id = Uuid::new_v5(&course_id, b"8bb4faf4-9a34-4df7-a166-89ade530d0f6");
    let exercise_5_slide_1_id = Uuid::new_v5(&course_id, b"b99d1041-7835-491e-a1c8-b47eee8e7ab4");
    let exercise_5_slide_1_task_1_id =
        Uuid::new_v5(&course_id, b"a6508b8a-f58e-43ac-9f02-785575e716f5");
    let exercise_5_slide_1_task_1_spec_1_id =
        Uuid::new_v5(&course_id, b"fe464d17-2365-4e65-8b33-e0ebb5a67836");
    let exercise_5_slide_1_task_1_spec_2_id =
        Uuid::new_v5(&course_id, b"6633ffc7-c76e-4049-840e-90eefa6b49e8");
    let exercise_5_slide_1_task_1_spec_3_id =
        Uuid::new_v5(&course_id, b"d77fb97d-322c-4c5f-a405-8978a8cfb0a9");
    let (exercise_block_5, exercise_5, exercise_slide_5, exercise_task_5) = create_best_exercise(
        exercise_5_id,
        exercise_5_slide_1_id,
        exercise_5_slide_1_task_1_id,
        Uuid::new_v5(&course_id, b"e869c471-b1b7-42a0-af05-dffd1d86a7bb"),
        Uuid::new_v5(&course_id, b"fe464d17-2365-4e65-8b33-e0ebb5a67836"),
        exercise_5_slide_1_task_1_spec_1_id,
        exercise_5_slide_1_task_1_spec_2_id,
        exercise_5_slide_1_task_1_spec_3_id,
    );
    create_page(
        &mut conn,
        course.id,
        admin,
        Some(chapter_2.id),
        CmsPageUpdate {
            url_path: "/chapter-2/intro".to_string(),
            title: "In the second chapter...".to_string(),
            chapter_id: Some(chapter_2.id),
            exercises: vec![exercise_5],
            exercise_slides: vec![exercise_slide_5],
            exercise_tasks: vec![exercise_task_5],
            content: serde_json::json!([exercise_block_5]),
        },
        Arc::clone(&jwt_key),
    )
    .await?;
    create_page(
        &mut conn,
        course.id,
        admin,
        None,
        CmsPageUpdate {
            url_path: "/glossary".to_string(),
            title: "Glossary".to_string(),
            chapter_id: None,
            exercises: vec![],
            exercise_slides: vec![],
            exercise_tasks: vec![],
            content: serde_json::json!([GutenbergBlock {
                name: "moocfi/glossary".to_string(),
                is_valid: true,
                client_id: Uuid::parse_str("3a388f47-4aa7-409f-af14-a0290b916225").unwrap(),
                attributes: attributes! {},
                inner_blocks: vec![]
            }]),
        },
        jwt_key,
    )
    .await?;

    // enrollments, user exercise states, submissions, grades
    info!("sample enrollments, user exercise states, submissions, grades");
    for &user_id in users {
        course_instance_enrollments::insert_enrollment_and_set_as_current(
            &mut conn,
            NewCourseInstanceEnrollment {
                course_id,
                course_instance_id: default_instance.id,
                user_id,
            },
        )
        .await?;

        submit_and_grade(
            &mut conn,
            b"8c447aeb-1791-4236-8471-204d8bc27507",
            exercise_1_id,
            exercise_1_slide_1_id,
            course.id,
            exercise_1_slide_1_task_1_id,
            user_id,
            default_instance.id,
            exercise_1_slide_1_task_1_spec_1_id.to_string(),
            100.0,
        )
        .await?;
        // this submission is for the same exercise, but no points are removed due to the update strategy
        submit_and_grade(
            &mut conn,
            b"a719fe25-5721-412d-adea-4696ccb3d883",
            exercise_1_id,
            exercise_1_slide_1_id,
            course.id,
            exercise_1_slide_1_task_1_id,
            user_id,
            default_instance.id,
            exercise_1_slide_1_task_1_spec_2_id.to_string(),
            1.0,
        )
        .await?;
        submit_and_grade(
            &mut conn,
            b"bbc16d4b-1f91-4bd0-a47f-047665a32196",
            exercise_1_id,
            exercise_1_slide_1_id,
            course.id,
            exercise_1_slide_1_task_1_id,
            user_id,
            default_instance.id,
            exercise_1_slide_1_task_1_spec_3_id.to_string(),
            0.0,
        )
        .await?;
        submit_and_grade(
            &mut conn,
            b"c60bf5e5-9b67-4f62-9df7-16d268c1b5f5",
            exercise_1_id,
            exercise_1_slide_1_id,
            course.id,
            exercise_1_slide_1_task_1_id,
            user_id,
            default_instance.id,
            exercise_1_slide_1_task_1_spec_1_id.to_string(),
            60.0,
        )
        .await?;
        submit_and_grade(
            &mut conn,
            b"e0ec1386-72aa-4eed-8b91-72bba420c23b",
            exercise_2_id,
            exercise_2_slide_1_id,
            course.id,
            exercise_2_slide_1_task_1_id,
            user_id,
            default_instance.id,
            exercise_2_slide_1_task_1_spec_1_id.to_string(),
            70.0,
        )
        .await?;
        submit_and_grade(
            &mut conn,
            b"02c9e1ad-6e4c-4473-a3e9-dbfab018a055",
            exercise_5_id,
            exercise_5_slide_1_id,
            course.id,
            exercise_5_slide_1_task_1_id,
            user_id,
            default_instance.id,
            exercise_5_slide_1_task_1_spec_1_id.to_string(),
            80.0,
        )
        .await?;
        submit_and_grade(
            &mut conn,
            b"75df4600-d337-4083-99d1-e8e3b6bf6192",
            exercise_1_id,
            exercise_1_slide_1_id,
            course.id,
            exercise_1_slide_1_task_1_id,
            user_id,
            default_instance.id,
            exercise_1_slide_1_task_1_spec_1_id.to_string(),
            90.0,
        )
        .await?;
    }

    // feedback
    info!("sample feedback");
    let new_feedback = NewFeedback {
        feedback_given: "this part was unclear to me".to_string(),
        selected_text: Some("blanditiis".to_string()),
        related_blocks: vec![FeedbackBlock {
            id: block_id_4,
            text: Some(
                "blanditiis praesentium voluptatum deleniti atque corrupti quos dolores et quas"
                    .to_string(),
            ),
            order_number: Some(0),
        }],
        page_id: page_3,
    };
    let feedback = feedback::insert(
        &mut conn,
        PKeyPolicy::Generate,
        Some(student),
        course.id,
        new_feedback,
    )
    .await?;
    feedback::mark_as_read(&mut conn, feedback, true).await?;
    let new_feedback = NewFeedback {
        feedback_given: "I dont think we need these paragraphs".to_string(),
        selected_text: Some("verything".to_string()),
        related_blocks: vec![
            FeedbackBlock {
                id: block_id_1,
                text: Some("verything is a big topic.".to_string()),
                order_number: Some(0),
            },
            FeedbackBlock {
                id: block_id_4,
                text: Some("So big, that we need many paragraphs.".to_string()),
                order_number: Some(1),
            },
            FeedbackBlock {
                id: block_id_5,
                text: Some("Like th".to_string()),
                order_number: Some(2),
            },
        ],
        page_id: page_3,
    };
    feedback::insert(
        &mut conn,
        PKeyPolicy::Generate,
        Some(student),
        course.id,
        new_feedback,
    )
    .await?;
    feedback::insert(
        &mut conn,
        PKeyPolicy::Generate,
        None,
        course.id,
        NewFeedback {
            feedback_given: "Anonymous feedback".to_string(),
            selected_text: None,
            related_blocks: vec![FeedbackBlock {
                id: block_id_1,
                text: None,
                order_number: Some(0),
            }],
            page_id: page_3,
        },
    )
    .await?;
    feedback::insert(
        &mut conn,
        PKeyPolicy::Generate,
        None,
        course.id,
        NewFeedback {
            feedback_given: "Anonymous unrelated feedback".to_string(),
            selected_text: None,
            related_blocks: vec![],
            page_id: page_3,
        },
    )
    .await?;

    // edit proposals
    info!("sample edit proposals");
    let edits = NewProposedPageEdits {
        page_id: page_c1_1,
        block_edits: vec![NewProposedBlockEdit {
            block_id: block_id_4,
            block_attribute: "content".to_string(),
            original_text: "So bg, that we need many paragraphs.".to_string(),
            changed_text: "So bg, that we need many, many paragraphs.".to_string(),
        }],
    };
    proposed_page_edits::insert(
        &mut conn,
        PKeyPolicy::Generate,
        course.id,
        Some(student),
        &edits,
    )
    .await?;
    let edits = NewProposedPageEdits {
        page_id: page_c1_1,
        block_edits: vec![
            NewProposedBlockEdit {
                block_id: block_id_1,
                block_attribute: "content".to_string(),
                original_text: "Everything is a big topic.".to_string(),
                changed_text: "Everything is a very big topic.".to_string(),
            },
            NewProposedBlockEdit {
                block_id: block_id_5,
                block_attribute: "content".to_string(),
                original_text: "Like this.".to_string(),
                changed_text: "Like this!".to_string(),
            },
        ],
    };
    proposed_page_edits::insert(
        &mut conn,
        PKeyPolicy::Generate,
        course.id,
        Some(student),
        &edits,
    )
    .await?;

    // acronyms
    glossary::insert(&mut conn, "CS", "Computer science. Computer science is an essential part of being successful in your life. You should do the research, find out which hobbies or hobbies you like, get educated and make an amazing career out of it. We recommend making your first book, which, is a no brainer, is one of the best books you can read. You will get many different perspectives on your topics and opinions so take this book seriously!",  course.id).await?;
    glossary::insert(&mut conn, "HDD", "Hard disk drive. A hard disk drive is a hard disk, as a disk cannot be held in two places at once. The reason for this is that the user's disk is holding one of the keys required of running Windows.",  course.id).await?;
    glossary::insert(&mut conn, "SSD", "Solid-state drive. A solid-state drive is a hard drive that's a few gigabytes in size, but a solid-state drive is one where data loads are big enough and fast enough that you can comfortably write to it over long distances. This is what drives do. You need to remember that a good solid-state drive has a lot of data: it stores files on disks and has a few data centers. A good solid-state drive makes for a nice little library: its metadata includes information about everything it stores, including any data it can access, but does not store anything that does not exist outside of those files. It also stores large amounts of data from one location, which can cause problems since the data might be different in different places, or in different ways, than what you would expect to see when driving big data applications. The drives that make up a solid-state drive are called drives that use a variety of storage technologies. These drive technology technologies are called \"super drives,\" and they store some of that data in a solid-state drive. Super drives are designed to be fast but very big: they aren't built to store everything, but to store many kinds of data: including data about the data they contain, and more, like the data they are supposed to hold in them. The super drives that make up a solid-state drive can have capacities of up to 50,000 hard disks. These can be used to store files if",  course.id).await?;
    glossary::insert(&mut conn, "KB", "Keyboard.", course.id).await?;

    Ok(course.id)
}

pub async fn create_glossary_course(
    db_pool: &Pool<Postgres>,
    org_id: Uuid,
    admin: Uuid,
    course_id: Uuid,
    jwt_key: Arc<JwtKey>,
) -> Result<Uuid> {
    let mut conn = db_pool.acquire().await?;

    // Create new course
    let new_course = NewCourse {
        name: "Glossary Tooltip".to_string(),
        organization_id: org_id,
        slug: "glossary-tooltip".to_string(),
        language_code: "en-US".to_string(),
        teacher_in_charge_name: "admin".to_string(),
        teacher_in_charge_email: "admin@example.com".to_string(),
        description: "Sample course.".to_string(),
        is_draft: false,
        is_test_mode: false,
    };

    let (course, _front_page, _default_instance, default_module) =
        library::content_management::create_new_course(
            &mut conn,
            PKeyPolicy::Fixed(CreateNewCourseFixedIds {
                course_id,
                default_course_instance_id: Uuid::new_v5(
                    &course_id,
                    b"7344f1c8-b7ce-4c7d-ade2-5f39997bd454",
                ),
            }),
            new_course,
            admin,
            models_requests::make_spec_fetcher(Arc::clone(&jwt_key)),
            models_requests::fetch_service_info,
        )
        .await?;

    // Create course instance
    course_instances::insert(
        &mut conn,
        PKeyPolicy::Fixed(Uuid::new_v5(
            &course_id,
            b"67f077b4-0562-47ae-a2b9-db2f08f168a9",
        )),
        NewCourseInstance {
            course_id: course.id,
            name: Some("Non-default instance"),
            description: Some("This is a non-default instance"),
            support_email: Some("contact@example.com"),
            teacher_in_charge_name: "admin",
            teacher_in_charge_email: "admin@example.com",
            opening_time: None,
            closing_time: None,
        },
    )
    .await?;

    // Chapter & Main page
    let new_chapter = NewChapter {
        chapter_number: 1,
        course_id: course.id,
        front_page_id: None,
        name: "Glossary".to_string(),
        color: None,
        opens_at: None,
        deadline: Some(Utc.with_ymd_and_hms(2025, 1, 1, 23, 59, 59).unwrap()),
        course_module_id: Some(default_module.id),
    };
    let (chapter, _front_page) = library::content_management::create_new_chapter(
        &mut conn,
        PKeyPolicy::Fixed((
            Uuid::new_v5(&course_id, b"3d1d7303-b654-428a-8b46-1dbfe908d38a"),
            Uuid::new_v5(&course_id, b"97568e97-0d6c-4702-9534-77d6e2784c8a"),
        )),
        &new_chapter,
        admin,
        models_requests::make_spec_fetcher(Arc::clone(&jwt_key)),
        models_requests::fetch_service_info,
    )
    .await?;
    chapters::set_opens_at(&mut conn, chapter.id, Utc::now()).await?;

    // Create page
    create_page(
        &mut conn,
        course.id,
        admin,
        Some(chapter.id),
        CmsPageUpdate {
            url_path: "/tooltip".to_string(),
            title: "Tooltip".to_string(),
            chapter_id: Some(chapter.id),
            exercises: vec![],
            exercise_slides: vec![],
            exercise_tasks: vec![],
            content: serde_json::json!([paragraph(
                "Use the KB to write sentences for your CS-courses.",
                Uuid::new_v5(&course.id, b"6903cf16-4f79-4985-a354-4257be1193a2")
            ),]),
        },
        Arc::clone(&jwt_key),
    )
    .await?;

    // Setup glossary
    glossary::insert(&mut conn, "CS", "Computer science. Computer science is an essential part of being successful in your life. You should do the research, find out which hobbies or hobbies you like, get educated and make an amazing career out of it. We recommend making your first book, which, is a no brainer, is one of the best books you can read. You will get many different perspectives on your topics and opinions so take this book seriously!",  course.id).await?;
    glossary::insert(&mut conn, "HDD", "Hard disk drive. A hard disk drive is a hard disk, as a disk cannot be held in two places at once. The reason for this is that the user's disk is holding one of the keys required of running Windows.",  course.id).await?;
    glossary::insert(&mut conn, "SSD", "Solid-state drive. A solid-state drive is a hard drive that's a few gigabytes in size, but a solid-state drive is one where data loads are big enough and fast enough that you can comfortably write to it over long distances. This is what drives do. You need to remember that a good solid-state drive has a lot of data: it stores files on disks and has a few data centers. A good solid-state drive makes for a nice little library: its metadata includes information about everything it stores, including any data it can access, but does not store anything that does not exist outside of those files. It also stores large amounts of data from one location, which can cause problems since the data might be different in different places, or in different ways, than what you would expect to see when driving big data applications. The drives that make up a solid-state drive are called drives that use a variety of storage technologies. These drive technology technologies are called \"super drives,\" and they store some of that data in a solid-state drive. Super drives are designed to be fast but very big: they aren't built to store everything, but to store many kinds of data: including data about the data they contain, and more, like the data they are supposed to hold in them. The super drives that make up a solid-state drive can have capacities of up to 50,000 hard disks. These can be used to store files if",  course.id).await?;
    glossary::insert(&mut conn, "KB", "Keyboard.", course.id).await?;

    Ok(course.id)
}

pub async fn seed_cs_course_material(
    db_pool: &Pool<Postgres>,
    org: Uuid,
    admin: Uuid,
    jwt_key: Arc<JwtKey>,
) -> Result<Uuid> {
    let mut conn = db_pool.acquire().await?;
    let spec_fetcher = models_requests::make_spec_fetcher(Arc::clone(&jwt_key));
    // Create new course
    let new_course = NewCourse {
        name: "Introduction to Course Material".to_string(),
        organization_id: org,
        slug: "introduction-to-course-material".to_string(),
        language_code: "en-US".to_string(),
        teacher_in_charge_name: "admin".to_string(),
        teacher_in_charge_email: "admin@example.com".to_string(),
        description: "The definitive introduction to course material.".to_string(),
        is_draft: false,
        is_test_mode: false,
    };
    let (course, front_page, _default_instance, default_module) =
        library::content_management::create_new_course(
            &mut conn,
            PKeyPolicy::Fixed(CreateNewCourseFixedIds {
                course_id: Uuid::parse_str("d6b52ddc-6c34-4a59-9a59-7e8594441007")?,
                default_course_instance_id: Uuid::parse_str(
                    "8e6c35cd-43f2-4982-943b-11e3ffb1b2f8",
                )?,
            }),
            new_course,
            admin,
            &spec_fetcher,
            models_requests::fetch_service_info,
        )
        .await?;

    // Exercises
    let (
        quizzes_exercise_block_5,
        quizzes_exercise_5,
        quizzes_exercise_slide_5,
        quizzes_exercise_task_5,
    ) = quizzes_exercise(
        "Best quizzes exercise".to_string(),
        Uuid::new_v5(&course.id, b"cd3aa815-620e-43b3-b291-0fb10beca030"),
        Uuid::new_v5(&course.id, b"0b1bbfb0-df56-4e40-92f1-df0a33f1fc70"),
        Uuid::new_v5(&course.id, b"7f011d0e-1cbf-4870-bacf-1873cf360c15"),
        Uuid::new_v5(&course.id, b"b9446b94-0edf-465c-9a9a-57708b7ef180"),
        Uuid::new_v5(&course.id, b"58e71279-81e1-4679-83e6-8f5f23ec055a"),
        false,
        serde_json::json!({
                "id": "3a1b3e10-2dd5-4cb9-9460-4c08f19e16d3",
                "body": "very hard",
                "part": 3,
                "items": [{
                    "id": "7b0049ea-de8b-4eef-a4a9-164e0e874ecc",
                    "body": "",
                    "type": "multiple-choice",
                    "direction": "row",
                    "multi": false,
                    "multipleChoiceMultipleOptionsGradingPolicy": "default",
                    "order": 0,
                    "title": "Choose the first answer",
                    "quizId": "3a1b3e10-2dd5-4cb9-9460-4c08f19e16d3",
                    "options": [{
                        "id": "d5124283-4e84-4b4f-84c0-a91961b0ef21",
                        "body": "This is first option",
                        "order": 1,
                        "title": null,
                        "quizItemId": "7b0049ea-de8b-4eef-a4a9-164e0e874ecc",
                        "correct":true,
                        "messageAfterSubmissionWhenSelected": "Correct! This is indeed the first answer",
                        "additionalCorrectnessExplanationOnModelSolution": null,
                    },{
                        "id": "846c09e2-653a-4471-81ae-25726486b003",
                        "body": "This is second option",
                        "order": 1,
                        "title": null,
                        "quizItemId": "7b0049ea-de8b-4eef-a4a9-164e0e874ecc",
                        "correct":false,
                        "messageAfterSubmissionWhenSelected": "Incorrect. This is not the first answer",
                        "additionalCorrectnessExplanationOnModelSolution": null,
                    },{
                        "id": "8107ae39-96aa-4f54-aa78-1a33362a19c1",
                        "body": "This is third option",
                        "order": 1,
                        "title": null,
                        "quizItemId": "7b0049ea-de8b-4eef-a4a9-164e0e874ecc",
                        "correct":false,
                        "messageAfterSubmissionWhenSelected": "Incorrect. This is not the first answer",
                        "additionalCorrectnessExplanationOnModelSolution": null,
                    },],
                    "allAnswersCorrect": false,
                    "sharedOptionFeedbackMessage": null,
                    "usesSharedOptionFeedbackMessage": false
                }],
                "title": "Very good exercise",
                "tries": 1,
                "points": 3,
                "section": 0,
                "courseId": "d6b52ddc-6c34-4a59-9a59-7e8594441007",
                "deadline": "2021-12-17T07:15:33.479Z",
                "createdAt": "2021-12-17T07:15:33.479Z",
                "updatedAt": "2021-12-17T07:15:33.479Z",
                "autoReject": false,
                "autoConfirm": true,
                "randomizeOptions": false,
                "triesLimited": true,
                "submitMessage": "This is an extra submit message from the teacher.",
                "excludedFromScore": true,
                "grantPointsPolicy": "grant_whenever_possible",
                "awardPointsEvenIfWrong": false}),
        Some(Utc.with_ymd_and_hms(2125, 1, 1, 23, 59, 59).unwrap()),
    );

    let (
        quizzes_exercise_block_6,
        quizzes_exercise_6,
        quizzes_exercise_slide_6,
        quizzes_exercise_task_6,
    ) = quizzes_exercise(
        "Best quizzes exercise".to_string(),
        Uuid::new_v5(&course.id, b"925d4a89-0f25-4e8e-bc11-350393d8d894"),
        Uuid::new_v5(&course.id, b"ff92ca4a-aa9c-11ec-ac56-475e57747ad3"),
        Uuid::new_v5(&course.id, b"9037cb17-3841-4a79-8f50-bbe595a4f785"),
        Uuid::new_v5(&course.id, b"d6d80ae0-97a1-4db1-8a3b-2bdde3cfbe9a"),
        Uuid::new_v5(&course.id, b"085b60ec-aa9d-11ec-b500-7b1e176646f8"),
        false,
        serde_json::from_str(include_str!(
            "../../assets/quizzes-multiple-choice-additional-feedback.json"
        ))?,
        Some(Utc.with_ymd_and_hms(2125, 1, 1, 23, 59, 59).unwrap()),
    );

    let (
        quizzes_exercise_block_7,
        quizzes_exercise_7,
        quizzes_exercise_slide_7,
        quizzes_exercise_task_7,
    ) = quizzes_exercise(
        "Best quizzes exercise".to_string(),
        Uuid::new_v5(&course.id, b"57905c8a-aa9d-11ec-92d4-47ab996cb70c"),
        Uuid::new_v5(&course.id, b"5b058552-aa9d-11ec-bc36-57e1c5f8407a"),
        Uuid::new_v5(&course.id, b"5d953894-aa9d-11ec-97e7-2ff4d73f69f1"),
        Uuid::new_v5(&course.id, b"604dae7c-aa9d-11ec-8df1-575042832340"),
        Uuid::new_v5(&course.id, b"6365746e-aa9d-11ec-8718-0b5628cbe29f"),
        false,
        serde_json::json!({
                "id": "33cd47ea-aa9d-11ec-897c-5b22513d61ee",
                "body": "very hard",
                "part": 5,
                "items": [{
                    "id": "395888c8-aa9d-11ec-bb81-cb3a3f2609e4",
                    "body": "",
                    "type": "multiple-choice",
                    "direction": "column",
                    "multi": false,
                    "multipleChoiceMultipleOptionsGradingPolicy": "default",
                    "order": 0,
                    "title": "Choose the first answer",
                    "quizId": "33cd47ea-aa9d-11ec-897c-5b22513d61ee",
                    "options": [{
                        "id": "490543d8-aa9d-11ec-a20f-07269e5c09df",
                        "body": "This is first option",
                        "order": 1,
                        "title": null,
                        "quizItemId": "395888c8-aa9d-11ec-bb81-cb3a3f2609e4",
                        "correct":true,
                        "messageAfterSubmissionWhenSelected": "Correct! This is indeed the first answer",
                        "additionalCorrectnessExplanationOnModelSolution": null,
                    },{
                        "id": "45e77450-aa9d-11ec-abea-6b824f5ae1f6",
                        "body": "This is second option",
                        "order": 1,
                        "title": null,
                        "quizItemId": "395888c8-aa9d-11ec-bb81-cb3a3f2609e4",
                        "correct":false,
                        "messageAfterSubmissionWhenSelected": "Incorrect. This is not the first answer",
                        "additionalCorrectnessExplanationOnModelSolution": null,
                    },{
                        "id": "43428140-aa9d-11ec-a6b3-83ec8e2dfb88",
                        "body": "This is third option",
                        "order": 1,
                        "title": null,
                        "quizItemId": "395888c8-aa9d-11ec-bb81-cb3a3f2609e4",
                        "correct":false,
                        "messageAfterSubmissionWhenSelected": "Incorrect. This is not the first answer",
                        "additionalCorrectnessExplanationOnModelSolution": null,
                    },],
                    "allAnswersCorrect": false,
                    "sharedOptionFeedbackMessage": null,
                    "usesSharedOptionFeedbackMessage": false
                }],
                "title": "Very good exercise",
                "tries": 1,
                "points": 3,
                "section": 0,
                "courseId": "d6b52ddc-6c34-4a59-9a59-7e8594441007",
                "deadline": "2021-12-17T07:15:33.479Z",
                "createdAt": "2021-12-17T07:15:33.479Z",
                "updatedAt": "2021-12-17T07:15:33.479Z",
                "autoReject": false,
                "autoConfirm": true,
                "randomizeOptions": false,
                "triesLimited": true,
                "submitMessage": "This is an extra submit message from the teacher.",
                "excludedFromScore": true,
                "grantPointsPolicy": "grant_whenever_possible",
                "awardPointsEvenIfWrong": false}),
        Some(Utc.with_ymd_and_hms(2125, 1, 1, 23, 59, 59).unwrap()),
    );

    let (
        quizzes_exercise_block_8,
        quizzes_exercise_8,
        quizzes_exercise_slide_8,
        quizzes_exercise_task_8,
    ) = quizzes_exercise(
        "Best quizzes exercise".to_string(),
        Uuid::new_v5(&course.id, b"c1a4831c-cc78-4f42-be18-2a35a7f3b506"),
        Uuid::new_v5(&course.id, b"75045b18-aa9d-11ec-b3d1-6f64c2d6d46d"),
        Uuid::new_v5(&course.id, b"712fd37c-e3d7-4569-8a64-371d7dda9c19"),
        Uuid::new_v5(&course.id, b"6799021d-ff0c-4e4d-b5db-c2c19fba7fb9"),
        Uuid::new_v5(&course.id, b"01b69776-3e82-4694-98a9-5ce53f2a4ab5"),
        false,
        serde_json::json!({
                "id": "9a186f2b-7616-472e-b839-62ab0f2f0a6c",
                "body": "very hard",
                "part": 6,
                "items": [{
                    "id": "871c3640-aa9d-11ec-8103-633d645899a3",
                    "body": "",
                    "type": "multiple-choice",
                    "direction": "column",
                    "multi": false,
                    "multipleChoiceMultipleOptionsGradingPolicy": "default",
                    "order": 0,
                    "title": "Choose the first answer",
                    "quizId": "9a186f2b-7616-472e-b839-62ab0f2f0a6c",
                    "options": [{
                        "id": "4435ed30-c1da-46a0-80b8-c5b9ee923dd4",
                        "body": "This is first option",
                        "order": 1,
                        "title": null,
                        "quizItemId": "871c3640-aa9d-11ec-8103-633d645899a3",
                        "correct":true,
                        "messageAfterSubmissionWhenSelected": "Correct! This is indeed the first answer",
                        "additionalCorrectnessExplanationOnModelSolution": null,
                    },{
                        "id": "1d5de4d0-8499-4ac1-b44c-21c1562639cb",
                        "body": "This is second option",
                        "order": 1,
                        "title": null,
                        "quizItemId": "871c3640-aa9d-11ec-8103-633d645899a3",
                        "correct":false,
                        "messageAfterSubmissionWhenSelected": "Incorrect. This is not the first answer",
                        "additionalCorrectnessExplanationOnModelSolution": null,
                    },{
                        "id": "93fe358e-aa9d-11ec-9aa1-f3d18a09d58c",
                        "body": "This is third option",
                        "order": 1,
                        "title": null,
                        "quizItemId": "871c3640-aa9d-11ec-8103-633d645899a3",
                        "correct":false,
                        "messageAfterSubmissionWhenSelected": "Incorrect. This is not the first answer",
                        "additionalCorrectnessExplanationOnModelSolution": null,
                    },],
                    "allAnswersCorrect": false,
                    "sharedOptionFeedbackMessage": null,
                    "usesSharedOptionFeedbackMessage": false
                }],
                "title": "Very good exercise",
                "tries": 1,
                "points": 3,
                "section": 0,
                "courseId": "d6b52ddc-6c34-4a59-9a59-7e8594441007",
                "deadline": "2021-12-17T07:15:33.479Z",
                "createdAt": "2021-12-17T07:15:33.479Z",
                "updatedAt": "2021-12-17T07:15:33.479Z",
                "autoReject": false,
                "autoConfirm": true,
                "randomizeOptions": false,
                "triesLimited": true,
                "submitMessage": "This is an extra submit message from the teacher.",
                "excludedFromScore": true,
                "grantPointsPolicy": "grant_whenever_possible",
                "awardPointsEvenIfWrong": false}),
        Some(Utc.with_ymd_and_hms(2125, 1, 1, 23, 59, 59).unwrap()),
    );

    pages::update_page(
        &mut conn,
        PageUpdateArgs {
            page_id: front_page.id,
            author: admin,
            cms_page_update: CmsPageUpdate {
                title: "Introduction to Course Material".to_string(),
                url_path: "/".to_string(),
                chapter_id: None,
                content: serde_json::to_value(&[
                    GutenbergBlock::landing_page_hero_section("Welcome to Introduction to Course Material", "In this course you'll learn the basics of UI/UX design. At the end of course you should be able to create your own design system.")
                    .with_id(Uuid::parse_str("6ad81525-0010-451f-85e5-4832e3e364a8")?),
                    GutenbergBlock::course_objective_section()
                        .with_id(Uuid::parse_str("2eec7ad7-a95f-406f-acfe-f3a332b86e26")?),
                    GutenbergBlock::empty_block_from_name("moocfi/course-chapter-grid".to_string())
                        .with_id(Uuid::parse_str("bb51d61b-fd19-44a0-8417-7ffc6058b247")?),
                    GutenbergBlock::empty_block_from_name("moocfi/course-progress".to_string())
                        .with_id(Uuid::parse_str("1d7c28ca-86ab-4318-8b10-3e5b7cd6e465")?),
                ])
                .unwrap(),
                exercises: vec![],
                exercise_slides: vec![],
                exercise_tasks: vec![],
            },
            retain_ids: true,
            history_change_reason: HistoryChangeReason::PageSaved,
            is_exam_page: false
        },
        &spec_fetcher,
        models_requests::fetch_service_info,
    )
    .await?;
    // FAQ, we should add card/accordion block to visualize here.
    let (_page, _history) = pages::insert_course_page(
        &mut conn,
        &NewCoursePage::new(course.id, 1, "/faq", "FAQ"),
        admin,
    )
    .await?;

    // Chapter-1
    let new_chapter = NewChapter {
        chapter_number: 1,
        course_id: course.id,
        front_page_id: None,
        name: "User Interface".to_string(),
        color: None,
        opens_at: None,
        deadline: None,
        course_module_id: Some(default_module.id),
    };
    let (chapter_1, front_page_ch_1) = library::content_management::create_new_chapter(
        &mut conn,
        PKeyPolicy::Fixed((
            Uuid::new_v5(&course.id, b"77e95910-2289-452f-a1dd-8b8bf4a829a0"),
            Uuid::new_v5(&course.id, b"91b6887f-8bc0-4df6-89a4-5687890bc955"),
        )),
        &new_chapter,
        admin,
        &spec_fetcher,
        models_requests::fetch_service_info,
    )
    .await?;
    chapters::set_opens_at(&mut conn, chapter_1.id, Utc::now()).await?;

    pages::update_page(
        &mut conn,
        PageUpdateArgs {
            page_id: front_page_ch_1.id,
            author: admin,
            cms_page_update: CmsPageUpdate {
                title: "User Interface".to_string(),
                url_path: "/chapter-1".to_string(),
                chapter_id: Some(chapter_1.id),
                content: serde_json::to_value(&[
                    GutenbergBlock::hero_section("User Interface", "In the industrial design field of human–computer interaction, a user interface is the space where interactions between humans and machines occur.")
                    .with_id(Uuid::parse_str("848ac898-81c0-4ebc-881f-6f84e9eaf472")?),
                GutenbergBlock::empty_block_from_name("moocfi/pages-in-chapter".to_string())
                    .with_id(Uuid::parse_str("c8b36f58-5366-4d6b-b4ec-9fc0bd65950e")?),
                GutenbergBlock::empty_block_from_name("moocfi/chapter-progress".to_string())
                    .with_id(Uuid::parse_str("cdb9e4b9-ba68-4933-b037-4648e3df7a6c")?),
                GutenbergBlock::empty_block_from_name("moocfi/exercises-in-chapter".to_string())
                    .with_id(Uuid::parse_str("457431b0-55db-46ac-90ae-03965f48b27e")?),
                ])
                .unwrap(),
                exercises: vec![],
                exercise_slides: vec![],
                exercise_tasks: vec![],
            },
            retain_ids: true,
            history_change_reason: HistoryChangeReason::PageSaved,
            is_exam_page: false
        },
        &spec_fetcher,
        models_requests::fetch_service_info,
    )
    .await?;

    // /chapter-1/design
    let design_content = CmsPageUpdate {
        url_path: "/chapter-1/design".to_string(),
        title: "Design".to_string(),
        chapter_id: Some(chapter_1.id),
        exercises: vec![],
        exercise_slides: vec![],
        exercise_tasks: vec![],
        content: serde_json::json!([
            GutenbergBlock::hero_section("Design", "A design is a plan or specification for the construction of an object or system or for the implementation of an activity or process, or the result of that plan or specification in the form of a prototype, product or process.")
                .with_id(Uuid::parse_str("98729704-9dd8-4309-aa08-402f9b2a6071")?),
            heading("First heading", Uuid::parse_str("731aa55f-238b-42f4-8c40-c093dd95ee7f")?, 2),
            GutenbergBlock::block_with_name_and_attributes(
                "core/paragraph",
                attributes!{
                  "content": "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Curabitur bibendum felis nisi, vitae commodo mi venenatis in. Mauris hendrerit lacinia augue ut hendrerit. Vestibulum non tellus mattis, convallis magna vel, semper mauris. Maecenas porta, arcu eget porttitor sagittis, nulla magna auctor dolor, sed tempus sem lacus eu tortor. Ut id diam quam. Etiam quis sagittis justo. Quisque sagittis dolor vitae felis facilisis, ut suscipit ipsum malesuada. Nulla tempor ultricies erat ut venenatis. Ut pulvinar lectus non mollis efficitur.",
                  "dropCap": false
                },
            )
                .with_id(Uuid::parse_str("9ebddb78-23f6-4440-8d8f-5e4b33abb16f")?),
                heading("Second heading", Uuid::parse_str("a70aac40-acda-48e3-8f53-b64370be4585")?, 3),
            GutenbergBlock::block_with_name_and_attributes(
                "core/paragraph",
                attributes!{
                  "content": "Sed quis fermentum mi. Integer commodo turpis a fermentum tristique. Integer convallis, nunc sed scelerisque varius, mi tellus molestie metus, eu ultrices justo tellus non arcu. Cras euismod, lectus eu scelerisque mattis, odio ex ornare ipsum, a dapibus nulla leo maximus orci. Etiam laoreet venenatis lorem, vitae iaculis mauris. Nullam lobortis, tortor eget ullamcorper lobortis, tellus odio tincidunt dolor, vitae gravida nibh turpis ac sem. Integer non sodales eros.",
                  "dropCap": false
                },
            )
                .with_id(Uuid::parse_str("029ae4b5-08b0-49f7-8baf-d916b5f879a2")?),
            GutenbergBlock::block_with_name_and_attributes(
                "core/paragraph",
                attributes!{
                  "content": "Vestibulum a scelerisque ante. Fusce interdum eros elit, posuere mattis sapien tristique id. Integer commodo mi orci, sit amet tempor libero vulputate in. Ut id gravida quam. Proin massa dolor, posuere nec metus eu, dignissim viverra nulla. Vestibulum quis neque bibendum, hendrerit diam et, fermentum diam. Sed risus nibh, suscipit in neque nec, bibendum interdum nibh. Aliquam ut enim a mi ultricies finibus. Nam tristique felis ac risus interdum molestie. Nulla venenatis, augue sed porttitor ultrices, lacus ante sollicitudin dui, vel vehicula ex enim ac mi.",
                  "dropCap": false
                },
            )
            .with_id(Uuid::parse_str("3693e92b-9cf0-485a-b026-2851de58e9cf")?),
            heading("Third heading", Uuid::parse_str("4d16bfea-4fa9-4355-bbd4-4c61e33d3d7c")?, 2),
            GutenbergBlock::block_with_name_and_attributes(
                "core/paragraph",
                attributes!{
                  "content": "Sed quis fermentum mi. Integer commodo turpis a fermentum tristique. Integer convallis, nunc sed scelerisque varius, mi tellus molestie metus, eu ultrices justo tellus non arcu. Cras euismod, lectus eu scelerisque mattis, odio ex ornare ipsum, a dapibus nulla leo maximus orci. Etiam laoreet venenatis lorem, vitae iaculis mauris. Nullam lobortis, tortor eget ullamcorper lobortis, tellus odio tincidunt dolor, vitae gravida nibh turpis ac sem. Integer non sodales eros.",
                  "dropCap": false
                },
            )
                .with_id(Uuid::parse_str("4ef39962-634d-488c-be82-f44e5db19421")?),
            GutenbergBlock::block_with_name_and_attributes(
                "core/paragraph",
                attributes!{
                  "content": "Vestibulum a scelerisque ante. Fusce interdum eros elit, posuere mattis sapien tristique id. Integer commodo mi orci, sit amet tempor libero vulputate in. Ut id gravida quam. Proin massa dolor, posuere nec metus eu, dignissim viverra nulla. Vestibulum quis neque bibendum, hendrerit diam et, fermentum diam. Sed risus nibh, suscipit in neque nec, bibendum interdum nibh. Aliquam ut enim a mi ultricies finibus. Nam tristique felis ac risus interdum molestie. Nulla venenatis, augue sed porttitor ultrices, lacus ante sollicitudin dui, vel vehicula ex enim ac mi.",
                  "dropCap": false
                },
            )
            .with_id(Uuid::parse_str("0d47c02a-194e-42a4-927e-fb29a4fda39c")?),
        ]),
    };
    create_page(
        &mut conn,
        course.id,
        admin,
        Some(chapter_1.id),
        design_content,
        Arc::clone(&jwt_key),
    )
    .await?;

    // /chapter-1/human-machine-interface
    let content_b = CmsPageUpdate {
        chapter_id: Some(chapter_1.id),
        url_path: "/chapter-1/human-machine-interface".to_string(),
        title: "Human-machine interface".to_string(),
        exercises: vec![],
        exercise_slides: vec![],
        exercise_tasks: vec![],
        content: serde_json::json!([
            GutenbergBlock::hero_section("Human-machine interface", "In the industrial design field of human–computer interaction, a user interface is the space where interactions between humans and machines occur.")
                .with_id(Uuid::parse_str("ae22ae64-c0e5-42e1-895a-4a49411a72e8")?),
            GutenbergBlock::block_with_name_and_attributes(
                "core/paragraph",
                attributes!{
                  "content": "Sed venenatis, magna in ornare suscipit, orci ipsum consequat nulla, ut pulvinar libero metus et metus. Maecenas nec bibendum est. Donec quis ante elit. Nam in eros vitae urna aliquet vestibulum. Donec posuere laoreet facilisis. Aliquam auctor a tellus a tempus. Sed molestie leo eget commodo pellentesque. Curabitur lacinia odio nisl, eu sodales nunc placerat sit amet. Vivamus venenatis, risus vitae lobortis eleifend, odio nisi faucibus tortor, sed aliquet leo arcu et tellus. Donec ultrices consectetur nunc, non rhoncus sapien malesuada et. Nulla tempus ipsum vitae justo scelerisque, sed pretium neque fermentum. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. Curabitur accumsan et ex pellentesque dignissim. Integer viverra libero quis tortor dignissim elementum.",
                  "dropCap": false
                },
            )
                .with_id(Uuid::parse_str("b05a62ad-e5f7-432c-8c88-2976d971e7e1")?),
            GutenbergBlock::block_with_name_and_attributes(
                "core/paragraph",
                attributes!{
                    "content": "Sed quis fermentum mi. Integer commodo turpis a fermentum tristique. Integer convallis, nunc sed scelerisque varius, mi tellus molestie metus, eu ultrices banana justo tellus non arcu. Cras euismod, cat lectus eu scelerisque mattis, odio ex ornare ipsum, a dapibus nulla leo maximus orci. Etiam laoreet venenatis lorem, vitae iaculis mauris. Nullam lobortis, tortor eget ullamcorper lobortis, tellus odio tincidunt dolor, vitae gravida nibh turpis ac sem. Integer non sodales eros.",
                    "dropCap": false
                },
            )
                .with_id(Uuid::parse_str("db20e302-d4e2-4f56-a0b9-e48a4fbd5fa8")?),
            GutenbergBlock::block_with_name_and_attributes(
                "core/paragraph",
                attributes!{
                  "content": "Vestibulum a scelerisque ante. Fusce interdum eros elit, posuere mattis sapien tristique id. Integer commodo mi orci, sit amet tempor libero vulputate in. Ut id gravida quam. Proin massa dolor, posuere nec metus eu, dignissim viverra nulla. Vestibulum quis neque bibendum, hendrerit diam et, fermentum diam. Sed risus nibh, suscipit in neque nec, bibendum interdum nibh. Aliquam ut enim a mi ultricies finibus. Nam tristique felis ac risus interdum molestie. Nulla venenatis, augue sed porttitor ultrices, lacus ante sollicitudin dui, vel vehicula ex enim ac mi.",
                  "dropCap": false
                },
            )
            .with_id(Uuid::parse_str("c96f56d5-ea35-4aae-918a-72a36847a49c")?),
        ]),
    };
    create_page(
        &mut conn,
        course.id,
        admin,
        Some(chapter_1.id),
        content_b,
        Arc::clone(&jwt_key),
    )
    .await?;

    // Chapter-2
    let new_chapter_2 = NewChapter {
        chapter_number: 2,
        course_id: course.id,
        front_page_id: None,
        name: "User Experience".to_string(),
        color: None,
        opens_at: None,
        deadline: None,
        course_module_id: Some(default_module.id),
    };
    let (chapter_2, front_page_ch_2) = library::content_management::create_new_chapter(
        &mut conn,
        PKeyPolicy::Fixed((
            Uuid::new_v5(&course.id, b"5adff726-8910-4163-9fdb-e2f0f45c04d7"),
            Uuid::new_v5(&course.id, b"4d916791-5a09-4e3c-8201-c46509e0b2c7"),
        )),
        &new_chapter_2,
        admin,
        &spec_fetcher,
        models_requests::fetch_service_info,
    )
    .await?;
    chapters::set_opens_at(&mut conn, chapter_2.id, Utc::now()).await?;

    pages::update_page(
        &mut conn,
        PageUpdateArgs {
            page_id: front_page_ch_2.id,
            author: admin,
            cms_page_update: CmsPageUpdate {
                url_path: "/chapter-2".to_string(),
                title: "User Experience".to_string(),
                chapter_id: Some(chapter_2.id),
                content: serde_json::to_value(&[
                    GutenbergBlock::hero_section("User Experience", "The user experience is how a user interacts with and experiences a product, system or service. It includes a person's perceptions of utility, ease of use, and efficiency.")
                        .with_id(Uuid::parse_str("c5c623f9-c7ca-4f8e-b04b-e91cecef217a")?),
                    GutenbergBlock::empty_block_from_name("moocfi/pages-in-chapter".to_string())
                        .with_id(Uuid::parse_str("37bbc4e9-2e96-45ea-a6f8-bbc7dc7f6be3")?),
                    GutenbergBlock::empty_block_from_name("moocfi/chapter-progress".to_string())
                        .with_id(Uuid::parse_str("2e91c140-fd17-486b-8dc1-0a9589a18e3a")?),
                    GutenbergBlock::empty_block_from_name("moocfi/exercises-in-chapter".to_string())
                        .with_id(Uuid::parse_str("1bf7e311-75e8-48ec-bd55-e8f1185d76d0")?),
                ])
                .unwrap(),
                exercises: vec![],
                exercise_slides: vec![],
                exercise_tasks: vec![],
            },
            retain_ids: true,
            history_change_reason: HistoryChangeReason::PageSaved,
            is_exam_page: false
        },
        &spec_fetcher,
        models_requests::fetch_service_info,
    )
    .await?;
    // /chapter-2/user-research
    let page_content = CmsPageUpdate {
        chapter_id: Some(chapter_2.id),
        content: serde_json::json!([
            GutenbergBlock::hero_section("User research", "User research focuses on understanding user behaviors, needs, and motivations through observation techniques, task analysis, and other feedback methodologies.")
                .with_id(Uuid::parse_str("a43f5460-b588-44ac-84a3-5fdcabd5d3f7")?),
            GutenbergBlock::block_with_name_and_attributes(
                "core/paragraph",
                attributes!{
                  "content": "Sed venenatis, magna in ornare suscipit, orci ipsum consequat nulla, ut pulvinar libero metus et metus. Maecenas nec bibendum est. Donec quis ante elit. Nam in eros vitae urna aliquet vestibulum. Donec posuere laoreet facilisis. Aliquam auctor a tellus a tempus. Sed molestie leo eget commodo pellentesque. Curabitur lacinia odio nisl, eu sodales nunc placerat sit amet. Vivamus venenatis, risus vitae lobortis eleifend, odio nisi faucibus tortor, sed aliquet leo arcu et tellus. Donec ultrices consectetur nunc, non rhoncus sapien malesuada et. Nulla tempus ipsum vitae justo scelerisque, sed pretium neque fermentum. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. Curabitur accumsan et ex pellentesque dignissim. Integer viverra libero quis tortor dignissim elementum.",
                  "dropCap": false
                },
            )
                .with_id(Uuid::parse_str("816310e3-bbd7-44ae-87cb-3f40633a4b08")?),
            GutenbergBlock::block_with_name_and_attributes(
                "core/paragraph",
                attributes!{
                  "content": "Sed quis fermentum mi. Integer commodo turpis a fermentum tristique. Integer convallis, nunc sed scelerisque varius, mi tellus molestie metus, eu ultrices justo tellus non arcu. Cras euismod, lectus eu scelerisque mattis, odio ex ornare ipsum, a dapibus nulla leo maximus orci. Etiam laoreet venenatis lorem, vitae iaculis mauris. Nullam lobortis, tortor eget ullamcorper lobortis, tellus odio tincidunt dolor, vitae gravida nibh turpis ac sem. Integer non sodales eros.",
                  "dropCap": false
                },
            )
                .with_id(Uuid::parse_str("37aa6421-768e-49b9-b447-5f457e5192bc")?),
            GutenbergBlock::block_with_name_and_attributes(
                "core/paragraph",
                attributes!{
                    "content": "Vestibulum a scelerisque ante. Fusce interdum eros elit, posuere mattis sapien tristique id. Integer commodo mi orci, sit amet tempor libero vulputate in. Ut id gravida quam. Proin massa dolor, posuere nec metus eu, dignissim viverra nulla. Vestibulum quis neque bibendum, hendrerit diam et, fermentum diam. Sed risus nibh, suscipit in neque nec, bibendum interdum nibh. Aliquam ut banana cat enim a mi ultricies finibus. Nam tristique felis ac risus interdum molestie. Nulla venenatis, augue sed porttitor ultrices, lacus ante sollicitudin dui, vel vehicula ex enim ac mi.",
                  "dropCap": false
                },
            )
            .with_id(Uuid::parse_str("cf11a0fb-f56e-4e0d-bc12-51d920dbc278")?),
        ]),
        exercises: vec![],
        exercise_slides: vec![],
        exercise_tasks: vec![],
        url_path: "/chapter-2/user-research".to_string(),
        title: "User research".to_string(),
    };
    create_page(
        &mut conn,
        course.id,
        admin,
        Some(chapter_2.id),
        page_content,
        Arc::clone(&jwt_key),
    )
    .await?;

    let page_content = include_str!("../../assets/example-page.json");
    let parse_page_content = serde_json::from_str(page_content)?;
    create_page(
        &mut conn,
        course.id,
        admin,
        Some(chapter_2.id),
        CmsPageUpdate {
            content: parse_page_content,
            exercises: vec![],
            exercise_slides: vec![],
            exercise_tasks: vec![],
            url_path: "/chapter-2/content-rendering".to_string(),
            title: "Content rendering".to_string(),
            chapter_id: Some(chapter_2.id),
        },
        Arc::clone(&jwt_key),
    )
    .await?;

    // Multiple choice

    create_page(
        &mut conn,
        course.id,
        admin,
        Some(chapter_2.id),
        CmsPageUpdate {
            url_path: "/chapter-2/page-3".to_string(),
            title: "Page 3".to_string(),
            chapter_id: Some(chapter_2.id),
            exercises: vec![quizzes_exercise_5],
            exercise_slides: vec![quizzes_exercise_slide_5],
            exercise_tasks: vec![quizzes_exercise_task_5],
            content: serde_json::json!([
                paragraph(
                    "Second chapters third page",
                    Uuid::new_v5(&course.id, b"4ebd0208-8328-5d69-8c44-ec50939c0967")
                ),
                quizzes_exercise_block_5,
            ]),
        },
        jwt_key.clone(),
    )
    .await?;

    create_page(
        &mut conn,
        course.id,
        admin,
        Some(chapter_2.id),
        CmsPageUpdate {
            url_path: "/chapter-2/page-4".to_string(),
            title: "Page 4".to_string(),
            chapter_id: Some(chapter_2.id),
            exercises: vec![quizzes_exercise_6],
            exercise_slides: vec![quizzes_exercise_slide_6],
            exercise_tasks: vec![quizzes_exercise_task_6],
            content: serde_json::json!([
                paragraph(
                    "Second chapters fourth page",
                    Uuid::new_v5(&course.id, b"4841cabb-77a0-53cf-b539-39fbd060e73b")
                ),
                quizzes_exercise_block_6,
            ]),
        },
        jwt_key.clone(),
    )
    .await?;

    create_page(
        &mut conn,
        course.id,
        admin,
        Some(chapter_2.id),
        CmsPageUpdate {
            url_path: "/chapter-2/page-5".to_string(),
            title: "Page 5".to_string(),
            chapter_id: Some(chapter_2.id),
            exercises: vec![quizzes_exercise_7],
            exercise_slides: vec![quizzes_exercise_slide_7],
            exercise_tasks: vec![quizzes_exercise_task_7],

            content: serde_json::json!([
                paragraph(
                    "Second chapters fifth page",
                    Uuid::new_v5(&course.id, b"9a614406-e1b4-5920-8e0d-54d1a3ead5f3")
                ),
                quizzes_exercise_block_7,
            ]),
        },
        jwt_key.clone(),
    )
    .await?;

    create_page(
        &mut conn,
        course.id,
        admin,
        Some(chapter_2.id),
        CmsPageUpdate {
            url_path: "/chapter-2/page-6".to_string(),
            title: "Page 6".to_string(),
            chapter_id: Some(chapter_2.id),
            exercises: vec![quizzes_exercise_8],
            exercise_slides: vec![quizzes_exercise_slide_8],
            exercise_tasks: vec![quizzes_exercise_task_8],
            content: serde_json::json!([
                paragraph(
                    "Second chapters sixth page",
                    Uuid::new_v5(&course.id, b"891de1ca-f3a9-506f-a268-3477ea4fdd27")
                ),
                quizzes_exercise_block_8,
            ]),
        },
        jwt_key,
    )
    .await?;

    Ok(course.id)
}
