import { css } from "@emotion/css"
import { useTranslation } from "react-i18next"

import { EmbedAttributes } from "../../../../../../types/GutenbergBlockAttributes"
import BreakFromCentered from "../../../../../shared-module/components/Centering/BreakFromCentered"
import { baseTheme } from "../../../../../shared-module/styles/theme"
import aspectRatioFromClassName from "../../../../../utils/aspectRatioFromClassName"
import { sanitizeCourseMaterialHtml } from "../../../../../utils/sanitizeCourseMaterialHtml"

export const YoutubeEmbedBlock: React.FC<React.PropsWithChildren<EmbedAttributes>> = (props) => {
  const { t } = useTranslation()
  const { url } = props
  let video = url?.split("v=")[1]
  if (url) {
    try {
      const parsedUrl = new URL(url)
      const vValue = parsedUrl.searchParams.get("v")
      if (vValue) {
        video = vValue
      }
    } catch (e) {
      // eslint-disable-next-line i18next/no-literal-string
      console.error(`Could not parse Youtube url: `, e)
    }
  }

  return (
    <BreakFromCentered sidebar={false}>
      <figure
        className={css`
          width: 100%;
          max-width: 1000px;
          margin: 4rem auto;
        `}
      >
        <iframe
          className={css`
            display: block;
            width: 100%;
            aspect-ratio: ${aspectRatioFromClassName(props.className)};
          `}
          src={`https://www.youtube-nocookie.com/embed/${video}`}
          title={t("title-youtube-video-player")}
          frameBorder="0"
          allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
          allowFullScreen
        />
        <figcaption
          className={css`
            text-align: center;
            font-size: ${baseTheme.fontSizes[0]}px;
            margin-top: 0.5em;
            margin-bottom: 1em;
            color: ${baseTheme.colors.gray[400]};
          `}
          dangerouslySetInnerHTML={{ __html: sanitizeCourseMaterialHtml(props.caption ?? "") }}
        ></figcaption>
      </figure>
    </BreakFromCentered>
  )
}
