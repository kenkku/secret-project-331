import styled from "@emotion/styled"
import React from "react"
import { FieldError, FieldPath, FieldValues, UseFormRegister } from "react-hook-form"
import { useTranslation } from "react-i18next"

interface Props<T extends FieldValues> {
  id: FieldPath<T>
  error: FieldError | undefined
  defaultValue: string | null | undefined
  placeholder: string
  register: UseFormRegister<T>
  required?: boolean
  type?: string
  value?: string
  className?: string
  errorMessage?: string
}

const ErrorText = styled.p`
  color: red;
`

const FormTextAreaField = <T extends FieldValues>({
  id,
  defaultValue,
  error,
  register,
  required,
  placeholder,
  errorMessage,
  ...rest
}: Props<T>): React.ReactElement => {
  const { t } = useTranslation()
  return (
    <>
      <label htmlFor={id}>{placeholder}</label>
      <br />
      {required && error && (
        <>
          <span>{t("this-field-required")}</span>
          <br />
        </>
      )}
      <textarea
        id={id}
        placeholder={placeholder}
        defaultValue={defaultValue || ""}
        {...register(id, { required: required })}
        {...rest}
      ></textarea>
      <br />
      {errorMessage && <ErrorText> {errorMessage} </ErrorText>}
    </>
  )
}

export default FormTextAreaField
