import { css } from "@emotion/css"
import React, { useState } from "react"
import { useForm } from "react-hook-form"
import { useTranslation } from "react-i18next"

import { CourseInstance, CourseInstanceForm } from "../../../../../../shared-module/bindings"
import Button from "../../../../../../shared-module/components/Button"
import TimePicker from "../../../../../../shared-module/components/InputFields/DateTimeLocal"
import FormField from "../../../../../FormField"

interface FormProps {
  initialData: CourseInstance | null
  onSubmit: (form: CourseInstanceForm) => void
  onCancel: () => void
}

interface Fields {
  name: string
  description: string
  supportEmail: string
  teacherName: string
  teacherEmail: string
}

const NewCourseInstanceForm: React.FC<React.PropsWithChildren<FormProps>> = ({
  initialData,
  onSubmit,
  onCancel,
}) => {
  const { t } = useTranslation()
  const {
    register,
    handleSubmit,
    formState: { errors },
  } = useForm<Fields>()
  const [newOpeningTime, setNewOpeningTime] = useState(initialData?.starts_at || null)
  const [newClosingTime, setNewClosingTime] = useState(initialData?.ends_at || null)
  const onSubmitWrapper = handleSubmit((data) => {
    onSubmit({
      name: data.name || null,
      description: data.description || null,
      support_email: data.supportEmail || null,
      teacher_in_charge_name: data.teacherName,
      teacher_in_charge_email: data.teacherEmail,
      opening_time: newOpeningTime,
      closing_time: newClosingTime,
    })
  })

  return (
    <>
      <form onSubmit={onSubmitWrapper}>
        <FormField
          id={"name"}
          error={errors["name"]}
          defaultValue={initialData?.name}
          placeholder={t("text-field-label-name")}
          register={register}
        />
        <FormField
          id={"description"}
          error={errors["description"]}
          defaultValue={initialData?.description}
          placeholder={t("text-field-label-description")}
          register={register}
        />
        <FormField
          id={"supportEmail"}
          error={errors["supportEmail"]}
          defaultValue={initialData?.support_email}
          placeholder={t("support-email")}
          register={register}
        />
        <FormField
          id={"teacherName"}
          error={errors["teacherName"]}
          defaultValue={initialData?.teacher_in_charge_name}
          placeholder={t("teacher-in-charge-name")}
          register={register}
        />
        <FormField
          id={"teacherEmail"}
          error={errors["teacherEmail"]}
          defaultValue={initialData?.teacher_in_charge_email}
          placeholder={t("teacher-in-charge-email")}
          register={register}
        />
        <TimePicker
          label={t("opening-time")}
          onChange={(time) => setNewOpeningTime(new Date(time))}
          className={css`
            margin-bottom: 0.5rem;
          `}
        />
        <TimePicker
          label={t("closing-time")}
          onChange={(time) => setNewClosingTime(new Date(time))}
          className={css`
            margin-bottom: 0.5rem;
          `}
        />
        <Button variant="primary" size="medium" type="submit" value={t("button-text-submit")}>
          {t("button-text-submit")}
        </Button>
        <Button variant="secondary" size="medium" onClick={onCancel}>
          {t("button-text-cancel")}
        </Button>
      </form>
    </>
  )
}

export default NewCourseInstanceForm
