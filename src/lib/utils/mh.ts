// Shared helpers for MH display logic — centralizes duplicated maps previously copied across ~6 routes.
// Keep in sync with Rust ordering logic in src-tauri/src/db/queries.rs.
export const SHARP_COLORS: Record<string, string> = {
  red: '#ef4444',
  orange: '#f97316',
  yellow: '#eab308',
  green: '#22c55e',
  blue: '#3b82f6',
  white: '#f8fafc',
  purple: '#a855f7',
}
export const SHARP_COLORS_ARR = [
  '#e74c3c',
  '#ff9800',
  '#f4d03f',
  '#58d68d',
  '#5dade2',
  '#ffffff',
] as const
export const SHARP_LABELS = ['Red', 'Orange', 'Yellow', 'Green', 'Blue', 'White'] as const

export function parseSharpSegments(
  sharpness: string | null,
): { color: string; width: number }[] | null {
  if (!sharpness) return null
  try {
    const arr: number[] = JSON.parse(sharpness)
    if (!Array.isArray(arr)) return null
    const keys = ['red', 'orange', 'yellow', 'green', 'blue', 'white', 'purple'] as const
    return arr
      .map((v, i) => ({
        color: SHARP_COLORS[keys[i] ?? 'red'] ?? '#6b7280',
        width: Number(v) || 0,
      }))
      .filter((s) => s.width > 0)
  } catch {
    return null
  }
}
export function sharpnessValues(raw: string | null | undefined): number[] {
  if (!raw) return []
  try {
    const a = JSON.parse(raw)
    return (Array.isArray(a) ? a.map(Number) : []).slice(0, 6)
  } catch {
    return []
  }
}

export function elementColor(element: string | null | undefined): string {
  if (!element) return 'text-gray-400'
  const e = element.toLowerCase()
  if (e.includes('fire')) return 'text-orange-400'
  if (e.includes('water')) return 'text-blue-400'
  if (e.includes('thunder')) return 'text-yellow-400'
  if (e.includes('ice')) return 'text-cyan-400'
  if (e.includes('dragon')) return 'text-purple-400'
  return 'text-gray-400'
}

export function rankColor(rank: string | null | undefined): string {
  const r = (rank ?? '').toLowerCase()
  if (r === 'g' || r.includes('g rank') || r.includes('master'))
    return 'text-red-400 border-red-900/40 bg-red-950/20'
  if (r === 'high' || r.includes('high'))
    return 'text-orange-400 border-orange-900/40 bg-orange-950/20'
  if (r === 'low' || r.includes('low'))
    return 'text-emerald-400 border-emerald-900/40 bg-emerald-950/20'
  return 'text-gray-400 border-gray-800 bg-gray-900/40'
}

/**
 * Canonical rank → Badge tone mapping (single source of truth).
 * - Low → rankLow (gray), High → rankHigh (blue), G → rankG (yellow),
 *   Master → rankMaster (purple). Unknown ranks → neutral.
 */
export type RankTone = 'rankLow' | 'rankHigh' | 'rankG' | 'rankMaster' | 'neutral'

export function rankTone(rank: string | null | undefined): RankTone {
  const r = (rank ?? '').trim().toLowerCase()
  if (r === 'low') return 'rankLow'
  if (r === 'high') return 'rankHigh'
  if (r === 'g') return 'rankG'
  if (r === 'master') return 'rankMaster'
  return 'neutral'
}

/** Fallback label for missing data (keeps one spelling for future i18n). */
export function fallbackLabel(value: string | null | undefined): string {
  return value ?? 'Unknown'
}

export function slotLabel(slot: string | null | undefined): string {
  if (!slot) return '—'
  const s = slot.toLowerCase()
  if (s === 'head' || s === 'helm') return 'Head'
  if (s === 'chest' || s === 'body' || s === 'mail') return 'Chest'
  if (s === 'arms' || s === 'arm') return 'Arms'
  if (s === 'waist') return 'Waist'
  if (s === 'legs' || s === 'greaves') return 'Legs'
  return slot
}
