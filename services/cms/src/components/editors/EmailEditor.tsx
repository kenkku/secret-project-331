import SaveIcon from "@mui/icons-material/Save"
import LoadingButton from "@mui/lab/LoadingButton"
import { BlockInstance } from "@wordpress/blocks"
import dynamic from "next/dynamic"
import React, { useContext, useState } from "react"
import { useTranslation } from "react-i18next"

import { allowedEmailCoreBlocks } from "../../blocks/supportedGutenbergBlocks"
import CourseContext from "../../contexts/CourseContext"
import mediaUploadBuilder from "../../services/backend/media/mediaUpload"
import { EmailTemplate, EmailTemplateUpdate } from "../../shared-module/bindings"
import Spinner from "../../shared-module/components/Spinner"
import { modifyBlocks } from "../../utils/Gutenberg/modifyBlocks"
import { removeUnsupportedBlockType } from "../../utils/Gutenberg/removeUnsupportedBlockType"
import UpdateEmailDetailsForm from "../forms/UpdateEmailDetailsForm"

interface EmailEditorProps {
  data: EmailTemplate
  handleSave: (updatedTemplate: EmailTemplateUpdate) => Promise<EmailTemplate>
  needToRunMigrationsAndValidations: boolean
  setNeedToRunMigrationsAndValidations: React.Dispatch<boolean>
}

const EditorLoading = <Spinner variant="medium" />

const EmailGutenbergEditor = dynamic(() => import("./GutenbergEditor"), {
  ssr: false,
  loading: () => EditorLoading,
})

const EmailEditor: React.FC<React.PropsWithChildren<EmailEditorProps>> = ({
  data,
  handleSave,
  needToRunMigrationsAndValidations,
  setNeedToRunMigrationsAndValidations,
}) => {
  const courseId = useContext(CourseContext)?.courseId
  const { t } = useTranslation()
  const [content, setContent] = useState<BlockInstance[]>(
    modifyBlocks(
      (data.content ?? []) as BlockInstance[],
      allowedEmailCoreBlocks,
    ) as BlockInstance[],
  )
  const [name, setName] = useState(data.name)
  const [subject, setSubject] = useState(data.subject ?? "")
  const [saving, setSaving] = useState(false)
  const [error, setError] = useState<string | null>(null)

  const handleOnSave = async () => {
    setSaving(true)
    try {
      const res = await handleSave({
        subject,
        name,
        content: removeUnsupportedBlockType(content),
        exercise_completions_threshold: null,
        points_threshold: null,
      })
      setContent(res.content as BlockInstance[])
      setName(res.name)
      setError(null)
      setSubject(res.subject ?? "")
    } catch (e: unknown) {
      if (!(e instanceof Error)) {
        throw e
      }
      setError(e.toString())
    } finally {
      setSaving(false)
    }
  }

  return (
    <>
      <div className="editor__component">
        <div>
          {error && <pre>{error}</pre>}
          <LoadingButton
            // eslint-disable-next-line i18next/no-literal-string
            loadingPosition="start"
            startIcon={<SaveIcon />}
            loading={saving}
            onClick={handleOnSave}
          >
            {t("save")}
          </LoadingButton>

          <UpdateEmailDetailsForm
            name={name}
            subject={subject}
            setName={setName}
            setSubject={setSubject}
          />
        </div>
      </div>

      {courseId && (
        <EmailGutenbergEditor
          content={content}
          onContentChange={setContent}
          allowedBlocks={allowedEmailCoreBlocks}
          mediaUpload={mediaUploadBuilder({ courseId: courseId })}
          needToRunMigrationsAndValidations={needToRunMigrationsAndValidations}
          setNeedToRunMigrationsAndValidations={setNeedToRunMigrationsAndValidations}
        />
      )}
    </>
  )
}
export default EmailEditor
