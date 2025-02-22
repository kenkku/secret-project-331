import styled from "@emotion/styled"
import * as React from "react"
import { useTranslation } from "react-i18next"

import {
  PeerReviewQuestion,
  PeerReviewQuestionSubmission,
} from "../../../../../../shared-module/bindings"

import Essay from "./Essay"
import Likert from "./Likert"
interface ReviewProps {
  orderNumber: number
  review: PeerReviewQuestionSubmission[]
  questions: PeerReviewQuestion[]
}

const Wrapper = styled.div`
  background: #f5f6f7;
  margin-bottom: 10px;
  padding: 0 !important;
`
const Heading = styled.div`
  padding: 1rem;
  border-bottom: 2px solid #ebedee;
`

const PeerReviewQuestionAnswer: React.FunctionComponent<ReviewProps> = ({
  orderNumber,
  review,
  questions,
}) => {
  const { t } = useTranslation()
  // const { id, number_data, text_data, peer_review_question_id } = review

  return (
    <Wrapper>
      <Heading>{`${t("peer-review")} #${orderNumber + 1}`}</Heading>
      {review?.map(({ id, number_data, text_data, peer_review_question_id }, index) => {
        const questionIndex = questions.findIndex((q) => q.id === peer_review_question_id)
        const question = questions[questionIndex].question
        return (
          <>
            {text_data && <Essay key={id} question={question} content={text_data} index={index} />}
            {number_data !== null && (
              <Likert key={id} question={question} content={number_data} index={index} />
            )}
          </>
        )
      })}
    </Wrapper>
  )
}

export default PeerReviewQuestionAnswer
