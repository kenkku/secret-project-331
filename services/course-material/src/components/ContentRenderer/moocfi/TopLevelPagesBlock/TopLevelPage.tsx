import { css } from "@emotion/css"
import styled from "@emotion/styled"
import { useQuery } from "@tanstack/react-query"
import React from "react"
import { useTranslation } from "react-i18next"

import { fetchTopLevelPages } from "../../../../services/backend"
import ErrorBanner from "../../../../shared-module/components/ErrorBanner"
import Spinner from "../../../../shared-module/components/Spinner"
import TopLevelPage from "../../../../shared-module/components/TopLevelPage"
import useQueryParameter from "../../../../shared-module/hooks/useQueryParameter"
import { headingFont } from "../../../../shared-module/styles"
import withErrorBoundary from "../../../../shared-module/utils/withErrorBoundary"
import { coursePageRoute } from "../../../../utils/routing"

export interface TopLevelPagesProps {
  courseId: string
}

const Wrapper = styled.div`
  margin: 3rem 0;
`

const TopLevelPages: React.FC<React.PropsWithChildren<TopLevelPagesProps>> = ({ courseId }) => {
  const { t } = useTranslation()
  const getTopLevelPages = useQuery([`courses-${courseId}-top-level-pages`], () =>
    fetchTopLevelPages(courseId).then((pages) =>
      pages.filter((x) => x.url_path !== "/").sort((a, b) => a.order_number - b.order_number),
    ),
  )
  const courseSlug = useQueryParameter("courseSlug")
  const organizationSlug = useQueryParameter("organizationSlug")
  return (
    <>
      {getTopLevelPages.isError && (
        <ErrorBanner variant={"readOnly"} error={getTopLevelPages.error} />
      )}
      {getTopLevelPages.isLoading && <Spinner variant={"medium"} />}
      {getTopLevelPages.isSuccess && (
        <>
          {getTopLevelPages.data && (
            <Wrapper>
              <h2
                className={css`
                  font-family: ${headingFont};
                  font-size: clamp(30px, 3.5vw, 46px);
                  font-weight: 700;
                  color: #1a2333;
                  text-align: center;
                  margin-bottom: 1.5rem;
                  opacity: 0.9;
                `}
              >
                {t("information-pages")}
              </h2>
              {getTopLevelPages.data.map((page, index) => {
                const url = coursePageRoute(organizationSlug, courseSlug, page.url_path)
                return (
                  <TopLevelPage
                    title={page.title}
                    url={url}
                    key={page.id}
                    index={Number(index) + 1}
                  />
                )
              })}
            </Wrapper>
          )}
        </>
      )}
    </>
  )
}
export default withErrorBoundary(TopLevelPages)
