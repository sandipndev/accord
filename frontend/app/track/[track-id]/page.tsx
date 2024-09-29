"use client"

import { Track } from "@/components/track"
import { SemitoneStatus, useTrackQuery } from "@/lib/graphql/generated"
import Link from "next/link"
import { useEffect, useState, useRef } from "react"

type Props = {
  params: {
    "track-id": string
  }
}

const TrackDetail: React.FC<Props> = ({ params }) => {
  const trackId = params["track-id"]

  const { data, loading: trackLoading } = useTrackQuery({
    variables: {
      trackId,
    },
    pollInterval: 1000,
  })
  const [preloadingAudio, setPreloadingAudio] = useState(true)
  const loading = trackLoading && preloadingAudio

  const [, setRenderCount] = useState(0)

  const [currentSemitone, setCurrentSemitone] = useState(0)
  const [isPlaying, setIsPlaying] = useState(false)
  const [currentTime, setCurrentTime] = useState(0)
  const [seekTime, setSeekTime] = useState(0)
  const [isSeeking, setIsSeeking] = useState(false)
  const audioRefs = useRef<{ [key: number]: HTMLAudioElement }>({})
  const videoRef = useRef<HTMLVideoElement>(null) // Reference to the video element

  const allSemitoneConversionsComplete = data?.track.semitones.every(
    ({ status }) => status === SemitoneStatus.Completed,
  )

  // Preload all audio files once data is available
  useEffect(() => {
    if (allSemitoneConversionsComplete) {
      const semitoneShifts = []
      for (let i = -10; i <= 10; i++) {
        semitoneShifts.push(i)
      }

      semitoneShifts.forEach((semitone) => {
        const semitoneLabel = semitone >= 0 ? `+${semitone}` : `${semitone}`
        const fileName =
          semitone === 0 ? `${trackId}.mp3` : `${trackId}_${semitoneLabel}_ST.mp3`
        const fileUrl = `/media/${fileName}`

        const audio = new Audio(fileUrl)
        audioRefs.current[semitone] = audio
      })

      setPreloadingAudio(false)
    }
  }, [allSemitoneConversionsComplete, data, trackId])

  // Update currentTime when the audio is playing
  useEffect(() => {
    const audio = audioRefs.current[currentSemitone]
    if (audio) {
      const updateTime = () => {
        setCurrentTime(audio.currentTime)
        if (!isSeeking) {
          setSeekTime(audio.currentTime)
        }
      }

      const handleEnded = () => {
        setIsPlaying(false)
        setCurrentTime(0)
        setSeekTime(0)
        audio.currentTime = 0
        if (videoRef.current) {
          videoRef.current.currentTime = 0
        }
      }

      audio.addEventListener("timeupdate", updateTime)
      audio.addEventListener("ended", handleEnded)
      return () => {
        audio.removeEventListener("timeupdate", updateTime)
        audio.removeEventListener("ended", handleEnded)
      }
    }
  }, [currentSemitone, isSeeking])

  // Handle play/pause functionality
  const handlePlayPause = () => {
    const audio = audioRefs.current[currentSemitone]
    if (audio) {
      if (isPlaying) {
        audio.pause()
        if (videoRef.current) {
          videoRef.current.currentTime = audio.currentTime
          videoRef.current.pause()
        }
        setIsPlaying(false)
      } else {
        audio.currentTime = currentTime
        audio.play()
        if (videoRef.current) {
          videoRef.current.currentTime = audio.currentTime
          videoRef.current.play()
        }
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
      setSeekTime(time)

      nextAudio.currentTime = time
      if (isPlaying) {
        nextAudio.play()
      }
      if (videoRef.current) {
        videoRef.current.currentTime = time
      }
      setCurrentSemitone(newSemitone)
    }
  }

  // Handle seeking in the audio
  const handleSeekChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const time = parseFloat(e.target.value)
    setSeekTime(time)
    setIsSeeking(true)
  }

  const handleSeekEnd = () => {
    const audio = audioRefs.current[currentSemitone]
    if (audio) {
      audio.currentTime = seekTime
      setCurrentTime(seekTime)
      if (videoRef.current) {
        videoRef.current.currentTime = seekTime
      }
    }
    setIsSeeking(false)
  }

  useEffect(() => {
    const audioRefsCurrent = audioRefs.current
    return () => {
      Object.values(audioRefsCurrent).forEach((audio) => {
        audio.pause()
      })
      videoRef.current?.pause() // Pause the video on unmount
      setIsPlaying(false)
    }
  }, [])

  const handleDownload = () => {
    const semitoneLabel =
      currentSemitone >= 0 ? `+${currentSemitone}` : `${currentSemitone}`
    const fileName =
      currentSemitone === 0 ? `${trackId}.mp3` : `${trackId}_${semitoneLabel}_ST.mp3`
    const fileUrl = `/media/${fileName}`

    // Create a temporary anchor element to initiate download
    const link = document.createElement("a")
    link.href = fileUrl
    link.download = `${data?.track.name}_${semitoneLabel}.mp3`
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

  if (!allSemitoneConversionsComplete && data?.track) return <Track track={data?.track} />

  const playingAudio = audioRefs.current[currentSemitone]
  const playTime = (playingAudio && playingAudio.currentTime) || seekTime
  const duration = (playingAudio && playingAudio.duration) || 0

  const minSemitoneAvailable =
    data?.track.semitones.reduce((min, { shift }) => Math.min(min, shift), 0) || -6
  const maxSemitoneAvailable =
    data?.track.semitones.reduce((max, { shift }) => Math.max(max, shift), 0) || 4

  return (
    <div>
      <Link href="/" className="text-blue-500">
        {"< "} back
      </Link>
      <div className="mt-2 text-xl font-bold">{data?.track.name}</div>
      <div className="text-sm text-cyan-500">{data?.track.youtubeUrl}</div>
      <hr className="mt-6 border-zinc-700" />
      <div className="mt-4">
        <div>
          <strong className="font-bold text-xl">
            Current Semitone:{" "}
            {currentSemitone > 0 ? `+${currentSemitone}` : currentSemitone}
          </strong>
        </div>
        <video
          ref={videoRef} // Reference to the video element
          src={`/media/${trackId}.mp4`}
          controls={false}
          muted
          className="mt-4"
        />
        <div className="my-4">
          <button
            onClick={() => changeSemitone(-1)}
            disabled={currentSemitone <= minSemitoneAvailable}
            className="px-4 py-2 mr-2 bg-blue-500 hover:bg-blue-600 text-white rounded disabled:opacity-50"
          >
            -
          </button>
          <button
            onClick={() => changeSemitone(1)}
            disabled={currentSemitone >= maxSemitoneAvailable}
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
            onChange={handleSeekChange}
            onMouseUp={handleSeekEnd}
            onTouchEnd={handleSeekEnd}
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

export default TrackDetail

// Format time in mm:ss
const formatTime = (time: number) => {
  if (isNaN(time)) return "0:00"
  const minutes = Math.floor(time / 60)
  const seconds = Math.floor(time % 60)
  return `${minutes}:${seconds < 10 ? "0" : ""}${seconds}`
}
