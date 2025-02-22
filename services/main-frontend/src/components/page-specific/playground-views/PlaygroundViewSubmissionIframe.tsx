import { css } from "@emotion/css"
import { UseMutationResult, UseQueryResult } from "@tanstack/react-query"
import { useTranslation } from "react-i18next"

import { ExerciseTaskGradingResult } from "../../../shared-module/bindings"
import MessageChannelIFrame from "../../../shared-module/components/MessageChannelIFrame"
import {
  CurrentStateMessage,
  IframeState,
  UserInformation,
} from "../../../shared-module/exercise-service-protocol-types"
import { isMessageFromIframe } from "../../../shared-module/exercise-service-protocol-types.guard"

interface PlaygroundViewSubmissionIframeProps {
  url: string
  publicSpecQuery: UseQueryResult<unknown>
  gradingQuery: UseMutationResult<ExerciseTaskGradingResult>
  modelSolutionSpecQuery: UseQueryResult<unknown>
  userAnswer: unknown
  setCurrentStateReceivedFromIframe: React.Dispatch<
    React.SetStateAction<CurrentStateMessage | null>
  >
  showIframeBorders: boolean
  sendModelsolutionSpec: boolean
  disableSandbox: boolean
  userInformation: UserInformation
}

const EXAMPLE_UUID = "886d57ba-4c88-4d88-9057-5e88f35ae25f"
const TITLE = "PLAYGROUND"

const PlaygroundViewSubmissionIframe: React.FC<
  React.PropsWithChildren<PlaygroundViewSubmissionIframeProps>
> = ({
  url,
  publicSpecQuery,
  gradingQuery,
  modelSolutionSpecQuery,
  setCurrentStateReceivedFromIframe,
  showIframeBorders,
  userAnswer,
  sendModelsolutionSpec,
  disableSandbox,
  userInformation,
}) => {
  const { t } = useTranslation()
  if (publicSpecQuery.isLoading || publicSpecQuery.isError) {
    return <>{t("error-no-public-spec")}</>
  }
  if (modelSolutionSpecQuery.isLoading || modelSolutionSpecQuery.isError) {
    return <>{t("error-no-model-solution-spec")}</>
  }
  if (gradingQuery.isLoading || gradingQuery.isError) {
    return <>{t("error-no-grading")}</>
  }
  const iframeState: IframeState = {
    // eslint-disable-next-line i18next/no-literal-string
    view_type: "view-submission",
    exercise_task_id: EXAMPLE_UUID,
    user_information: userInformation,
    data: {
      grading: gradingQuery.data ?? null,
      user_answer: userAnswer,
      public_spec: publicSpecQuery.data,
      model_solution_spec: sendModelsolutionSpec ? modelSolutionSpecQuery.data : null,
    },
  }
  // Makes sure the iframe renders again when the data changes
  const iframeKey = url + JSON.stringify(iframeState) + disableSandbox
  return (
    <div
      className={css`
        margin-top: 1rem;
      `}
    >
      <MessageChannelIFrame
        key={iframeKey}
        url={url}
        postThisStateToIFrame={iframeState}
        onMessageFromIframe={async (msg) => {
          if (isMessageFromIframe(msg)) {
            if (msg.message === "current-state") {
              // eslint-disable-next-line @typescript-eslint/no-explicit-any
              setCurrentStateReceivedFromIframe(msg)
            } else if (msg.message === "file-upload") {
              // cannot upload from submission view
            }
          }
        }}
        title={TITLE}
        showBorders={showIframeBorders}
        disableSandbox={disableSandbox}
      />
    </div>
  )
}

export default PlaygroundViewSubmissionIframe
