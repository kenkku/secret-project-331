import { useQuery } from "@tanstack/react-query"
import React from "react"
import { Trans, useTranslation } from "react-i18next"

import Layout from "../../../components/Layout"
import { fetchCompletionRegistrationLink } from "../../../services/backend/course-modules"
import ErrorBanner from "../../../shared-module/components/ErrorBanner"
import Spinner from "../../../shared-module/components/Spinner"
import dontRenderUntilQueryParametersReady, {
  SimplifiedUrlQuery,
} from "../../../shared-module/utils/dontRenderUntilQueryParametersReady"
import withErrorBoundary from "../../../shared-module/utils/withErrorBoundary"

export interface CompletionRedirectPageProps {
  query: SimplifiedUrlQuery<"courseModuleId">
}

const CompletionRedirectPage: React.FC<React.PropsWithChildren<CompletionRedirectPageProps>> = ({
  query,
}) => {
  const { courseModuleId } = query
  const { t } = useTranslation()
  const userCompletionInformation = useQuery(
    [`course-${courseModuleId}-completion-registration-link`],
    () => fetchCompletionRegistrationLink(courseModuleId),
    {
      onSuccess: (data) => window.location.replace(data.url),
    },
  )
  return (
    <Layout>
      {userCompletionInformation.isError && (
        <ErrorBanner error={userCompletionInformation.error} variant={"readOnly"} />
      )}
      {userCompletionInformation.isLoading && <Spinner variant={"medium"} />}
      {userCompletionInformation.isSuccess && (
        <div>
          <Trans
            t={t}
            i18nKey="you-are-being-redirected-to-completion-registration-page-if-nothing-happens-click-here"
          >
            You are automatically being redirected to Open University&apos;s completion registration
            page. If nothing happens, please{" "}
            <a
              href={userCompletionInformation.data.url}
              onClick={(event) => {
                event.preventDefault()
                window.location.replace(userCompletionInformation.data.url)
              }}
            >
              click here
            </a>
            .
          </Trans>
        </div>
      )}
    </Layout>
  )
}

export default withErrorBoundary(dontRenderUntilQueryParametersReady(CompletionRedirectPage))
