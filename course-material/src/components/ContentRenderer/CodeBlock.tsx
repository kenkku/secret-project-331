import { css } from "@emotion/css"
import { BlockRendererProps } from "."
import { normalWidthCenteredComponentStyles } from "../../styles/componentStyles"

interface CodeBlockAttributes {
  content: string
}

const CodeBlock: React.FC<BlockRendererProps<CodeBlockAttributes>> = ({ data }) => {
  const attributes: CodeBlockAttributes = data.attributes
  return (
    <pre
      className={css`
        ${normalWidthCenteredComponentStyles}
      `}
    >
      <code>{attributes.content}</code>
    </pre>
  )
}

export default CodeBlock
