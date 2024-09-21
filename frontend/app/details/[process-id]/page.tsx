"use client"

import { useGetProcessQuery } from "@/lib/graphql/generated"
import { useEffect, useState, useRef } from "react"

type Props = {
  params: {
    "process-id": string
  }
}

const ProcessDetail: React.FC<Props> = ({ params }) => {
  const processId = params["process-id"]

  const { data } = useGetProcessQuery({
    variables: {
      id: processId,
    },
    pollInterval: 1000,
  })

  const [currentSemitone, setCurrentSemitone] = useState(0)
  const [isPlaying, setIsPlaying] = useState(false)
  const [currentTime, setCurrentTime] = useState(0)
  const audioRefs = useRef<{ [key: number]: HTMLAudioElement }>({})

  // Preload all audio files once data is available
  useEffect(() => {
    if (data?.getProcess.status === "DONE") {
      const semitoneShifts = []
      for (let i = -10; i <= 10; i++) {
        semitoneShifts.push(i)
      }

      semitoneShifts.forEach((semitone) => {
        const semitoneLabel = semitone >= 0 ? `+${semitone}` : `${semitone}`
        const fileName = `${processId}_${semitoneLabel}_ST.mp3`
        const fileUrl = `/media/${fileName}`

        if (semitone === 0) {
          const audio = new Audio(`/media/${processId}.mp3`)
          audioRefs.current[semitone] = audio
        } else {
          const audio = new Audio(fileUrl)
          audioRefs.current[semitone] = audio
        }
      })
    }
  }, [data, processId])

  // Update currentTime when the audio is playing
  useEffect(() => {
    const audio = audioRefs.current[currentSemitone]
    if (audio) {
      const updateTime = () => setCurrentTime(audio.currentTime)
      audio.addEventListener("timeupdate", updateTime)
      return () => {
        audio.removeEventListener("timeupdate", updateTime)
      }
    }
  }, [currentSemitone])

  // Handle play/pause functionality
  const handlePlayPause = () => {
    const audio = audioRefs.current[currentSemitone]
    if (audio) {
      if (isPlaying) {
        audio.pause()
        setIsPlaying(false)
      } else {
        audio.currentTime = currentTime
        audio.play()
        setIsPlaying(true)
      }
    }
  }

  // Change semitone and update audio
  const changeSemitone = (delta: number) => {
    const newSemitone = currentSemitone + delta
    if (newSemitone < -10 || newSemitone > 10) return

    const prevAudio = audioRefs.current[currentSemitone]
    const nextAudio = audioRefs.current[newSemitone]
    if (prevAudio && nextAudio) {
      prevAudio.pause()
      const time = prevAudio.currentTime
      setCurrentTime(time)

      nextAudio.currentTime = time
      if (isPlaying) {
        nextAudio.play()
      }
      setCurrentSemitone(newSemitone)
    }
  }

  if (data?.getProcess.status !== "DONE")
    return (
      <>
        <div className="">{data?.getProcess.name}</div>
        <div>
          Still Processing...
          <br /> Current State: <span>{data?.getProcess.status}</span>
        </div>
      </>
    )

  return (
    <div>
      <div className="text-xl">{data?.getProcess.name}</div>
      <div className="mt-4">
        <div>
          <strong>Current Semitone: {currentSemitone}</strong>
        </div>
        <div className="my-4">
          <button
            onClick={() => changeSemitone(-1)}
            disabled={currentSemitone <= -10}
            className="px-4 py-2 mr-2 bg-blue-500 text-white rounded disabled:opacity-50"
          >
            -
          </button>
          <button
            onClick={() => changeSemitone(1)}
            disabled={currentSemitone >= 10}
            className="px-4 py-2 bg-blue-500 text-white rounded disabled:opacity-50"
          >
            +
          </button>
        </div>
        <div>
          <button
            onClick={handlePlayPause}
            className="px-4 py-2 bg-green-500 text-white rounded"
          >
            {isPlaying ? "Pause" : "Play"}
          </button>
        </div>
      </div>
    </div>
  )
}

export default ProcessDetail
