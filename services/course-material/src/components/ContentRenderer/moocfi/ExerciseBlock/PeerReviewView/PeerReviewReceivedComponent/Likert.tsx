import styled from "@emotion/styled"
import React from "react"
import { useTranslation } from "react-i18next"

import { headingFont } from "../../../../../../shared-module/styles"

import SVGMatcher from "./SVGmatcher"

interface LinkertProps {
  question: string
  index: number
  content: number | null
}

interface StyledProps {
  active: boolean
}

const Wrapper = styled.div`
  margin: 0 auto;
  padding: 1rem;
`
const Question = styled.span`
  font-size: 16px;
  color: #215887;
  line-height: 1.4;
  margin-bottom: 0.8rem;
`
const IconContainer = styled.div`
  min-height: 100px;
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(100px, 1fr));
  gap: 20px;
  margin: 10px auto 0 auto;
  max-width: 1000px;
`

/* eslint-disable i18next/no-literal-string */
const Icon = styled.div<StyledProps>`
  width: 150px;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  padding: 25px 0;
  background-color: ${({ active }: StyledProps) => (active ? "#313947" : " #f5f6f7")};
  cursor: pointer;
  border-radius: 4px;
  transition: all 0.2s;

  svg .bg {
    fill: ${({ active }) => active && "#ffd93b"};
  }

  .linkert-scale-text {
    margin-top: 6px;
    font-size: 15px;
    font-weight: 600;
    color: ${({ active }) => (active ? "#ffffff" : "#313947")};
    font-family: ${headingFont};
  }
`

const Likert: React.FC<LinkertProps> = ({ question, content, index }) => {
  const { t } = useTranslation()
  const arr = [
    {
      text: t("likert-scale-strongly-disagree"),
    },
    {
      text: t("likert-scale-disagree"),
    },
    {
      text: t("likert-scale-neither-agree-nor-disagree"),
    },
    {
      text: t("likert-scale-agree"),
    },
    {
      text: t("likert-scale-strongly-agree"),
    },
  ]

  return (
    <Wrapper>
      <Question>{`${t("question")} ${index + 1}: ${question}`}</Question>
      <IconContainer>
        {arr.map((option, n) => (
          <Icon key={n} active={content === n + 1}>
            {SVGMatcher(option.text.toLowerCase())}
            <p className="linkert-scale-text">{option.text}</p>
          </Icon>
        ))}
      </IconContainer>
    </Wrapper>
  )
}

export default Likert
