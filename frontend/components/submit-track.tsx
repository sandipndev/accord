/* eslint-disable @typescript-eslint/no-unused-expressions */

"use client"

import { useState } from "react"
import Link from "next/link"

import { useCreateTrackMutation, useTrackLazyQuery } from "@/lib/graphql/generated"
import { gql } from "@apollo/client"

import { Track } from "./track"
import { Loading } from "./loading"

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
    const url = new URL(youtubeUrl)
    url.searchParams.delete("list")
    const { data } = await createTrack({ variables: { youtubeUrl: url.toString() } })
    const trackId = data?.createTrack.id
    await track({ variables: { trackId } })
    setYoutubeUrl("")
  }

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
        {trackData?.track ? (
          <Link
            href={`/track/${trackData?.track.id}`}
            className="bg-green-500 hover:bg-green-600 text-white text-sm font-semibold rounded-lg px-4 py-2.5"
          >
            View
          </Link>
        ) : (
          <button
            onClick={handleSubmit}
            disabled={loading}
            type="button"
            className="bg-blue-500 hover:bg-blue-600 text-white text-sm font-semibold rounded-lg px-4 py-2.5"
          >
            {loading ? (
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

      {trackData && trackData.track && <Track track={trackData.track} />}
    </div>
  )
}
