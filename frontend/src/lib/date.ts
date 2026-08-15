/**
 * Ledger timestamp formatting helpers (Issue #44).
 *
 * On-chain timestamps (`timestamp`, `created_at`, `next_billing_at`, etc.)
 * are Soroban's `ledger().timestamp()`: a `u64` count of seconds since the
 * Unix epoch. These helpers convert that into localized, human-readable
 * strings for the provider dashboard and subscriber portal.
 */

/** Anything a ledger timestamp might arrive as from chain/API responses. */
export type LedgerTimestamp = bigint | number | string;

function toEpochSeconds(timestamp: LedgerTimestamp): number {
  if (typeof timestamp === 'bigint') {
    if (timestamp < 0n) throw new RangeError(`Ledger timestamp cannot be negative: ${timestamp}`);
    return Number(timestamp);
  }
  if (typeof timestamp === 'number') {
    if (!Number.isFinite(timestamp) || timestamp < 0) {
      throw new RangeError(`Invalid ledger timestamp: ${timestamp}`);
    }
    return timestamp;
  }
  const parsed = Number(timestamp);
  if (!Number.isFinite(parsed) || parsed < 0) {
    throw new RangeError(`Invalid ledger timestamp: "${timestamp}"`);
  }
  return parsed;
}

function toDate(timestamp: LedgerTimestamp): Date {
  return new Date(toEpochSeconds(timestamp) * 1000);
}

/**
 * Format a ledger timestamp as a localized date, e.g. "Jan 15, 2026".
 */
export function formatLedgerDate(
  timestamp: LedgerTimestamp,
  locale = 'en-US'
): string {
  return new Intl.DateTimeFormat(locale, {
    year: 'numeric',
    month: 'short',
    day: 'numeric'
  }).format(toDate(timestamp));
}

/**
 * Format a ledger timestamp as a localized date + time, e.g.
 * "Jan 15, 2026, 3:45 PM".
 */
export function formatLedgerDateTime(
  timestamp: LedgerTimestamp,
  locale = 'en-US'
): string {
  return new Intl.DateTimeFormat(locale, {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
    hour: 'numeric',
    minute: '2-digit'
  }).format(toDate(timestamp));
}

/**
 * Format a ledger timestamp relative to now (or a supplied reference time),
 * e.g. "in 3 days", "2 hours ago", "just now". Falls back to an absolute
 * date once the gap exceeds a month, since "47 days ago" is less useful
 * than a calendar date at that distance.
 */
export function formatRelativeLedgerTime(
  timestamp: LedgerTimestamp,
  referenceEpochSeconds: number = Math.floor(Date.now() / 1000),
  locale = 'en-US'
): string {
  const target = toEpochSeconds(timestamp);
  const diffSeconds = target - referenceEpochSeconds;
  const absSeconds = Math.abs(diffSeconds);

  const UNITS: Array<[Intl.RelativeTimeFormatUnit, number]> = [
    ['year', 31536000],
    ['month', 2592000],
    ['day', 86400],
    ['hour', 3600],
    ['minute', 60],
    ['second', 1]
  ];

  if (absSeconds < 30) {
    return 'just now';
  }

  const rtf = new Intl.RelativeTimeFormat(locale, { numeric: 'auto' });
  for (const [unit, secondsInUnit] of UNITS) {
    if (absSeconds >= secondsInUnit || unit === 'second') {
      const value = Math.round(diffSeconds / secondsInUnit);
      return rtf.format(value, unit);
    }
  }
  return 'just now';
}

/**
 * Format the remaining time until a future ledger timestamp as a compact
 * countdown string, e.g. "2d 4h", "45m", "expired". Intended for
 * next-billing-date / grace-period countdowns in the UI.
 */
export function formatCountdown(
  targetTimestamp: LedgerTimestamp,
  referenceEpochSeconds: number = Math.floor(Date.now() / 1000)
): string {
  const target = toEpochSeconds(targetTimestamp);
  const remaining = target - referenceEpochSeconds;

  if (remaining <= 0) {
    return 'expired';
  }

  const days = Math.floor(remaining / 86400);
  const hours = Math.floor((remaining % 86400) / 3600);
  const minutes = Math.floor((remaining % 3600) / 60);

  if (days > 0) return `${days}d ${hours}h`;
  if (hours > 0) return `${hours}h ${minutes}m`;
  if (minutes > 0) return `${minutes}m`;
  return '<1m';
}
