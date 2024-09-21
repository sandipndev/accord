import React from "react"
import type { Metadata } from "next"

import "./globals.css"
import ApolloServerWrapper from "@/lib/apollo-client/server-wrapper"

export const metadata: Metadata = {
  title: "Accorde",
  description:
    "Convert YouTube Karaoke tracks, shift the pitch by a few semitones to match your voice, and export the perfect MP3 for your performance",
}

const RootLayout: React.FC<React.PropsWithChildren> = ({ children }) => {
  return (
    <html lang="en" className="antialiased">
      <ApolloServerWrapper>
        <body className="max-w-4xl m-auto p-10 text-white">{children}</body>
      </ApolloServerWrapper>
    </html>
  )
}

export default RootLayout
