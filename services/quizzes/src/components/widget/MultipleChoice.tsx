import { css, cx } from "@emotion/css"
import _ from "lodash"
import React from "react"
import { useTranslation } from "react-i18next"

import { QuizItemAnswer } from "../../../types/types"
import { baseTheme } from "../../shared-module/styles"
import { respondToOrLarger } from "../../shared-module/styles/respond"
import { sanitizeFlexDirection } from "../../shared-module/utils/css-sanitization"
import withErrorBoundary from "../../shared-module/utils/withErrorBoundary"
import { quizTheme } from "../../styles/QuizStyles"
import { COLUMN, ROW } from "../../util/constants"
import { orderArrayWithId } from "../../util/randomizer"
import ParsedText from "../ParsedText"

import { QuizItemComponentProps } from "."

const optionButton = css`
  align-items: center;
  border: none;
  display: flex;
  flex: 1;
  justify-content: center;
  margin: 0.3rem 0.3rem 0.3rem 0;
  padding: 1rem;
  transition: background-color 0.2s;
  text-align: left;
`

const optionButtonColumn = css`
  ${respondToOrLarger.xs} {
    justify-content: left;
  }
`

// eslint-disable-next-line i18next/no-literal-string
const optionButtonSelected = css`
  background: ${quizTheme.selectedItemBackground};
  color: ${quizTheme.selectedItemColor};
`

export interface LeftBorderedDivProps {
  correct: boolean | undefined
  direction?: string
  message?: string
}

const MultipleChoice: React.FunctionComponent<QuizItemComponentProps> = ({
  quizItemAnswerState,
  quizItem,
  user_information,
  setQuizItemAnswerState,
}) => {
  const { t } = useTranslation()
  // Column means that all the options are always diplayed on top of each other, regardless of the
  // device width.
  const direction = sanitizeFlexDirection(quizItem.direction, ROW)

  const handleOptionSelect = (event: React.MouseEvent<HTMLButtonElement>) => {
    if (!quizItemAnswerState) {
      return
    }

    const selectedOptionId = event.currentTarget.value
    let newItemAnswer: QuizItemAnswer
    // multi is set to true then student can select multiple options for an answer
    if (quizItem.multi) {
      const optionAnswers = _.xor(quizItemAnswerState.optionAnswers, [selectedOptionId])
      newItemAnswer = {
        ...quizItemAnswerState,
        optionAnswers,
        valid: optionAnswers.length > 0,
      }
    } else {
      newItemAnswer = {
        ...quizItemAnswerState,
        optionAnswers: [selectedOptionId],
        valid: true,
      }
    }
    setQuizItemAnswerState(newItemAnswer)
  }

  let quiz_options = quizItem.options
  if (quizItem.shuffleOptions) {
    quiz_options = orderArrayWithId(quiz_options, user_information.pseudonymous_id)
  }

  return (
    <div
      className={css`
        flex: 1;
        margin: 0.7rem 0;
      `}
    >
      <div
        className={css`
          /* font-size: ${quizTheme.quizTitleFontSize}; */
          font-weight: bold;
          font-family: "Raleway", sans-serif;
          font-size: clamp(18px, 2vw, 20px) !important;
        `}
      >
        <ParsedText parseLatex parseMarkdown inline text={quizItem.title} />
      </div>
      <p
        className={css`
          color: ${quizTheme.quizBodyColor};
          font-size: ${quizTheme.quizBodyFontSize};
          margin: 0.5rem 0;
        `}
      >
        <ParsedText parseLatex parseMarkdown inline text={quizItem.body} />
      </p>
      <div
        className={css`
          display: flex;
          flex-direction: column;

          ${respondToOrLarger.sm} {
            flex-direction: ${direction};
          }
        `}
      >
        {quiz_options.map((qo) => {
          const selected = quizItemAnswerState?.optionAnswers?.includes(qo.id)

          return (
            <button
              key={qo.id}
              value={qo.id}
              onClick={handleOptionSelect}
              aria-pressed={selected ?? false}
              className={cx(
                optionButton,
                selected && optionButtonSelected,
                direction === COLUMN && optionButtonColumn,
              )}
            >
              <ParsedText parseMarkdown parseLatex inline text={qo.title || qo.body || ""} />
            </button>
          )
        })}
      </div>
      {quizItem.multi && (
        <div
          className={css`
            font-size: 13px;
            color: ${baseTheme.colors.gray[500]};
            margin: 0.3rem auto;
            width: fit-content;
          `}
        >
          {t("select-all-correct-options")}
        </div>
      )}
    </div>
  )
}

export default withErrorBoundary(MultipleChoice)
