import { keyframes } from "@emotion/css"
import styled from "@emotion/styled"
import { useQuery } from "@tanstack/react-query"
import { groupBy } from "lodash"
import * as React from "react"
import { useTranslation } from "react-i18next"

import { fetchPeerReviewDataReceivedByExerciseId } from "../../../../../../services/backend"
import {
  PeerReviewQuestion,
  PeerReviewQuestionSubmission,
} from "../../../../../../shared-module/bindings"
import ErrorBanner from "../../../../../../shared-module/components/ErrorBanner"
import Spinner from "../../../../../../shared-module/components/Spinner"
import { baseTheme, headingFont } from "../../../../../../shared-module/styles"

import PeerReviewQuestionAnswer from "./PeerReviewQuestionAnswer"

const openAnimation = keyframes`
  0% { opacity: 0; }
  100% { opacity: 1; }
`

const slideDown = keyframes`
  from { opacity: 0; height: 0; padding: 0;}
  to { opacity: 1; height: 100%; padding: 10px;}
`

// eslint-disable-next-line i18next/no-literal-string
const Wrapper = styled.div`
  details {
    transition: all 0.3s ease-in-out;
  }

  details[open] summary ~ * {
    animation: ${openAnimation} 0.3s ease-in-out;
    color: ${baseTheme.colors.gray[700]};
  }

  details[open] > div {
    animation-name: ${slideDown};
    animation-duration: 0.3s;
    animation-fill-mode: forwards;
    padding: 1rem 1rem 1rem 1rem;
  }

  details summary {
    padding: 1rem;
    display: flex;
    align-items: center;
    position: relative;
    cursor: pointer;
    font-weight: 500;
    font-family: ${headingFont};
    list-style: none;
    color: ${baseTheme.colors.gray[700]};
    outline: 0;
    background: ${baseTheme.colors.clear[100]};
    margin-bottom: 5px;
  }

  details summary::-webkit-details-marker {
    display: none;
  }

  details[open] > summary {
    color: ${baseTheme.colors.gray[700]};
  }

  details summary:after {
    content: "+";
    position: absolute;
    font-size: 2.4rem;
    color: ${baseTheme.colors.gray[700]};
    line-height: 0;
    margin-top: 0.75rem;
    top: 14px;
    right: 4%;
    font-weight: 200;
    transform-origin: center;
    transition: all 200ms linear;
  }
  details[open] summary:after {
    transform: rotate(45deg);
    font-size: 2.4rem;
  }
  details[open] summary {
    opacity: 0.9;
  }
`
const Notification = styled.div`
  display: inline-block;
  width: 20px;
  height: 20px;
  background: #44827e;
  color: #fff;
  line-height: 113%;
  font-size: 15px;
  font-family: ${headingFont};
  border-radius: 50%;
  text-align: center;
  margin-left: 0.5rem;
  border: 1px solid ${baseTheme.colors.green[100]};
`

interface PeerReviewProps {
  id: string
  submissionId: string
}

const PeerReview: React.FunctionComponent<PeerReviewProps> = ({ id, submissionId }) => {
  const { t } = useTranslation()
  let result: PeerReviewQuestionSubmission[] = []
  let questions: PeerReviewQuestion[] = []

  const getPeerReviewReceived = useQuery(
    [`exercise-${id}-exercise-slide-submission-${submissionId}-peer-reviews-received`],
    () => fetchPeerReviewDataReceivedByExerciseId(id, submissionId),
  )

  if (getPeerReviewReceived.isLoading) {
    return <Spinner variant={"medium"} />
  }

  if (getPeerReviewReceived.isError) {
    return <ErrorBanner variant={"readOnly"} error={getPeerReviewReceived.error} />
  }

  if (
    getPeerReviewReceived.isSuccess &&
    getPeerReviewReceived.data.peer_review_question_submissions.length > 0
  ) {
    const { peer_review_questions, peer_review_question_submissions } = getPeerReviewReceived.data
    result = peer_review_question_submissions
    questions = peer_review_questions
  }

  const ordered = result.sort((a, b) => b.created_at.getTime() - a.created_at.getTime())

  const groupByPeerReviewSubmissionId = groupBy(
    ordered,
    (review) => review.peer_review_submission_id,
  )

  return (
    <Wrapper>
      <details>
        <summary>
          {t("peer-reviews-received-from-other-student")}
          <Notification>{Object.keys(groupByPeerReviewSubmissionId).length ?? "0"}</Notification>
        </summary>
        {Object.values(groupByPeerReviewSubmissionId).map((item, index) => (
          <PeerReviewQuestionAnswer
            orderNumber={index}
            key={index}
            review={item}
            questions={questions}
          />
        ))}
      </details>
    </Wrapper>
  )
}

export default PeerReview
