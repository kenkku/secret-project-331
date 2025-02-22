/* eslint-disable i18next/no-literal-string */
import { css, injectGlobal } from "@emotion/css"

import {
  BASE_BUTTON_STYLES,
  PrimaryButtonStyles,
  SecondaryButtonStyles,
  TertiaryButtonStyles,
} from "../shared-module/components/Button"
import { primaryFont } from "../shared-module/styles"

// Using this instead of directly injectGlobal because stylelint works in this one.
const localCss = css`
  .wp-block-button__link {
    border-radius: 0;
    ${BASE_BUTTON_STYLES}
  }
  .is-style-material-primary-button .wp-block-button__link {
    ${PrimaryButtonStyles({ variant: "primary", size: "large" })}
  }
  .is-style-material-secondary-button .wp-block-button__link {
    ${SecondaryButtonStyles({ variant: "secondary", size: "large" })}
  }
  .is-style-material-tertiary-button .wp-block-button__link {
    ${TertiaryButtonStyles({ variant: "tertiary", size: "large" })}
  }
  .components-base-control {
    font-family: ${primaryFont} !important;
  }
`

injectGlobal`
${localCss}
`

const LocalStyles: React.FC<React.PropsWithChildren<unknown>> = () => null

export default LocalStyles
