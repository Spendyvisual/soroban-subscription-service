import { describe, expect, test } from 'vitest';
import {
  decimalStringToStroops,
  formatCurrency,
  stroopsToDecimalString
} from './currency';

describe('stroopsToDecimalString', () => {
  test('converts whole units', () => {
    expect(stroopsToDecimalString(10_000_000n)).toBe('1');
  });

  test('converts fractional units and trims trailing zeros', () => {
    expect(stroopsToDecimalString(125_000_000n)).toBe('12.5');
  });

  test('preserves full 7-decimal precision', () => {
    expect(stroopsToDecimalString(1n)).toBe('0.0000001');
  });

  test('handles zero', () => {
    expect(stroopsToDecimalString(0n)).toBe('0');
  });

  test('handles negative amounts', () => {
    expect(stroopsToDecimalString(-125_000_000n)).toBe('-12.5');
  });

  test('pads to minFractionDigits when requested', () => {
    expect(stroopsToDecimalString(10_000_000n, 2)).toBe('1.00');
  });

  test('accepts number and numeric-string input', () => {
    expect(stroopsToDecimalString(125_000_000)).toBe('12.5');
    expect(stroopsToDecimalString('125000000')).toBe('12.5');
  });

  test('rejects non-integer strings', () => {
    expect(() => stroopsToDecimalString('12.5')).toThrow(RangeError);
  });
});

describe('formatCurrency', () => {
  test('formats with asset symbol', () => {
    expect(formatCurrency(125_000_000n, 'XLM')).toBe('12.5 XLM');
  });

  test('adds thousands separators', () => {
    expect(formatCurrency(1_000_000_000_000n, 'USDC')).toBe('100,000 USDC');
  });

  test('handles negative amounts with separators', () => {
    expect(formatCurrency(-1_000_000_000_000n, 'USDC')).toBe('-100,000 USDC');
  });

  test('respects minFractionDigits', () => {
    expect(formatCurrency(10_000_000n, 'XLM', { minFractionDigits: 2 })).toBe('1.00 XLM');
  });
});

describe('decimalStringToStroops', () => {
  test('round-trips with stroopsToDecimalString', () => {
    expect(decimalStringToStroops('12.5')).toBe(125_000_000n);
    expect(decimalStringToStroops('1')).toBe(10_000_000n);
    expect(decimalStringToStroops('0.0000001')).toBe(1n);
  });

  test('handles negative input', () => {
    expect(decimalStringToStroops('-12.5')).toBe(-125_000_000n);
  });

  test('rejects more than 7 fractional digits', () => {
    expect(() => decimalStringToStroops('1.00000001')).toThrow(RangeError);
  });

  test('rejects malformed input', () => {
    expect(() => decimalStringToStroops('abc')).toThrow(RangeError);
    expect(() => decimalStringToStroops('1.2.3')).toThrow(RangeError);
  });
});
