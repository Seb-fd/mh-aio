import { QueryClient } from '@tanstack/query-core'

let client: QueryClient | undefined

/** Singleton QueryClient for Tauri invoke caching (offline-friendly, in-memory). */
export function getQueryClient(): QueryClient {
  if (!client) {
    client = new QueryClient({
      defaultOptions: {
        queries: {
          staleTime: 60_000,
          gcTime: 5 * 60_000,
          retry: 1,
          refetchOnWindowFocus: false,
          refetchOnReconnect: false,
        },
      },
    })
  }
  return client
}
