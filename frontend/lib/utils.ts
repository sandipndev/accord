export const formatStatus = (status: string) => {
  const lowercased = status.toLowerCase()
  return lowercased.charAt(0).toUpperCase() + lowercased.slice(1)
}
