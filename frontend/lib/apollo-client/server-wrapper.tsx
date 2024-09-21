import React from "react"
import { unstable_noStore as noStore } from "next/cache"

import ApolloWrapper, { ClientConfig } from "@/lib/apollo-client/client"

import { env } from "@/env"

const config: ClientConfig = {
  backendUrl: env.NEXT_PUBLIC_BACKEND_URL,
}

const ApolloServerWrapper: React.FC<React.PropsWithChildren> = ({ children }) => {
  noStore()
  return <ApolloWrapper config={config}>{children}</ApolloWrapper>
}

export default ApolloServerWrapper
