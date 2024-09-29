/* eslint-disable @typescript-eslint/no-unused-expressions */

"use client"
import { useState } from "react"

import { useCreateTrackMutation, useTrackLazyQuery } from "@/lib/graphql/generated"
import { gql } from "@apollo/client"
import { formatStatus } from "@/lib/utils"
import { Loading } from "./loading"
import Link from "next/link"

gql`
  mutation CreateTrack($youtubeUrl: YoutubeUrl!) {
    createTrack(youtubeUrl: $youtubeUrl) {
      id
    }
  }

  query Track($trackId: TrackId!) {
    track(trackId: $trackId) {
      id
      name
      youtubeUrl
      createdAt
      semitones {
        id
        shift
        status
        createdAt
      }
    }
  }
`

export const SubmitTrack = () => {
  const [createTrack, { error, loading }] = useCreateTrackMutation()
  const [track, { data: trackData }] = useTrackLazyQuery({
    pollInterval: 10,
  })

  const [youtubeUrl, setYoutubeUrl] = useState("")

  const handleSubmit = async () => {
    const { data } = await createTrack({ variables: { youtubeUrl } })
    const trackId = data?.createTrack
    await track({ variables: { trackId } })
  }

  const allDone = trackData?.track.semitones.every(({ status }) => status === "DONE")
  const isProcessing = loading || (Boolean(trackData?.track) && !allDone)
  const done = Boolean(trackData?.track) && allDone

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
            href={`/track/${trackData?.track.id}`}
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
      {trackData && trackData.track && (
        <div className="mt-4 text-green-500 text-xl font-semibold capitalize">
          <span className="mr-4">Current Status: </span>
          {formatStatus(isProcessing ? "Processing" : "Done")}
        </div>
      )}
    </div>
  )
}
