import { createAction } from "typesafe-actions"

interface OpenAdvancedEditingModalAction {
  itemId: string
  editing: true
  mouseClickYPosition: number
}

interface CloseAdvancedEditingModalAction {
  itemId: string
  editing: false
}

type AdvancedEditingOptions = OpenAdvancedEditingModalAction | CloseAdvancedEditingModalAction

export const setAdvancedEditing = createAction(
  "SET_ADVANCED_EDITING",
  (options: AdvancedEditingOptions) => ({
    itemId: options.itemId,
    editing: options.editing,
    mouseClickYPosition: options.editing ? options.mouseClickYPosition : undefined,
  }),
)<{ itemId: string; editing: boolean; mouseClickYPosition: number | undefined }>()

export const setScaleMax = createAction("SET_SCALE_MAX", (itemId: string, newValue: number) => ({
  itemId: itemId,
  newValue: newValue,
}))<{ itemId: string; newValue: number }>()

export const setScaleMin = createAction("SET_SCALE_MIN", (itemId: string, newValue: number) => ({
  itemId: itemId,
  newValue: newValue,
}))<{ itemId: string; newValue: number }>()

// regex testing state
export const toggleValidRegexTestingState = createAction(
  "SET_TESTING_REGEX",
  (itemId: string, testing: boolean) => ({
    itemId: itemId,
    testing: testing,
  }),
)<{ itemId: string; testing: boolean }>()

export const toggleFormatRegexTestingState = createAction(
  "SET_FORMAT_TESTING_REGEX",
  (itemId: string, testing: boolean) => ({
    itemId: itemId,
    testing: testing,
  }),
)<{ itemId: string; testing: boolean }>()

// input test regexes
export const setValidityTestRegex = createAction(
  "SET_TEST_REGEX",
  (itemId: string, testRegex: string) => ({
    itemId: itemId,
    testRegex: testRegex,
  }),
)<{ itemId: string; testRegex: string }>()

export const setFormatTestRegex = createAction(
  "SET_FORMAT_TEST_REGEX",
  (itemId: string, testRegex: string) => ({
    itemId: itemId,
    testRegex: testRegex,
  }),
)<{ itemId: string; testRegex: string }>()

// regex test answers
export const setValidityRegexTestAnswer = createAction(
  "SETREGEX_TESTANSWER",
  (itemId: string, testAnswer: string) => ({
    itemId: itemId,
    testAnswer: testAnswer,
  }),
)<{ itemId: string; testAnswer: string }>()

export const setFormatRegexTestAnswer = createAction(
  "SET_FORMAT_REGEX_TESTANSWER",
  (itemId: string, testAnswer: string) => ({
    itemId: itemId,
    testAnswer: testAnswer,
  }),
)<{ itemId: string; testAnswer: string }>()

// are the regexes valid
export const setValidValidityRegex = createAction(
  "SET_VALIDREGEX",
  (itemId: string, valid: boolean) => ({
    itemId: itemId,
    valid: valid,
  }),
)<{ itemId: string; valid: boolean }>()

export const setFormatValidityRegex = createAction(
  "SET_VALIDFORMATREGEX",
  (itemId: string, valid: boolean) => ({
    itemId: itemId,
    valid: valid,
  }),
)<{ itemId: string; valid: boolean }>()
