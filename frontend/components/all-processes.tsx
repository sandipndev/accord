/* eslint-disable @typescript-eslint/no-unused-expressions */

"use client"
import React from "react"
import { gql } from "@apollo/client"
import { useAllProcessesQuery } from "@/lib/graphql/generated"
import { formatStatus } from "@/lib/utils"
import Link from "next/link"

gql`
  query AllProcesses {
    getProcesses {
      id
      name
      status
    }
  }
`

export const AllProcesses = () => {
  const { data } = useAllProcessesQuery({
    pollInterval: 1000,
  })

  return (
    <details className="text-white mt-6" open>
      <summary>All Processes</summary>
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
            {data?.getProcesses.map((process) => (
              <tr key={process.id} className="border-b border-zinc-600">
                <td className="px-6 py-4">{process.name}</td>
                <td className="px-6 py-4">{formatStatus(process.status)}</td>
                <td>
                  <Link
                    href={`/details/${process.id}`}
                    className="p-2 bg-gray-500 rounded"
                  >
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
