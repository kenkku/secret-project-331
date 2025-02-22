import { ExamInstructions, ExerciseServiceIframeRenderingInfo } from "../../shared-module/bindings"
import {
  isExamInstructions,
  isExerciseServiceIframeRenderingInfo,
} from "../../shared-module/bindings.guard"
import { isArray, validateResponse } from "../../shared-module/utils/fetching"

import { cmsClient } from "./cmsClient"

export const fetchExamsInstructions = async (examId: string): Promise<ExamInstructions> => {
  const response = await cmsClient.get(`/exams/${examId}/edit`, {
    responseType: "json",
  })
  return validateResponse(response, isExamInstructions)
}

export const getAllExerciseServices = async (): Promise<ExerciseServiceIframeRenderingInfo[]> => {
  const response = await cmsClient.get(`/exercise-services`)
  return validateResponse(response, isArray(isExerciseServiceIframeRenderingInfo))
}
