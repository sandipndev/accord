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

  const { data, loading: processLoading } = useGetProcessQuery({
    variables: {
      id: processId,
    },
    pollInterval: 1000,
  })
  const [preloadingAudio, setPreloadingAudio] = useState(true)
  const loading = processLoading && preloadingAudio

  const [, setRenderCount] = useState(0)

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

      setPreloadingAudio(true)
    }
  }, [data, processId, setPreloadingAudio])

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

  useEffect(() => {
    const audioRefsCurrent = audioRefs.current
    return () => {
      Object.values(audioRefsCurrent).forEach((audio) => {
        audio.pause()
      })
      setIsPlaying(false)
    }
  }, [])

  const handleSeek = (e: React.ChangeEvent<HTMLInputElement>) => {
    const audio = audioRefs.current[currentSemitone]
    if (audio) {
      const time = parseFloat(e.target.value)
      audio.currentTime = time
      setCurrentTime(time)
    }
  }

  const handleDownload = () => {
    const semitoneLabel =
      currentSemitone >= 0 ? `+${currentSemitone}` : `${currentSemitone}`
    const fileName =
      currentSemitone === 0 ? `${processId}.mp3` : `${processId}_${semitoneLabel}_ST.mp3`
    const fileUrl = `/media/${fileName}`

    // Create a temporary anchor element to initiate download
    const link = document.createElement("a")
    link.href = fileUrl
    link.download = `${data?.getProcess.name}_${semitoneLabel}.mp3`
    document.body.appendChild(link)
    link.click()
    document.body.removeChild(link)
  }

  useEffect(() => {
    const i = setInterval(() => {
      setRenderCount((prev) => (prev + 1) % 100)
    }, 1000)
    return () => clearInterval(i)
  }, [])

  if (loading) return <div>Loading...</div>

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

  const playingAudio = audioRefs.current[currentSemitone]
  const playTime = (playingAudio && playingAudio.currentTime) || 0
  const duration = (playingAudio && playingAudio.duration) || 0

  return (
    <div>
      <div className="text-xl font-bold">{data?.getProcess.name}</div>
      <div className="text-sm text-cyan-500">{data?.getProcess.youtubeUrl}</div>
      <hr className="mt-6 border-zinc-700" />
      <div className="mt-4">
        <div>
          <strong className="font-bold text-xl">
            Current Semitone:{" "}
            {currentSemitone > 0 ? `+${currentSemitone}` : currentSemitone}
          </strong>
        </div>
        <div className="my-4">
          <button
            onClick={() => changeSemitone(-1)}
            disabled={currentSemitone <= -10}
            className="px-4 py-2 mr-2 bg-blue-500 hover:bg-blue-600 text-white rounded disabled:opacity-50"
          >
            -
          </button>
          <button
            onClick={() => changeSemitone(1)}
            disabled={currentSemitone >= 10}
            className="px-4 py-2 bg-blue-500 hover:bg-blue-600 text-white rounded disabled:opacity-50"
          >
            +
          </button>
        </div>
        <div>
          <button
            onClick={handlePlayPause}
            className="px-4 py-2 bg-green-500 hover:bg-green-600 text-white rounded"
          >
            {isPlaying ? "Pause" : "Play"}
          </button>
        </div>
        <div className="mt-8">
          <input
            type="range"
            min="0"
            max={duration}
            value={playTime}
            onChange={handleSeek}
            className="w-full range-input"
          />
          <div className="flex justify-between text-sm">
            <span>{formatTime(playTime)}</span>
            <span>{formatTime(duration)}</span>
          </div>
        </div>
        <div className="mt-8">
          <button
            className="px-4 py-2 bg-slate-500 text-white rounded"
            onClick={handleDownload}
          >
            Download
          </button>
        </div>
      </div>
    </div>
  )
}

export default ProcessDetail

// Format time in mm:ss
const formatTime = (time: number) => {
  if (isNaN(time)) return "0:00"
  const minutes = Math.floor(time / 60)
  const seconds = Math.floor(time % 60)
  return `${minutes}:${seconds < 10 ? "0" : ""}${seconds}`
}
