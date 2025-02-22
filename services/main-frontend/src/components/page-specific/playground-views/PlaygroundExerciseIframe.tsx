import { css } from "@emotion/css"
import { UseQueryResult } from "@tanstack/react-query"
import { useTranslation } from "react-i18next"

import MessageChannelIFrame from "../../../shared-module/components/MessageChannelIFrame"
import {
  CurrentStateMessage,
  IframeState,
  UserInformation,
} from "../../../shared-module/exercise-service-protocol-types"
import { isMessageFromIframe } from "../../../shared-module/exercise-service-protocol-types.guard"
import { onUploadFileMessage } from "../../../shared-module/utils/exerciseServices"

interface PlaygroundExerciseIframeProps {
  url: string
  publicSpecQuery: UseQueryResult<unknown>
  userAnswer: unknown
  setCurrentStateReceivedFromIframe: React.Dispatch<
    React.SetStateAction<CurrentStateMessage | null>
  >
  showIframeBorders: boolean
  disableSandbox: boolean
  userInformation: UserInformation
}

const EXAMPLE_UUID = "886d57ba-4c88-4d88-9057-5e88f35ae25f"
const TITLE = "PLAYGROUND"

const PlaygroundExerciseIframe: React.FC<
  React.PropsWithChildren<PlaygroundExerciseIframeProps>
> = ({
  url,
  publicSpecQuery,
  setCurrentStateReceivedFromIframe,
  showIframeBorders,
  disableSandbox,
  userInformation,
  userAnswer,
}) => {
  const { t } = useTranslation()
  if (publicSpecQuery.isLoading || publicSpecQuery.isError) {
    return <>{t("error-no-public-spec")}</>
  }
  // Makes sure the iframe renders again when the data changes
  const iframeKey =
    url +
    JSON.stringify(publicSpecQuery.data) +
    disableSandbox +
    JSON.stringify(userAnswer) +
    JSON.stringify(userInformation)
  return (
    <div
      className={css`
        margin-top: 1rem;
      `}
    >
      <MessageChannelIFrame
        key={iframeKey}
        url={url}
        postThisStateToIFrame={
          {
            // eslint-disable-next-line i18next/no-literal-string
            view_type: "answer-exercise",
            exercise_task_id: EXAMPLE_UUID,
            user_information: userInformation,
            data: {
              public_spec: publicSpecQuery.data,
              previous_submission: userAnswer,
            },
          } as IframeState
        }
        onMessageFromIframe={async (msg, responsePort) => {
          if (isMessageFromIframe(msg)) {
            if (msg.message === "current-state") {
              setCurrentStateReceivedFromIframe(msg)
            } else if (msg.message === "file-upload") {
              // eslint-disable-next-line i18next/no-literal-string
              await onUploadFileMessage("playground", msg.files, responsePort)
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

export default PlaygroundExerciseIframe
