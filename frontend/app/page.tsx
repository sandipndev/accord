import { AllProcesses } from "@/components/all-processes"
import { SubmitProcess } from "@/components/submit-process"

const Home: React.FC = () => {
  return (
    <div className="max-w-4xl m-auto p-10">
      <SubmitProcess />
      <AllProcesses />
    </div>
  )
}

export default Home
