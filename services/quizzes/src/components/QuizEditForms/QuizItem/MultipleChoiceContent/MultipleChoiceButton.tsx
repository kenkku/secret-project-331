import { css } from "@emotion/css"
import styled from "@emotion/styled"
import { faTrash, faWindowClose } from "@fortawesome/free-solid-svg-icons"
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome"
import { Box, Button, Fade, Modal } from "@mui/material"
import React, { useContext, useState } from "react"
import { useTranslation } from "react-i18next"
import { useDispatch } from "react-redux"

import { NormalizedQuizItemOption } from "../../../../../types/types"
import IframeHeightContext from "../../../../shared-module/contexts/IframeHeightContext"
import { deletedOption } from "../../../../store/editor/editorActions"
import { setOptionEditing } from "../../../../store/editor/optionVariables/optionVariableActions"
import { useTypedSelector } from "../../../../store/store"
import { quizTheme } from "../../../../styles/QuizStyles"
import ParsedText from "../../../ParsedText"

import OptionModalContent from "./OptionModalContent"

const StyledModal = styled(Modal)`
  display: flex;
  align-items: center;
  justify-content: center;
`

const StyledEmotionBox = (heightOffset: number | undefined) => css`
  ${heightOffset &&
  `
    position: fixed;
    top: ${heightOffset + 15}px;
    ::before {
      content: "";
      width: 0;
      height: 0;
      border-style: solid;
      border-width: 0px 12px 15px 12px;
      border-color: transparent transparent white transparent;
      position: absolute;
      left: 0;
      right: 0;
      top: -15px;
      bottom: 0;
      margin: 0 auto;
    }
  `}
  background-color: #fafafa;
  min-width: 50% !important;
`
const StyledBox = styled(Box)`
  background-color: #fafafa;
  min-width: 50% !important;
`

const CloseButton = styled(Button)`
  padding: 1rem !important;
  float: right;
`

const DeleteOptionButton = styled(Button)`
  display: flex !important;
  padding: 1rem !important;
  float: right;
`
// eslint-disable-next-line i18next/no-literal-string
const BUTTON_BASE = `
  display: flex;
  border: none;
  flex: 1;
  flex-wrap: wrap;
  padding: 0 1rem;
  height: 85px;
  width:90%;
  color: ${quizTheme.gradingWrongItemColor};

  @media only screen and (max-width: 600px) {
    width: 100%;
  }
`
// eslint-disable-next-line i18next/no-literal-string
const CorrectButton = styled(Button)`
  ${BUTTON_BASE}
  background-color: ${quizTheme.quizCorrectAnswer};
  :hover {
    background-color: ${quizTheme.quizCorrectAnswerHover};
    border: none;
  }
`
// eslint-disable-next-line i18next/no-literal-string
const IncorrectButton = styled(Button)`
  ${BUTTON_BASE}
  background-color: ${quizTheme.quizWrongAnswer};
  :hover {
    background-color: ${quizTheme.quizWrongAnswerHover};
    border: none;
  }
`

interface MultipleChoiceButtonProps {
  option: NormalizedQuizItemOption
  index: number
}

const MultipleChoiceButton: React.FC<React.PropsWithChildren<MultipleChoiceButtonProps>> = ({
  option,
  index,
}) => {
  const { t } = useTranslation()
  const storeOption = useTypedSelector((state) => state.editor.options[option.id])
  const storeItem = useTypedSelector((state) => state.editor.items[option.quizItemId])
  const variables = useTypedSelector((state) => state.editor.optionVariables[option.id])
  const dispatch = useDispatch()
  const [heightOffset, setHeightOffset] = useState<number | undefined>(undefined)
  const iframeHeight = useContext(IframeHeightContext).height
  const ariaLabel = t("aria-label-option-index", { index })

  const handleClick = (event: React.MouseEvent<HTMLButtonElement, MouseEvent>) => {
    if (iframeHeight < event.pageY + 570) {
      setHeightOffset(iframeHeight - 600)
    } else {
      setHeightOffset(event.pageY)
    }
    dispatch(setOptionEditing(storeOption.id, true))
  }
  return (
    <>
      <StyledModal
        open={variables.optionEditing}
        onClose={() => dispatch(setOptionEditing(storeOption.id, false))}
      >
        <Fade in={variables.optionEditing}>
          <StyledBox className={StyledEmotionBox(heightOffset)}>
            <CloseButton
              aria-label={t("close")}
              onClick={() => dispatch(setOptionEditing(storeOption.id, false))}
            >
              <FontAwesomeIcon icon={faWindowClose} size="2x" />
            </CloseButton>
            <OptionModalContent option={storeOption} />
            <DeleteOptionButton
              onClick={() => {
                dispatch(setOptionEditing(storeOption.id, false))
                dispatch(deletedOption(storeOption.id, storeOption.quizItemId))
              }}
            >
              <FontAwesomeIcon icon={faTrash} size="2x" color="red" />
            </DeleteOptionButton>
          </StyledBox>
        </Fade>
      </StyledModal>
      {storeItem.allAnswersCorrect ? (
        <CorrectButton aria-label={ariaLabel} onClick={handleClick} variant="outlined">
          {storeOption.title}
        </CorrectButton>
      ) : (
        <>
          {storeOption.correct ? (
            <CorrectButton aria-label={ariaLabel} onClick={handleClick} variant="outlined">
              <ParsedText
                text={storeOption.title}
                errorText={t("quiz-title-invalid-format")}
                inline
                parseLatex
                parseMarkdown
              />
            </CorrectButton>
          ) : (
            <IncorrectButton aria-label={ariaLabel} onClick={handleClick} variant="outlined">
              <ParsedText
                text={storeOption.title}
                errorText={t("quiz-title-invalid-format")}
                inline
                parseLatex
                parseMarkdown
              />
            </IncorrectButton>
          )}
        </>
      )}
    </>
  )
}

export default MultipleChoiceButton
