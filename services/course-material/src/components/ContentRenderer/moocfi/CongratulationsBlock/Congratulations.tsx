import styled from "@emotion/styled"
import React from "react"
import { useTranslation } from "react-i18next"

import ConfettiBg from "../../../../img/confetti-bg.svg"
import BackgroundImage from "../../../../img/congratulation-bg.svg"
import { UserModuleCompletionStatus } from "../../../../shared-module/bindings"
import { headingFont } from "../../../../shared-module/styles"
import { respondToOrLarger } from "../../../../shared-module/styles/respond"

import CongratulationsLinks from "./CongratulationsLinks"
import ModuleCard from "./ModuleCard"

// eslint-disable-next-line i18next/no-literal-string
const Wrapper = styled.div`
  font-family: ${headingFont};
  background: #6ba578;
  width: 100%;
  border-radius: 4px;
  min-height: 100vh;
  display: flex;
  justify-content: center;
  align-items: center;
  position: relative;
  overflow: hidden !important;
  margin: 1rem 0;
`
const Content = styled.div`
  width: 100%;
  height: auto;
  padding: 2rem;
  text-align: center;
  z-index: 99;

  .heading {
    color: #ffff;
    font-weight: 800;
    font-size: clamp(30px, 5vw, 70px) !important;
  }

  .subtitle {
    display: inline-block;
    font-size: 22px;
    font-weight: 500;
    width: 100%;
    color: #fff;
    line-height: 150%;
    opacity: 1;

    ${respondToOrLarger.md} {
      width: 600px;
    }
  }
`

export const RegisterLink = styled.a`
  padding: 1rem 2rem;
  background: #1a2333;
  font-size: 18px;
  margin-right: 10px;
  font-weight: bold;
  height: 100%;
  color: #6fb27e;

  ${respondToOrLarger.md} {
    font-size: 20px;
  }
`

const StyledSVG = styled(ConfettiBg)`
  position: absolute;
  left: 0;
  top: 0;
  z-index: -1;
`
const StyledBackground = styled(BackgroundImage)`
  position: absolute;
  top: -30%;
`
const ModuleWrapper = styled.div`
  display: grid;
  grid-template-columns: minmax(auto, 480px);
  grid-gap: 20px;
  margin-top: 3rem;
  justify-content: center;

  ${respondToOrLarger.lg} {
    grid-template-columns: 480px 480px;
  }
`

export interface CongratulationsProps {
  modules: Array<UserModuleCompletionStatus>
}

const Congratulations: React.FC<React.PropsWithChildren<CongratulationsProps>> = ({ modules }) => {
  const { t } = useTranslation()
  const multipleModules = modules.length > 1
  return (
    <Wrapper>
      <StyledBackground />
      <Content>
        <StyledSVG />
        <h1 className="heading">{t("congratulations")}!</h1>
        <span className="subtitle">
          {t("you-have-completed-the-course-to-receive-credits-or-certificate-use-following-links")}
        </span>
        {!multipleModules && <CongratulationsLinks module={modules[0]} />}
        {multipleModules && (
          <ModuleWrapper>
            {modules
              .sort((a, b) => a.order_number - b.order_number)
              .map((module) => (
                <ModuleCard key={module.module_id} module={module} />
              ))}
          </ModuleWrapper>
        )}
      </Content>
    </Wrapper>
  )
}

export default Congratulations
