import { describe, expect, test } from 'vitest';
import {
  formatCountdown,
  formatLedgerDate,
  formatLedgerDateTime,
  formatRelativeLedgerTime
} from './date';

const T0 = 1_736_899_200; // 2025-01-15T00:00:00Z

describe('formatLedgerDate', () => {
  test('formats a unix-seconds timestamp as a localized date', () => {
    expect(formatLedgerDate(T0)).toBe('Jan 15, 2025');
  });

  test('accepts bigint and numeric-string input', () => {
    expect(formatLedgerDate(BigInt(T0))).toBe('Jan 15, 2025');
    expect(formatLedgerDate(String(T0))).toBe('Jan 15, 2025');
  });

  test('rejects negative timestamps', () => {
    expect(() => formatLedgerDate(-1)).toThrow(RangeError);
  });
});

describe('formatLedgerDateTime', () => {
  test('includes time of day', () => {
    const result = formatLedgerDateTime(T0);
    expect(result).toContain('Jan 15, 2025');
    expect(result).toMatch(/\d{1,2}:\d{2}\s?(AM|PM)/);
  });
});

describe('formatRelativeLedgerTime', () => {
  test('returns "just now" for a timestamp within 30 seconds', () => {
    expect(formatRelativeLedgerTime(T0, T0 + 10)).toBe('just now');
    expect(formatRelativeLedgerTime(T0, T0 - 10)).toBe('just now');
  });

  test('formats a past timestamp in minutes', () => {
    expect(formatRelativeLedgerTime(T0, T0 + 120)).toBe('2 minutes ago');
  });

  test('formats a future timestamp in hours', () => {
    expect(formatRelativeLedgerTime(T0, T0 - 3600 * 3)).toBe('in 3 hours');
  });

  test('formats a past timestamp in days', () => {
    expect(formatRelativeLedgerTime(T0, T0 + 86400 * 2)).toBe('2 days ago');
  });
});

describe('formatCountdown', () => {
  test('shows days and hours when more than a day remains', () => {
    expect(formatCountdown(T0 + 86400 * 2 + 3600 * 5, T0)).toBe('2d 5h');
  });

  test('shows hours and minutes when less than a day remains', () => {
    expect(formatCountdown(T0 + 3600 * 3 + 60 * 20, T0)).toBe('3h 20m');
  });

  test('shows minutes when less than an hour remains', () => {
    expect(formatCountdown(T0 + 60 * 45, T0)).toBe('45m');
  });

  test('shows <1m for sub-minute remainders', () => {
    expect(formatCountdown(T0 + 30, T0)).toBe('<1m');
  });

  test('shows expired for a timestamp at or before the reference', () => {
    expect(formatCountdown(T0, T0)).toBe('expired');
    expect(formatCountdown(T0 - 100, T0)).toBe('expired');
  });
});
