import styled from "@emotion/styled"
import { faTrash, faWindowClose } from "@fortawesome/free-solid-svg-icons"
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome"
import { Fade } from "@mui/material"
import React from "react"
import { useTranslation } from "react-i18next"
import { useDispatch } from "react-redux"

import { NormalizedQuizItem } from "../../../../../types/types"
import TextField from "../../../../shared-module/components/InputFields/TextField"
import { deletedItem } from "../../../../store/editor/editorActions"
import { setAdvancedEditing } from "../../../../store/editor/itemVariables/itemVariableActions"
import { editedItemMaxWords, editedItemMinWords } from "../../../../store/editor/items/itemAction"
import { useTypedSelector } from "../../../../store/store"
import {
  AdvancedBox,
  AdvancedBoxModalOpenClass,
  CloseButton,
  DeleteButton,
  ModalButtonWrapper,
  StyledModal,
} from "../../../Shared/Modal"

import EssayModalContent from "./EssayModalContent"

const OneLineInfoContainer = styled.div`
  padding: 1rem 0;
  display: flex;
`

const InlineFieldWrapper = styled.div`
  &:not(:last-of-type) {
    margin-right: 1rem;
  }
  width: 100%;
`

interface EssayContentProps {
  item: NormalizedQuizItem
}

const EssayContent: React.FC<React.PropsWithChildren<EssayContentProps>> = ({ item }) => {
  const { t } = useTranslation()
  const quizId = useTypedSelector((state) => state.editor.quizId)
  const storeItem = useTypedSelector((state) => state.editor.items[item.id])
  const variables = useTypedSelector((state) => state.editor.itemVariables[item.id])

  const dispatch = useDispatch()

  return (
    <>
      <StyledModal
        open={variables.advancedEditing}
        onClose={() => dispatch(setAdvancedEditing({ itemId: storeItem.id, editing: false }))}
      >
        <Fade in={variables.advancedEditing}>
          <AdvancedBox
            className={AdvancedBoxModalOpenClass(variables.advancedEditingYAxisLocation)}
          >
            <ModalButtonWrapper>
              <CloseButton
                aria-label={t("close")}
                onClick={() =>
                  dispatch(setAdvancedEditing({ itemId: storeItem.id, editing: false }))
                }
              >
                <FontAwesomeIcon icon={faWindowClose} size="2x" />
              </CloseButton>
            </ModalButtonWrapper>
            <EssayModalContent item={storeItem} />
            <ModalButtonWrapper>
              <DeleteButton
                onClick={() => {
                  dispatch(deletedItem(storeItem.id, quizId))
                }}
              >
                <FontAwesomeIcon icon={faTrash} size="2x" color="red" />
              </DeleteButton>
            </ModalButtonWrapper>
          </AdvancedBox>
        </Fade>
      </StyledModal>
      <OneLineInfoContainer>
        <InlineFieldWrapper>
          <TextField
            label={t("min-words")}
            value={storeItem.minWords?.toString() ?? ""}
            type="number"
            onChange={(value) => dispatch(editedItemMinWords(storeItem.id, Number(value)))}
          />
        </InlineFieldWrapper>
        <InlineFieldWrapper>
          <TextField
            label={t("max-words")}
            value={storeItem.maxWords?.toString() ?? ""}
            type="number"
            onChange={(value) => dispatch(editedItemMaxWords(storeItem.id, Number(value)))}
          />
        </InlineFieldWrapper>
      </OneLineInfoContainer>
    </>
  )
}

export default EssayContent
