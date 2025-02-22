import { useQuery } from "@tanstack/react-query"
import React, { useContext } from "react"

import PageContext from "../../../../contexts/PageContext"
import { fetchUserModuleCompletionStatuses } from "../../../../services/backend"
import BreakFromCentered from "../../../../shared-module/components/Centering/BreakFromCentered"
import ErrorBanner from "../../../../shared-module/components/ErrorBanner"
import LoginStateContext from "../../../../shared-module/contexts/LoginStateContext"
import withErrorBoundary from "../../../../shared-module/utils/withErrorBoundary"

import Congratulations from "./Congratulations"

const CongratulationsBlock: React.FC<React.PropsWithChildren<unknown>> = () => {
  const pageContext = useContext(PageContext)
  const courseInstanceId = pageContext.instance?.id
  const getModuleCompletions = useQuery(
    [`course-instance-${courseInstanceId}-module-completions`],
    () =>
      fetchUserModuleCompletionStatuses(courseInstanceId as NonNullable<typeof courseInstanceId>),
    { enabled: !!courseInstanceId },
  )

  const loginStateContext = useContext(LoginStateContext)
  if (!loginStateContext.signedIn) {
    return null
  }

  return (
    <>
      {getModuleCompletions.isError && (
        <ErrorBanner error={getModuleCompletions.error} variant="readOnly" />
      )}
      {getModuleCompletions.isLoading && null}
      {getModuleCompletions.isSuccess && (
        <>
          {/* This block is only visible after the default module is completed.*/}
          {getModuleCompletions.data.some((x) => x.default && x.completed) && (
            <BreakFromCentered sidebar={false}>
              <Congratulations modules={getModuleCompletions.data} />
            </BreakFromCentered>
          )}
        </>
      )}
    </>
  )
}

export default withErrorBoundary(CongratulationsBlock)
