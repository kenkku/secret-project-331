/* eslint-disable i18next/no-literal-string */
import { css } from "@emotion/css"
import _ from "lodash"
import { useRouter } from "next/router"
import React, { useState } from "react"
import ReactDOM from "react-dom"

import StateRenderer from "../components/StateRenderer"
import HeightTrackingContainer from "../shared-module/components/HeightTrackingContainer"
import { isMessageToIframe } from "../shared-module/exercise-service-protocol-types.guard"
import useExerciseServiceParentConnection from "../shared-module/hooks/useExerciseServiceParentConnection"
import withErrorBoundary from "../shared-module/utils/withErrorBoundary"
import {
  EditorExercisePublicSpec,
  IframeState,
  ModelSolutionSpec,
  PrivateSpec,
  PublicSpec,
  UserAnswer,
} from "../util/stateInterfaces"

const Iframe: React.FC<React.PropsWithChildren<unknown>> = () => {
  const [state, setState] = useState<IframeState | null>(null)
  const router = useRouter()
  const rawMaxWidth = router?.query?.width
  let maxWidth: number | null = 500
  if (rawMaxWidth) {
    maxWidth = Number(rawMaxWidth)
  }

  const port = useExerciseServiceParentConnection((messageData) => {
    if (isMessageToIframe(messageData)) {
      if (messageData.message === "set-state") {
        ReactDOM.flushSync(() => {
          if (messageData.view_type === "exercise-editor") {
            setState({
              viewType: messageData.view_type,
              exerciseTaskId: messageData.exercise_task_id,
              repositoryExercises: messageData.repository_exercises || null,
              privateSpec: messageData.data.private_spec as PrivateSpec,
            })
          } else if (messageData.view_type === "answer-exercise") {
            setState((oldState) => {
              const newPublicSpec = messageData.data.public_spec as EditorExercisePublicSpec
              return {
                viewType: messageData.view_type,
                initialPublicSpec:
                  oldState?.viewType === "answer-exercise"
                    ? oldState.initialPublicSpec
                    : // cloneDeep prevents setState from changing the initial spec
                      _.cloneDeep(newPublicSpec),
                publicSpec: newPublicSpec,
                previousSubmission: null, // todo messageData.data.previous_submission,
              }
            })
          } else if (messageData.view_type === "view-submission") {
            setState({
              viewType: messageData.view_type,
              exerciseTaskId: messageData.exercise_task_id,
              grading: messageData.data.grading,
              userAnswer: messageData.data.user_answer as UserAnswer,
              publicSpec: messageData.data.public_spec as PublicSpec,
              modelSolutionSpec: messageData.data.model_solution_spec as ModelSolutionSpec,
            })
          } else {
            // eslint-disable-next-line i18next/no-literal-string
            console.error("Unknown view type received from parent")
          }
        })
      } else {
        // eslint-disable-next-line i18next/no-literal-string
        console.error("Unexpected message from parent")
      }
    } else {
      // eslint-disable-next-line i18next/no-literal-string
      console.error("Frame received an unknown message from message port")
    }
  })

  return (
    <HeightTrackingContainer port={port}>
      <div
        className={css`
          width: 100%;
          ${maxWidth && `max-width: ${maxWidth}px;`}
          margin: 0 auto;
        `}
      >
        <StateRenderer port={port} setState={setState} state={state} />
      </div>
    </HeightTrackingContainer>
  )
}

export default withErrorBoundary(Iframe)
