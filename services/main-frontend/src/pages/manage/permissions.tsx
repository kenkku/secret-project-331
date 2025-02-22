import { css } from "@emotion/css"
import React from "react"
import { useTranslation } from "react-i18next"

import Layout from "../../components/Layout"
import { PermissionPage } from "../../components/PermissionPage"
import { respondToOrLarger } from "../../shared-module/styles/respond"

const GlobalPermissions: React.FC<React.PropsWithChildren<unknown>> = () => {
  const { t } = useTranslation()

  return (
    <Layout navVariant="simple">
      <div
        className={css`
          margin-top: 40px;
          ${respondToOrLarger.sm} {
            margin-top: 80px;
          }
        `}
      >
        <h1>{t("global-permissions")}</h1>
        <PermissionPage
          domain={{
            // eslint-disable-next-line i18next/no-literal-string
            tag: "Global",
          }}
        />
      </div>
    </Layout>
  )
}

export default GlobalPermissions
