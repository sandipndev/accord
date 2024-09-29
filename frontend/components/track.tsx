import { SemitoneStatus, TrackQuery } from "@/lib/graphql/generated"
import { formatStatus } from "@/lib/utils"

type TrackProps = {
  track: NonNullable<TrackQuery["track"]>
}

export const Track: React.FC<TrackProps> = ({ track }) => {
  return (
    <>
      <div className="mt-4 text-green-500 font-semibold capitalize flex space-x-4 items-center">
        <span>Name:</span>
        <span>{track.name}</span>
      </div>
      <div className="text-green-500 font-semibold capitalize flex space-x-4 items-center">
        <span>Current Status:</span>
        <span>
          {formatStatus(
            track.semitones.every(({ status }) => status === SemitoneStatus.Completed)
              ? "Done"
              : "Processing",
          )}
        </span>
      </div>
    </>
  )
}
