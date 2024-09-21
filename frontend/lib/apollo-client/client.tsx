"use client"

import { ApolloLink, HttpLink } from "@apollo/client"
import {
  ApolloClient,
  ApolloNextAppProvider,
  InMemoryCache,
  SSRMultipartLink,
} from "@apollo/experimental-nextjs-app-support"

export type ClientConfig = {
  backendUrl: string
}

const makeClient = ({ backendUrl }: ClientConfig) => {
  const httpLink = new HttpLink({
    uri: backendUrl,
    fetchOptions: { cache: "no-store" },
  })

  const cache = new InMemoryCache()

  const link =
    typeof window === "undefined"
      ? ApolloLink.from([new SSRMultipartLink({ stripDefer: true }), httpLink])
      : httpLink

  return new ApolloClient({
    link,
    cache,
    defaultOptions: {
      query: { fetchPolicy: "no-cache" },
      watchQuery: { fetchPolicy: "no-cache" },
    },
  })
}

const ApolloWrapper = ({
  config,
  children,
}: {
  config: ClientConfig
  children: React.ReactNode
}) => {
  const client = makeClient(config)

  return (
    <ApolloNextAppProvider makeClient={() => client}>{children}</ApolloNextAppProvider>
  )
}

export default ApolloWrapper
