import { Alert } from "@mui/lab"
import React from "react"
import { useTranslation } from "react-i18next"

import MessageChannelIFrame from "../../../../shared-module/components/MessageChannelIFrame"
import { IframeState } from "../../../../shared-module/exercise-service-protocol-types"
import { isMessageFromIframe } from "../../../../shared-module/exercise-service-protocol-types.guard"
import { onUploadFileMessage } from "../../../../shared-module/utils/exerciseServices"

interface ExerciseTaskIframeProps {
  exerciseServiceSlug: string
  url: string
  postThisStateToIFrame: IframeState | null
  setAnswer: ((answer: { valid: boolean; data: unknown }) => void) | null
  title: string
}

const ExerciseTaskIframe: React.FC<React.PropsWithChildren<ExerciseTaskIframeProps>> = ({
  exerciseServiceSlug,
  url,
  postThisStateToIFrame,
  setAnswer,
  title,
}) => {
  const { t } = useTranslation()
  if (!url || url.trim() === "") {
    return <Alert severity="error">{t("cannot-render-exercise-task-missing-url")}</Alert>
  }

  return (
    <MessageChannelIFrame
      url={url}
      postThisStateToIFrame={postThisStateToIFrame}
      onMessageFromIframe={async (messageContainer, responsePort) => {
        console.log(messageContainer)
        if (isMessageFromIframe(messageContainer)) {
          if (messageContainer.message === "current-state") {
            const { data, valid } = messageContainer
            if (setAnswer) {
              setAnswer({ data, valid })
            }
          } else if (messageContainer.message === "file-upload") {
            await onUploadFileMessage(exerciseServiceSlug, messageContainer.files, responsePort)
          }
        }
      }}
      title={title}
    />
  )
}

export default ExerciseTaskIframe
