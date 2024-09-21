"use client"
import { useState } from "react"

import { useCreateProcessMutation, useGetProcessLazyQuery } from "@/lib/graphql/generated"
import { gql } from "@apollo/client"
import { formatStatus } from "@/lib/utils"
import { Loading } from "./loading"
import Link from "next/link"

gql`
  mutation CreateProcess($youtubeUrl: String!) {
    createProcess(youtubeUrl: $youtubeUrl)
  }

  query GetProcess($id: ProcessId!) {
    getProcess(id: $id) {
      id
      name
      status
    }
  }
`

export const SubmitProcess = () => {
  const [createProcess, { error, loading }] = useCreateProcessMutation()
  const [process, { data: processData }] = useGetProcessLazyQuery({
    pollInterval: 10,
  })

  const [youtubeUrl, setYoutubeUrl] = useState("")

  const handleSubmit = async () => {
    const { data } = await createProcess({ variables: { youtubeUrl } })
    const processId = data?.createProcess

    await process({ variables: { id: processId } })
  }

  const isProcessing =
    loading ||
    (Boolean(processData?.getProcess) && processData?.getProcess.status !== "DONE")
  const done =
    Boolean(processData?.getProcess) && processData?.getProcess.status === "DONE"

  return (
    <div className="mt-6">
      <label htmlFor="link-input" className="block mb-2 text-white text-xl">
        Paste Youtube Link
      </label>
      <input
        type="url"
        id="link-input"
        value={youtubeUrl}
        onChange={(e) => setYoutubeUrl(e.target.value)}
        placeholder="https://www.youtube.com/watch?v=..."
        className="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
      />
      <div className="mt-4">
        {done ? (
          <Link
            href={`/details/${processData.getProcess.id}`}
            className="bg-green-500 hover:bg-green-600 text-white text-sm font-semibold rounded-lg px-4 py-2.5"
          >
            View
          </Link>
        ) : (
          <button
            onClick={handleSubmit}
            disabled={isProcessing}
            type="button"
            className="bg-blue-500 hover:bg-blue-600 text-white text-sm font-semibold rounded-lg px-4 py-2.5"
          >
            {isProcessing ? (
              <div className="flex space-x-2 items-center">
                <Loading />
                <div>Processing</div>
              </div>
            ) : (
              "Process"
            )}
          </button>
        )}
      </div>
      {error && (
        <div className="mt-4 text-red-500 text-sm font-semibold">{error.message}</div>
      )}
      {processData && processData.getProcess && (
        <div className="mt-4 text-green-500 text-xl font-semibold capitalize">
          <span className="mr-4">Current Status: </span>
          {formatStatus(processData.getProcess.status)}
        </div>
      )}
    </div>
  )
}
