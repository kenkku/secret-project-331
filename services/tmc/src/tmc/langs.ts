/* eslint-disable i18next/no-literal-string */

import * as cp from "child_process"
import * as readline from "readline"
import kill from "tree-kill"

import { OutputData } from "./cli"
import { isCliOutput } from "./cli.guard"
import { Compression, ExercisePackagingConfiguration } from "./generated"

const execute = async (cmd: string, args: Array<string>): Promise<OutputData> => {
  const cliPath = "/app/tmc-langs-cli"
  const executableArgs = [cmd, ...args]
  console.log("executing", cliPath, executableArgs.join(" "))

  const cprocess = cp.spawn(cliPath, executableArgs, {
    env: {
      ...process.env,
      RUST_LOG: "debug,j4rs=error",
      RUST_BACKTRACE: "1",
    },
  })

  return new Promise<OutputData>((resolve, reject) => {
    const timeout = setTimeout(() => {
      kill(cprocess.pid as number)
      reject("Process didn't seem to finish or was taking a really long time.")
    }, 20 * 60 * 1000)

    // process events
    cprocess.on("error", (error) => {
      // something went wrong with the process, reject
      clearTimeout(timeout)
      reject(error)
    })
    cprocess.on("close", (_code) => {
      // the process has finished
      clearTimeout(timeout)
    })

    // stdout/err events
    const rl = readline.createInterface({ input: cprocess.stdout })
    rl.on("line", (input) => {
      // received data from stdout
      try {
        const json = JSON.parse(input)
        if (isCliOutput(json)) {
          if (json["output-kind"] === "output-data") {
            const data = json.data
            if (data?.["output-data-kind"] === "error") {
              console.error("Error:", json.message)
              console.error("Trace:", data["output-data"].trace.join("\n"))
              reject(json)
            } else {
              // not an error
              resolve(json)
            }
          }
          switch (json["output-kind"]) {
            case "output-data":
              break
            case "status-update":
              console.log(json)
              break
            case "notification":
              console.error(json)
              break
            default:
          }
        } else {
          console.error("TMC-langs response didn't match expected type")
          console.error(json)
        }
      } catch (e) {
        console.warn("Failed to parse TMC-langs output")
        console.debug(input)
      }
    })
    cprocess.stderr.on("data", (chunk) => {
      // log errors
      console.error(chunk.toString())
    })
  })
}

export const extractProject = async (
  archivePath: string,
  outputPath: string,
  compression: Compression = "zstd",
  naive = false,
) => {
  await execute("extract-project", [
    "--archive-path",
    archivePath,
    "--output-path",
    outputPath,
    "--compression",
    compression,
    ...(naive ? ["--naive"] : []),
  ])
}

export const compressProject = async (
  exercisePath: string,
  outputPath: string,
  compression: Compression = "zstd",
  naive = false,
) => {
  await execute("compress-project", [
    "--exercise-path",
    exercisePath,
    "--output-path",
    outputPath,
    "--compression",
    compression,
    ...(naive ? ["--naive"] : []),
  ])
}

export const prepareSolution = async (exercisePath: string, outputPath: string) => {
  await execute("prepare-solution", ["--exercise-path", exercisePath, "--output-path", outputPath])
}

export const prepareStub = async (exercisePath: string, outputPath: string) => {
  await execute("prepare-stub", ["--exercise-path", exercisePath, "--output-path", outputPath])
}

export const prepareSubmission = async (
  clonePath: string,
  outputPath: string,
  submissionPath: string,
  submissionCompression: Compression = "zstd",
  naive = false,
) => {
  await execute("prepare-submission", [
    "--clone-path",
    clonePath,
    "--output-path",
    outputPath,
    "--submission-path",
    submissionPath,
    "--submission-compression",
    submissionCompression,
    ...(naive ? ["--extract-submission-naively"] : []),
  ])
}

export const getExercisePackagingConfiguration = async (
  exercisePath: string,
): Promise<ExercisePackagingConfiguration> => {
  const config = await execute("get-exercise-packaging-configuration", [
    "--exercise-path",
    exercisePath,
  ])
  if (config.data?.["output-data-kind"] === "exercise-packaging-configuration") {
    return config.data["output-data"]
  } else {
    throw "Unexpected data"
  }
}
