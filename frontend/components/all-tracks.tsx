/* eslint-disable @typescript-eslint/no-unused-expressions */

"use client"
import React from "react"
import { gql } from "@apollo/client"
import { useTracksQuery } from "@/lib/graphql/generated"
import { formatStatus } from "@/lib/utils"
import Link from "next/link"

gql`
  query Tracks {
    tracks {
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

export const AllTracks = () => {
  const { data } = useTracksQuery({
    pollInterval: 1000,
  })

  return (
    <details className="text-white mt-6" open>
      <summary>All Tracks</summary>
      <div className="mt-4 text-white">
        <table className="min-w-full p-6">
          <thead>
            <tr className="border-zinc-600 border-b-2">
              <th className="px-6 py-3 text-left text-xs font-medium  uppercase tracking-wider">
                Name
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider">
                Status
              </th>
              <th></th>
            </tr>
          </thead>
          <tbody>
            {data?.tracks.map((track) => (
              <tr key={track.id} className="border-b border-zinc-600">
                <td className="px-6 py-4">{track.name}</td>
                <td className="px-6 py-4">
                  {formatStatus(
                    track.semitones.every(({ status }) => status === "DONE")
                      ? "Done"
                      : "Processing",
                  )}
                </td>
                <td>
                  <Link href={`/track/${track.id}`} className="p-2 bg-gray-500 rounded">
                    View
                  </Link>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </details>
  )
}
