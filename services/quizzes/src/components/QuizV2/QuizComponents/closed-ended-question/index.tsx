import styled from "@emotion/styled"
import { faPlus } from "@fortawesome/free-solid-svg-icons"
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome"
import React, { useState } from "react"
import { useTranslation } from "react-i18next"

import { PrivateSpecQuizItemClosedEndedQuestion } from "../../../../../types/quizTypes"
import Accordion from "../../../../shared-module/components/Accordion"
import RadioButton from "../../../../shared-module/components/InputFields/RadioButton"
import SelectField from "../../../../shared-module/components/InputFields/SelectField"
import TextField from "../../../../shared-module/components/InputFields/TextField"
import { primaryFont } from "../../../../shared-module/styles"
import EditorCard from "../common/EditorCard"

interface ClosedEndedQuestionEditorProps {
  quizItem: PrivateSpecQuizItemClosedEndedQuestion
}

interface TestTableProps {
  testStrings: string[]
  quizItem: PrivateSpecQuizItemClosedEndedQuestion
}

const OptionTitle = styled.div`
  font-size: 20px;
  font-family: ${primaryFont};
  font-weight: bold;
`

const RadioButtonContainer = styled.div`
  display: flex;
  flex-direction: row;
  gap: 4px;
`

const convertToString = (regexInput: string | null) => {
  return regexInput ? regexInput : ""
}

const REGEX_PATTERNS = [
  {
    label: "String",
    value: "\\S+",
  },
  {
    label: "Date (mm/dd/YYYY)",
    value: "d{2}\\/\\d{2}\\/\\d{4}",
  },
  {
    label: "Date (YYYY-mm-dd)",
    value: "d{2}\\/\\d{2}\\/\\d{4}",
  },
  {
    label: "Date (dd.mm.YYYY)",
    value: "d{2}\\/\\d{2}\\/\\d{4}",
  },
  {
    label: "Whole number",
    value: "\\d+",
  },
  {
    label: "Decimal",
    value: "\\d+\\,\\d+",
  },
]

const RegexMethodView: React.FC<ClosedEndedQuestionEditorProps> = ({ quizItem }) => {
  const { t } = useTranslation()

  return (
    <>
      <TextField
        value={convertToString(quizItem.validityRegex)}
        label={t("validity-regular-expression")}
        name={t("validity-regular-expression")}
      />
      <TextField
        value={convertToString(quizItem.formatRegex)}
        label={t("format-regular-expression")}
        name={t("format-regular-expression")}
      />
    </>
  )
}

const TestButtonContainer = styled.div`
  * {
    margin: 0px;
    margin-bottom: 8px;
  }
`

const RegexTestTableContainer = styled.div`
  display: flex;
`

const TestTable = styled.table`
  margin-left: auto;
  margin-right: auto;
`

const RegexTableHeaderCell = styled.th`
  background-color: #f9f9f9;
  padding: 8px;
`

const RegexTableStringCell = styled.td`
  background-color: #f9f9f9;
  padding: 6px;
  min-width: 150px;
`

const RegexTableCorrectCell = styled.td`
  background-color: #dae6e5;
  color: #065853;
  padding: 4px;
  text-align: center;
  text-transform: uppercase;
`

const RegexTableFailedCell = styled.td`
  background-color: #f0e1dd;
  color: #a84835;
  padding: 4px;
  text-align: center;
  text-transform: uppercase;
`

const CircleButton = styled(FontAwesomeIcon)`
  background-color: #dae6e5;
  height: 12px;
  width: 12px;
  padding: 4px;
  display: inline;
  border-radius: 50%;
  cursor: pointer;
  margin-left: 2px;

  :hover {
    background-color: #bcd1d0;
  }
`

const AddNewRowContainer = styled.div`
  display: flex;
  flex-direction: row;
  align-items: center;
  gap: 6px;
`

