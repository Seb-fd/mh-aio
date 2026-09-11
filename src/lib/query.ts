/**
 * Shared TanStack Query policy for Tauri reference-data lists.
 * Datasets are immutable for the whole app session (the bundled seed only
 * changes between releases), so cached lists stay fresh forever and repeat
 * visits — including back navigation — render instantly with zero IPC.
 */

/** Retry preserving the boot-time 'Preparing database…' backoff (max ~8s). */
export function dbRetry(failureCount: number, err: unknown): boolean {
  if (failureCount >= 6) return false
  const msg = err instanceof Error ? err.message : String(err)
  return msg.includes('state not managed')
}

/** 400ms, 800ms, … linear backoff matching the legacy manual retry loop. */
export function dbRetryDelay(attempt: number): number {
  return 400 * (attempt + 1)
}

/** 'Preparing database…' while seed-blocked retries are still in flight. */
export function dbPreparing(isPending: boolean, failureCount: number): boolean {
  return isPending && failureCount > 0
}

/** Error text for list pages: preparing state while retrying, else the failure. */
export function dbErrorText(
  isPending: boolean,
  failureCount: number,
  error: unknown,
): string | null {
  if (dbPreparing(isPending, failureCount)) return 'Preparing database...'
  if (isPending || error == null) return null
  return error instanceof Error ? error.message : String(error)
}

export const dbCache = {
  staleTime: Infinity,
  gcTime: 30 * 60_000,
} as const
