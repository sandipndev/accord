import { AllTracks } from "@/components/all-tracks"
import { SubmitTrack } from "@/components/submit-track"

const Home: React.FC = () => {
  return (
    <>
      <SubmitTrack />
      <AllTracks />
    </>
  )
}

export default Home