const RegexTestTable: React.FC<TestTableProps> = ({ quizItem, testStrings }) => {
  const { t } = useTranslation()

  const validateStrings = () => {
    const validationRegExp = new RegExp(convertToString(quizItem.validityRegex))
    const formatRegExp = new RegExp(convertToString(quizItem.formatRegex))

    return testStrings.map((string) => {
      return {
        string,
        validation: validationRegExp.test(string),
        format: formatRegExp.test(string),
      }
    })
  }

  const result = validateStrings()

  return (
    <TestTable>
      <tr key={`test-table-headers`}>
        <RegexTableHeaderCell> {t("string")} </RegexTableHeaderCell>
        <RegexTableHeaderCell> {t("format")} </RegexTableHeaderCell>
        <RegexTableHeaderCell> {t("validation")} </RegexTableHeaderCell>
      </tr>
      {result.map((result, idx) => (
        <tr key={`test-table-row-${idx}`}>
          <RegexTableStringCell> {result.string} </RegexTableStringCell>
          {result.format ? (
            <RegexTableCorrectCell> {t("passed")} </RegexTableCorrectCell>
          ) : (
            <RegexTableFailedCell> {t("failed")} </RegexTableFailedCell>
          )}
          {result.validation ? (
            <RegexTableCorrectCell> {t("passed")} </RegexTableCorrectCell>
          ) : (
            <RegexTableFailedCell> {t("failed")} </RegexTableFailedCell>
          )}
        </tr>
      ))}
    </TestTable>
  )
}

const ExactStringMethodView: React.FC<ClosedEndedQuestionEditorProps> = ({ quizItem }) => {
  const { t } = useTranslation()

  return (
    <>
      <SelectField
        id="regex-pattern-select"
        label={t("format-regular-expression")}
        options={REGEX_PATTERNS}
      />
      <TextField
        value={convertToString(quizItem.validityRegex)}
        label={t("correct-answer")}
        name={t("correct-answer")}
      />
    </>
  )
}

const ClosedEndedQuestionEditor: React.FC<ClosedEndedQuestionEditorProps> = ({ quizItem }) => {
  const { t } = useTranslation()
  const [method, setMethod] = useState(0)
  const [testStrings, setTestStrings] = useState([""])

  const handleTestStringChange = (updatedIdx: number) => (value: string) => {
    setTestStrings(
      testStrings.map((content, idx) => {
        if (idx == updatedIdx) {
          return value
        }
        return content
      }),
    )
  }

  const addNewString = () => {
    setTestStrings([...testStrings, ""])
  }

  return (
    <EditorCard title={t("quiz-open-name")}>
      <OptionTitle> {t("grading-strategy")} </OptionTitle>
      <RadioButtonContainer>
        <RadioButton
          checked={method == 0}
          onClick={() => setMethod(0)}
          label={t("exact-string")}
        ></RadioButton>
        <RadioButton
          checked={method == 1}
          onClick={() => setMethod(1)}
          label={t("regex")}
        ></RadioButton>
      </RadioButtonContainer>
      {method == 0 && <ExactStringMethodView quizItem={quizItem} />}
      {method == 1 && <RegexMethodView quizItem={quizItem} />}
      <Accordion variant="detail" title={t("advanced-options")}>
        <details>
          <summary> {t("advanced-options")} </summary>
          <TestButtonContainer>
            {testStrings.map((string, idx) => (
              <TextField
                key={`test-string-field-${idx}`}
                value={string}
                onChange={handleTestStringChange(idx)}
                label={t("test-string")}
                name={t("test-string")}
              />
            ))}
            <AddNewRowContainer>
              <CircleButton icon={faPlus} onClick={() => addNewString()} />
              <p> {t("add-example-string")}</p>
            </AddNewRowContainer>
          </TestButtonContainer>
          <RegexTestTableContainer>
            <RegexTestTable quizItem={quizItem} testStrings={testStrings} />
          </RegexTestTableContainer>
        </details>
      </Accordion>
    </EditorCard>
  )
}

export default ClosedEndedQuestionEditor
