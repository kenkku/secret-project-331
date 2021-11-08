#[cfg(test)]
use crate::{
    controllers::{
        auth::Login,
        main_frontend::{
            courses::GetFeedbackQuery, exercises::ExerciseSubmissions, feedback::MarkAsRead,
            proposed_edits::GetEditProposalsQuery,
        },
        ErrorResponse, UploadResult,
    },
    models::{
        chapters::{
            Chapter, ChapterStatus, ChapterUpdate, ChapterWithStatus, NewChapter,
            UserCourseInstanceChapterProgress,
        },
        course_instance_enrollments::CourseInstanceEnrollment,
        course_instances::{CourseInstance, CourseInstanceForm, VariantStatus},
        courses::{Course, CourseStructure, CourseUpdate, NewCourse},
        email_templates::{EmailTemplate, EmailTemplateNew, EmailTemplateUpdate},
        exercise_service_info::{CourseMaterialExerciseServiceInfo, ExerciseServiceInfoApi},
        exercise_services::{ExerciseService, ExerciseServiceNewOrUpdate},
        exercise_slides::ExerciseSlide,
        exercise_tasks::{CourseMaterialExerciseTask, ExerciseTask},
        exercises::{
            ActivityProgress, CourseMaterialExercise, Exercise, ExerciseStatus, GradingProgress,
        },
        feedback::{Feedback, FeedbackBlock, FeedbackCount, NewFeedback},
        gradings::{Grading, UserPointsUpdateStrategy},
        organizations::Organization,
        page_history::{HistoryChangeReason, PageHistory},
        pages::{
            CmsPageExercise, CmsPageExerciseSlide, CmsPageExerciseTask, CmsPageUpdate,
            ContentManagementPage, CoursePageWithUserData, ExerciseWithExerciseTasks,
            HistoryRestoreData, NewPage, Page, PageRoutingDataWithChapterStatus, PageSearchRequest,
            PageSearchResult, PageWithExercises,
        },
        playground_examples::{PlaygroundExample, PlaygroundExampleData},
        proposed_block_edits::{
            BlockProposal, BlockProposalAction, BlockProposalInfo, NewProposedBlockEdit,
            ProposalStatus,
        },
        proposed_page_edits::{
            EditProposalInfo, NewProposedPageEdits, PageProposal, ProposalCount,
        },
        submissions::{
            NewSubmission, Submission, SubmissionCount, SubmissionCountByExercise,
            SubmissionCountByWeekAndHour, SubmissionInfo, SubmissionResult,
        },
        user_course_settings::UserCourseSettings,
        user_exercise_states::{
            UserCourseInstanceChapterExerciseProgress, UserCourseInstanceProgress,
        },
    },
    utils::pagination::Pagination,
};

ts_rs::export! {
  Chapter,
  EmailTemplate,
  CmsPageExercise,
  CmsPageExerciseSlide,
  CmsPageExerciseTask,
  CmsPageUpdate,
  ContentManagementPage,
  CourseStructure,
  Page,
  UploadResult,
  PageWithExercises,
  UserCourseInstanceProgress,
  UserCourseInstanceChapterProgress,
  UserCourseInstanceChapterExerciseProgress,
  CourseInstanceEnrollment,
  CourseInstance,
  ChapterWithStatus,
  CourseMaterialExercise,
  PageRoutingDataWithChapterStatus,
  SubmissionResult,
  ExerciseService,
  ExerciseServiceNewOrUpdate,
  Course,
  Exercise,
  ExerciseSlide,
  ExerciseServiceInfoApi,
  SubmissionCount,
  SubmissionCountByWeekAndHour,
  SubmissionCountByExercise,
  ExerciseSubmissions,
  Organization,
  NewChapter,
  ChapterUpdate,
  EmailTemplateNew,
  EmailTemplateUpdate,
  NewPage,
  NewSubmission,
  NewCourse,
  CourseUpdate,
  Login,
  SubmissionInfo,
  PageSearchResult,
  PageSearchRequest,
  PageHistory,
  HistoryChangeReason,
  HistoryRestoreData,
  Feedback,
  MarkAsRead,
  NewFeedback,
  FeedbackBlock,
  FeedbackCount,
  GetFeedbackQuery,
  CourseInstanceForm,
  PageProposal,
  BlockProposal,
  ProposalCount,
  EditProposalInfo,
  GetEditProposalsQuery,
  NewProposedPageEdits,
  ErrorResponse,
  // dependencies
  VariantStatus,
  ChapterStatus,
  CourseMaterialExerciseTask,
  CourseMaterialExerciseServiceInfo,
  ExerciseStatus,
  Submission,
  Grading,
  ActivityProgress,
  GradingProgress,
  UserPointsUpdateStrategy,
  Pagination,
  ProposalStatus,
  NewProposedBlockEdit,
  BlockProposalInfo,
  BlockProposalAction,
  // returned from the API as serde_json::Value
  ExerciseTask,
  ExerciseWithExerciseTasks,
  UserCourseSettings,
  PlaygroundExample,PlaygroundExampleData,
  CoursePageWithUserData
    => "../../shared-module/src/bindings.ts"
}
