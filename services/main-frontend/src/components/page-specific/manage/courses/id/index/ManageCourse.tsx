import { css } from "@emotion/css"
import { Dialog } from "@mui/material"
import {
  QueryObserverResult,
  RefetchOptions,
  RefetchQueryFilters,
  useQueryClient,
} from "@tanstack/react-query"
import React, { useState } from "react"
import { useTranslation } from "react-i18next"

import { deleteCourse, postNewCourseTranslation } from "../../../../../../services/backend/courses"
import { Course, NewCourse } from "../../../../../../shared-module/bindings"
import Button from "../../../../../../shared-module/components/Button"
import OnlyRenderIfPermissions from "../../../../../../shared-module/components/OnlyRenderIfPermissions"
import useToastMutation from "../../../../../../shared-module/hooks/useToastMutation"
import { baseTheme, headingFont } from "../../../../../../shared-module/styles"
import NewCourseForm from "../../../../../forms/NewCourseForm"
import CourseCourseInstances from "../course-instances/CourseCourseInstances"
import CourseLanguageVersionsList, {
  formatLanguageVersionsQueryKey,
} from "../language-versions/CourseLanguageVersionsList"

import UpdateCourseForm from "./UpdateCourseForm"
import UpdatePeerReviewQueueReviewsReceivedButton from "./UpdatePeerReviewQueueReviewsReceivedButton"

interface Props {
  course: Course
  refetch: (
    options?: (RefetchOptions & RefetchQueryFilters<unknown>) | undefined,
  ) => Promise<QueryObserverResult<Course, unknown>>
}

const ManageCourse: React.FC<React.PropsWithChildren<Props>> = ({ course, refetch }) => {
  const { t } = useTranslation()
  const queryClient = useQueryClient()
  const [showForm, setShowForm] = useState(false)
  const [showNewLanguageVersionForm, setShowNewLanguageVersionForm] = useState(false)
  const deleteCourseMutation = useToastMutation(
    async () => {
      await deleteCourse(course.id)
      await refetch()
    },
    {
      notify: true,
      // eslint-disable-next-line i18next/no-literal-string
      method: "DELETE",
    },
    {
      onSuccess: async () => {
        await refetch()
      },
    },
  )

  const handleOnUpdateCourse = async () => {
    setShowForm(!showForm)
    await refetch()
  }

  const handleCreateNewLanguageVersion = async (newCourse: NewCourse) => {
    await postNewCourseTranslation(course.id, newCourse)
    await refetch()
    setShowNewLanguageVersionForm(false)
    queryClient.invalidateQueries([formatLanguageVersionsQueryKey(course.id)])
  }

  return (
    <>
      <h1
        className={css`
          font-size: clamp(2rem, 3.6vh, 36px);
          color: ${baseTheme.colors.gray[700]};
          font-family: ${headingFont};
          font-weight: bold;
        `}
      >
        {course.name}
        {course.is_draft && ` (${t("draft")})`}
        {course.deleted_at && ` (${t("deleted")})`}
      </h1>
      <OnlyRenderIfPermissions
        action={{
          type: "usually_unacceptable_deletion",
        }}
        resource={{
          type: "course",
          id: course.id,
        }}
      >
        <Button
          variant="secondary"
          size="medium"
          onClick={() => {
            const confirmation = confirm(
              // eslint-disable-next-line i18next/no-literal-string
              `${t("delete-course-confirmation")}\n\n${t(
                "delete-course-confirmation-explanation",
              )}`,
            )
            if (confirmation) {
              deleteCourseMutation.mutate()
            }
          }}
        >
          {t("button-text-delete")}
        </Button>
      </OnlyRenderIfPermissions>
      <Button variant="primary" size="medium" onClick={() => setShowForm(!showForm)}>
        {t("edit")}
      </Button>
      <Dialog open={showForm} onClose={() => setShowForm(!showForm)}>
        <div
          className={css`
            margin: 1rem;
          `}
        >
          <Button variant="primary" size="medium" onClick={() => setShowForm(!showForm)}>
            {t("button-text-close")}
          </Button>
          <UpdateCourseForm
            courseId={course.id}
            courseName={course.name}
            courseDescription={course.description}
            isDraft={course.is_draft}
            isTest={course.is_test_mode}
            onSubmitForm={handleOnUpdateCourse}
          />
        </div>
      </Dialog>
      <Dialog open={showNewLanguageVersionForm} onClose={() => setShowNewLanguageVersionForm(true)}>
        <div
          className={css`
            margin: 1rem;
          `}
        >
          <div>{t("create-new-language-version-of", { "course-name": course.name })}</div>
          <NewCourseForm
            organizationId={course.organization_id}
            onSubmitNewCourseForm={handleCreateNewLanguageVersion}
            onClose={() => setShowNewLanguageVersionForm(false)}
          />
        </div>
      </Dialog>

      <h2>{t("title-all-course-language-versions")}</h2>
      <CourseLanguageVersionsList courseId={course.id} />
      <Button size="medium" variant="primary" onClick={() => setShowNewLanguageVersionForm(true)}>
        {t("button-text-new")}
      </Button>
      <CourseCourseInstances courseId={course.id} />

      <UpdatePeerReviewQueueReviewsReceivedButton courseId={course.id} />
    </>
  )
}

export default ManageCourse
