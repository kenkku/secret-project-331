import { Alert } from "@mui/lab"
import React, { useContext } from "react"
import { useTranslation } from "react-i18next"
import { useMemoOne } from "use-memo-one"
import { v5 } from "uuid"

import { SIDEBAR_WIDTH_PX } from "../../components/Layout"
import CourseContext from "../../contexts/CourseContext"
import MessageChannelIFrame from "../../shared-module/components/MessageChannelIFrame"
import Spinner from "../../shared-module/components/Spinner"
import LoginStateContext from "../../shared-module/contexts/LoginStateContext"
import { IframeState } from "../../shared-module/exercise-service-protocol-types"
import { isMessageFromIframe } from "../../shared-module/exercise-service-protocol-types.guard"
import useMedia from "../../shared-module/hooks/useMedia"
import useUserInfo from "../../shared-module/hooks/useUserInfo"
import { respondToOrLarger } from "../../shared-module/styles/respond"
import { onUploadFileMessage } from "../../shared-module/utils/exerciseServices"
import getGuestPseudonymousUserId from "../../shared-module/utils/getGuestPseudonymousUserId"
import withNoSsr from "../../shared-module/utils/withNoSsr"

const VIEW_TYPE = "exercise-editor"
const UNEXPECTED_MESSAGE_ERROR = "Unexpected message or structure is not valid."
const IFRAME_EDITOR = "IFRAME EDITOR"

interface ExerciseTaskIFrameEditorProps {
  exerciseServiceSlug: string
  exerciseTaskId: string
  onPrivateSpecChange(newSpec: string): void
  privateSpec: string | null
  url: string | null | undefined
}

const ExerciseTaskIFrameEditor: React.FC<
  React.PropsWithChildren<ExerciseTaskIFrameEditorProps>
> = ({ exerciseServiceSlug, exerciseTaskId, onPrivateSpecChange, privateSpec, url }) => {
  const { t } = useTranslation()
  const loginStateContext = useContext(LoginStateContext)
  const userInfo = useUserInfo()
  const userId = userInfo.data?.user_id || getGuestPseudonymousUserId()
  const courseContext = useContext(CourseContext)

  const largeScreen = useMedia(respondToOrLarger.xl)

  const postThisStateToIFrame: IframeState = useMemoOne(() => {
    return {
      view_type: VIEW_TYPE,
      exercise_task_id: exerciseTaskId,
      user_information: {
        pseudonymous_id: v5(courseContext?.courseId ?? "", userId) ?? getGuestPseudonymousUserId(),
        signed_in: Boolean(loginStateContext.signedIn),
      },
      data: {
        private_spec:
          privateSpec === null || privateSpec === undefined ? null : JSON.parse(privateSpec),
      },
    }
  }, [privateSpec])

  if (!url || url.trim() === "") {
    return <Alert severity="error">{t("error-cannot-render-exercise-task-missing-url")}</Alert>
  }

  if (!userInfo.data) {
    return <Spinner variant="medium" />
  }

  return (
    <MessageChannelIFrame
      url={url}
      postThisStateToIFrame={postThisStateToIFrame}
      onMessageFromIframe={async (messageContainer, responsePort) => {
        if (isMessageFromIframe(messageContainer)) {
          if (messageContainer.message === "current-state") {
            // eslint-disable-next-line @typescript-eslint/no-explicit-any
            onPrivateSpecChange(JSON.stringify((messageContainer.data as any).private_spec))
          } else if (messageContainer.message === "file-upload") {
            await onUploadFileMessage(exerciseServiceSlug, messageContainer.files, responsePort)
          }
        } else {
          console.error(UNEXPECTED_MESSAGE_ERROR)
        }
      }}
      breakFromCenteredProps={
        largeScreen
          ? {
              sidebar: true,
              // eslint-disable-next-line i18next/no-literal-string
              sidebarWidth: `${SIDEBAR_WIDTH_PX}px`,
              // eslint-disable-next-line i18next/no-literal-string
              sidebarPosition: "right",
            }
          : undefined
      }
      title={IFRAME_EDITOR}
    />
  )
}

// withNoSsr used here because this component uses the useMedia hook and if we accidentally rendered this on the server, we could get rehydation mismatches which could break react rendering
export default withNoSsr(ExerciseTaskIFrameEditor)
