import React, { Dispatch, SetStateAction } from "react"
import { useTranslation } from "react-i18next"

import { ExerciseFeedback } from "../pages/api/grade"
import { State } from "../pages/iframe"
import withErrorBoundary from "../shared-module/utils/withErrorBoundary"
import withNoSsr from "../shared-module/utils/withNoSsr"

import AnswerExercise from "./AnswerExercise"
import ExerciseEditor from "./ExerciseEditor"
import ViewSubmission from "./ViewSubmission"

interface RendererProps {
  state: State | null
  setState: Dispatch<SetStateAction<State | null>>
  port: MessagePort | null
}

const Renderer: React.FC<React.PropsWithChildren<RendererProps>> = ({ state, setState, port }) => {
  const { t } = useTranslation()

  if (!port) {
    return <>{t("waiting-for-port")}</>
  }

  if (!state) {
    return <>{t("waiting-for-content")}</>
  }

  if (state.view_type === "answer-exercise") {
    return <AnswerExercise port={port} state={state.public_spec} />
  } else if (state.view_type === "view-submission") {
    const feedbackJson: unknown | null = state.grading?.feedback_json
    const exerciseFeedback = feedbackJson ? (feedbackJson as ExerciseFeedback) : null
    return (
      <ViewSubmission
        port={port}
        publicSpec={state.public_spec}
        answer={state.answer}
        gradingFeedback={exerciseFeedback}
        modelSolutionSpec={state.model_solution_spec ? state.model_solution_spec : null}
      />
    )
  } else if (state.view_type === "exercise-editor") {
    return <ExerciseEditor state={state.private_spec} port={port} setState={setState} />
  } else {
    return <>{t("waiting-for-content")}</>
  }
}

export default withErrorBoundary(withNoSsr(Renderer))
