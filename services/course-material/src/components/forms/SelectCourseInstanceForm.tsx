import { css } from "@emotion/css"
import styled from "@emotion/styled"
import { useQuery } from "@tanstack/react-query"
import React, { useState } from "react"
import { useTranslation } from "react-i18next"

import { fetchBackgroundQuestionsAndAnswers } from "../../services/backend"
import { CourseInstance, NewCourseBackgroundQuestionAnswer } from "../../shared-module/bindings"
import Button from "../../shared-module/components/Button"
import ErrorBanner from "../../shared-module/components/ErrorBanner"
import CheckBox from "../../shared-module/components/InputFields/CheckBox"
import RadioButton from "../../shared-module/components/InputFields/RadioButton"
import { baseTheme } from "../../shared-module/styles"
import { assertNotNullOrUndefined } from "../../shared-module/utils/nullability"

const FieldContainer = styled.div`
  margin-bottom: 1.5rem;
`

// eslint-disable-next-line i18next/no-literal-string
const GreenText = styled.span`
  color: ${baseTheme.colors.green[700]};
`

const AdditionalQuestionWrapper = styled.div`
  margin: 0.5rem 0;
`

interface SelectCourseInstanceFormProps {
  courseInstances: CourseInstance[]
  onSubmitForm: (
    courseInstanceId: string,
    newCourseBackgroundQuestionAnswer: NewCourseBackgroundQuestionAnswer[],
  ) => void
  initialSelectedInstanceId?: string
}

const SelectCourseInstanceForm: React.FC<
  React.PropsWithChildren<SelectCourseInstanceFormProps>
> = ({ courseInstances, onSubmitForm, initialSelectedInstanceId }) => {
  const { t } = useTranslation()
  const [instance, setInstance] = useState(
    figureOutInitialValue(courseInstances, initialSelectedInstanceId),
  )
  const [additionalQuestionAnswers, setAdditionalQuestionAnswers] = useState<
    NewCourseBackgroundQuestionAnswer[]
  >([])
  const additionalQuestionsQuery = useQuery(
    ["additional-questions", instance],
    () => fetchBackgroundQuestionsAndAnswers(assertNotNullOrUndefined(instance)),
    {
      enabled: instance !== undefined,
      onSuccess(data) {
        // Populates initial answers for all questions
        setAdditionalQuestionAnswers((prev) => {
          const newState: NewCourseBackgroundQuestionAnswer[] = []
          data.background_questions.forEach((question) => {
            const prevAnswer = prev.find((a) => a.course_background_question_id === question.id)
            const savedAnswer = data.answers.find(
              (a) => a.course_background_question_id === question.id,
            )
            let initialValue = prevAnswer?.answer_value ?? savedAnswer?.answer_value ?? null
            if (question.question_type === "Checkbox" && initialValue === null) {
              initialValue = "f"
            }
            newState.push({
              answer_value: initialValue,
              course_background_question_id: question.id,
            })
          })
          return newState
        })
      },
    },
  )

  const enrollOnCourse = async () => {
    if (instance) {
      onSubmitForm(instance, additionalQuestionAnswers)
    }
  }

  const additionalQuestions = additionalQuestionsQuery.data?.background_questions

  return (
    <div>
      <>
        <h2>
          {t("title-select-course-instance")}
          <GreenText>*</GreenText>
        </h2>
        <FieldContainer
          role="radiogroup"
          aria-label={t("label-course-instance")}
          // eslint-disable-next-line i18next/no-literal-string
          aria-required="true"
        >
          {courseInstances.map((x) => (
            <RadioButton
              key={x.id}
              label={x.name || t("default-course-instance-name")}
              onChange={(_event) => setInstance(x.id)}
              defaultChecked={instance === x.id}
              // eslint-disable-next-line i18next/no-literal-string
              name="select-course-instance"
            />
          ))}
        </FieldContainer>
        <div
          className={css`
            margin-top: -0.5rem;
            margin-bottom: 1rem;
            color: ${baseTheme.colors.gray[600]};
          `}
        >
          <GreenText>*</GreenText> {t("select-course-instance-explanation")}
        </div>
        {instance !== undefined && additionalQuestions && additionalQuestions.length > 0 && (
          <div
            className={css`
              margin-bottom: 1rem;
            `}
          >
            <>
              <h2>{t("title-additional-questions")}</h2>
              {additionalQuestions.map((additionalQuestion) => {
                if (additionalQuestion.question_type === "Checkbox") {
                  const answer = additionalQuestionAnswers.find(
                    (a) => a.course_background_question_id === additionalQuestion.id,
                  )
                  return (
                    <AdditionalQuestionWrapper key={additionalQuestion.id}>
                      <CheckBox
                        label={additionalQuestion.question_text}
                        checked={answer?.answer_value === "t"}
                        onChange={(value) => {
                          // eslint-disable-next-line i18next/no-literal-string
                          const valueAsString = value ? "t" : "f"
                          setAdditionalQuestionAnswers((prev) => {
                            const newArray = prev.filter(
                              (a) => a.course_background_question_id !== additionalQuestion.id,
                            )
                            newArray.push({
                              answer_value: valueAsString,
                              course_background_question_id: additionalQuestion.id,
                            })
                            return newArray
                          })
                        }}
                      />
                    </AdditionalQuestionWrapper>
                  )
                }
                return (
                  <AdditionalQuestionWrapper key={additionalQuestion.id}>
                    {t("unsupported-question-type")}
                  </AdditionalQuestionWrapper>
                )
              })}
            </>
          </div>
        )}
        {additionalQuestionsQuery.error && (
          <ErrorBanner variant="readOnly" error={additionalQuestionsQuery.error} />
        )}
        <div>
          <Button
            size="medium"
            variant="primary"
            onClick={enrollOnCourse}
            disabled={!instance || additionalQuestionsQuery.isLoading}
          >
            {t("continue")}
          </Button>
        </div>
      </>
    </div>
  )
}

function figureOutInitialValue(
  items: CourseInstance[],
  initialSelectedInstanceId: string | undefined,
): string | undefined {
  if (initialSelectedInstanceId) {
    return initialSelectedInstanceId
  }
  if (items.length === 1) {
    return items[0].id
  }
}

export default SelectCourseInstanceForm
