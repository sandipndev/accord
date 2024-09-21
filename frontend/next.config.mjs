/** @type {import('next').NextConfig} */
const nextConfig = {
  // Proxy Graphql and Media to Backend
  rewrites: () => [
    {
      source: "/graphql",
      destination: "http://localhost:8765/graphql",
    },
    {
      source: "/media/:path*",
      destination: "http://localhost:8765/media/:path*",
    },
  ],
};

export default nextConfig;
