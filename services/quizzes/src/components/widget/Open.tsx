import React, { useId, useState } from "react"
import { useTranslation } from "react-i18next"

import TextField from "../../shared-module/components/InputFields/TextField"
import { stripNonPrintableCharacters } from "../../shared-module/utils/strings"
import withErrorBoundary from "../../shared-module/utils/withErrorBoundary"
import MarkdownText from "../MarkdownText"
import CloseEndedQuestionWrapper from "../Shared/CloseEndedQuestionWrapper"

import { QuizItemComponentProps } from "."

const Open: React.FC<QuizItemComponentProps> = ({
  quizDirection,
  quizItem,
  quizItemAnswerState,
  setQuizItemAnswerState,
}) => {
  const { t } = useTranslation()
  const fieldId = useId()
  const [showFormatError, setShowFormatError] = useState(false)

  const handleChange = (newValue: string) => {
    if (!quizItemAnswerState) {
      return
    }

    if (!quizItem.formatRegex) {
      return setQuizItemAnswerState({ ...quizItemAnswerState, textData: newValue, valid: true })
    }

    const newValueIsValid = newValue
      ? answerFormatIsValidAgainstRegex(newValue, quizItem.formatRegex)
      : true
    setQuizItemAnswerState({ ...quizItemAnswerState, textData: newValue, valid: newValueIsValid })
  }

  const formatErrorVisible =
    showFormatError && quizItemAnswerState?.textData && !quizItemAnswerState?.valid

  return (
    <CloseEndedQuestionWrapper wideScreenDirection={quizDirection}>
      <div>{quizItem.title && <MarkdownText text={quizItem.title} />}</div>
      <div>{quizItem.body && <MarkdownText text={quizItem.body} />}</div>
      <div>
        <TextField
          id={fieldId}
          aria-label={t("answer")}
          label={t("answer")}
          type="text"
          value={quizItemAnswerState?.textData ?? ""}
          onChange={(e) => handleChange(e)}
          onFocus={() => setShowFormatError(true)}
          onBlur={() => setShowFormatError(false)}
          error={
            formatErrorVisible
              ? t("error-answer-does-not-match-the-specified-answer-format")
              : undefined
          }
        />
      </div>
    </CloseEndedQuestionWrapper>
  )
}

const answerFormatIsValidAgainstRegex = (answer: string, validatorRegex: string): boolean => {
  const cleanedInput = stripNonPrintableCharacters(answer)
  // eslint-disable-next-line i18next/no-literal-string
  const validator = new RegExp(validatorRegex.trim(), "i")
  return validator.test(cleanedInput)
}

export default withErrorBoundary(Open)
