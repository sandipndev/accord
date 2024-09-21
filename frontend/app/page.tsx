import { AllProcesses } from "@/components/all-processes"
import { SubmitProcess } from "@/components/submit-process"

const Home: React.FC = () => {
  return (
    <>
      <SubmitProcess />
      <AllProcesses />
    </>
  )
}

export default Home
