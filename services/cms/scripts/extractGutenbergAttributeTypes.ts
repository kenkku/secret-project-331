/* eslint-disable i18next/no-literal-string */
// Require imports needs to happen in a specific order.
/* eslint-disable import/order */
/* eslint-disable @typescript-eslint/no-var-requires */

import { Block } from "@wordpress/blocks"
import { addFilter } from "@wordpress/hooks"
import fs from "fs"
import { compile } from "json-schema-to-typescript"
import { JSONSchema, JSONSchemaTypeName } from "json-schema-to-typescript/dist/src/types/JSONSchema"

import {
  modifyEmbedBlockAttributes,
  modifyImageBlockAttributes,
} from "../src/utils/Gutenberg/modifyBlockAttributes"

const jsdom = require("jsdom")
const { JSDOM } = jsdom

const dom = new JSDOM(`<body>
<script>document.body.appendChild(document.createElement("hr"));</script>
</body>`)
const mock = () => {
  // No op
}
Object.defineProperty(dom.window, "matchMedia", {
  writable: true,
  value: (query: unknown) => {
    return {
      matches: false,
      media: query,
      onchange: null,
      addListener: mock, // deprecated
      removeListener: mock, // deprecated
      addEventListener: mock,
      removeEventListener: mock,
      dispatchEvent: mock,
    }
  },
})
global.window = dom.window
global.document = dom.window.document
global.navigator = dom.window.navigator
// @ts-ignore: Just to prevent a crash, not used
global.CSS = {}

class FakeMutationObserver {
  observe() {
    // No op
  }
}

// @ts-ignore: Just to prevent a crash, not used
global.MutationObserver = FakeMutationObserver

// The following import order matters and are dependant on above window definition.
const blockLibrary = require("@wordpress/block-library")
const blocks = require("@wordpress/blocks")

addFilter("blocks.registerBlockType", "moocfi/modifyImageAttributes", modifyImageBlockAttributes)
addFilter("blocks.registerBlockType", "moocfi/modifyEmbedAttributes", modifyEmbedBlockAttributes)
const { supportedCoreBlocks } = require("../src/blocks/supportedGutenbergBlocks")

async function main() {
  blockLibrary.registerCoreBlocks()

  blocks.getBlockTypes().forEach((block: Block<Record<string, unknown>>) => {
    if (supportedCoreBlocks.indexOf(block.name) === -1) {
      blocks.unregisterBlockType(block.name)
    }
  })

  const sanitizeNames = (name: string) => {
    const newName = name.replace("core/", "").replace(/-./g, (x) => x.toUpperCase()[1])
    return newName.charAt(0).toUpperCase() + newName.slice(1) + "Attributes"
  }

  const blockTypes: Array<Block<Record<string, unknown>>> = blocks.getBlockTypes()
  const jsonSchemaTypes = blockTypes.map((block) => {
    // Fetch core/table head, foot, body types
    if (block.name === "core/table") {
      const tableBlockJSON = require("@wordpress/block-library/src/table/block.json")
      const tableAttributes = tableBlockJSON.attributes
      return {
        title: sanitizeNames(block.name),
        type: "object" as JSONSchemaTypeName,
        properties: {
          ...block.attributes,
          // We can use same cells ref, as they seem to be same for head, foot, body
          head: {
            items: {
              $ref: "#/$defs/cells",
            },
          },
          foot: {
            items: {
              $ref: "#/$defs/cells",
            },
          },
          body: {
            items: {
              $ref: "#/$defs/cells",
            },
          },
        },
        additionalProperties: false,
        required: Object.entries(block.attributes)
          .filter(([_key, value]) => (value as { default: unknown }).default !== undefined)
          .map(([key, _value]) => key),
        $defs: {
          // Cells and CellAttributes are equal for all three in block.json that is imported from block-library
          cells: {
            type: "object" as JSONSchemaTypeName,
            properties: {
              cells: {
                ...tableAttributes.head.query.cells,
                items: {
                  $ref: "#/$defs/cellAttributes",
                },
              },
            },
            additionalProperties: false,
          },
          cellAttributes: {
            type: "object" as JSONSchemaTypeName,
            properties: {
              ...tableAttributes.head.query.cells.query,
            },
            additionalProperties: false,
          },
        },
      }
    }

    let res = [
      {
        title: sanitizeNames(block.name),
        type: "object" as JSONSchemaTypeName,
        properties: { ...block.attributes },
        additionalProperties: false,
        required: Object.entries(block.attributes)
          .filter(([_key, value]) => (value as { default: unknown }).default !== undefined)
          .map(([key, _value]) => key),
      },
    ]
    if (block.deprecated) {
      res = res.concat(
        block.deprecated?.map((deprecated, n) => {
          const blockName = sanitizeNames(block.name)
          return {
            title: blockName.replace("Attributes", `Deprecated${n + 1}Attributes`),
            type: "object" as JSONSchemaTypeName,
            properties: { ...deprecated.attributes },
            additionalProperties: false,
            required: Object.entries(deprecated.attributes)
              .filter(([_key, value]) => (value as { default: unknown }).default !== undefined)
              .map(([key, _value]) => key),
            deprecated: true,
          }
        }),
      )
    }

    return res
  })

  const typescriptTypes = await Promise.all(
    jsonSchemaTypes
      .flat()
      .filter((o) => !!o)
      .map(async (schema) => {
        const jsonSchema = schema as JSONSchema
        const title = jsonSchema.title ?? "SchemaWithoutName"
        let bannerComment = ``
        // @ts-ignore: We set this above
        if (schema.deprecated) {
          bannerComment = `/**
* @deprecated This is an older version of ${title.replaceAll(
            /Deprecated\d+/g,
            "",
          )}. We may need to support rendering this if someone has created content using an older version of Gutenberg.
*/`
        }
        return await compile(jsonSchema, title, {
          bannerComment,
        })
      }),
  )

  const types = `/* eslint-disable @typescript-eslint/no-empty-interface */

// ###########################################
// ## This file is autogenerated by running ##
// ## 'bin/extract-gutenberg-types'         ##
// ## in the root of the repo.              ##
// ##                                       ##
// ## Do not edit this file by hand.        ##
// ###########################################

${typescriptTypes.join("\n")}`

  await fs.promises.writeFile("../course-material/types/GutenbergBlockAttributes.ts", types)
  console.info("Done!")
  process.exit(0)
}

main()
