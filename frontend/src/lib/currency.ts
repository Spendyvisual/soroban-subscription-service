/**
 * Currency formatting helpers (Issue #43).
 *
 * On-chain amounts (`price_amount`, `amount_charged`, etc.) are stored as
 * `i128` in the smallest token unit ("stroops"). Both XLM and USDC on
 * Stellar use 7 decimal places, so 1 XLM/USDC == 10_000_000 stroops.
 *
 * These helpers only ever deal with `bigint`/numeric-string amounts, never
 * `number`, since JS numbers can silently lose precision above 2^53 and
 * on-chain balances routinely exceed that.
 */

export const STROOPS_PER_UNIT = 10_000_000n; // 10^7, Stellar's fixed asset precision
const STROOPS_DECIMALS = 7;

export type StellarAssetSymbol = 'XLM' | 'USDC';

/** Accepts the numeric-ish shapes an amount might arrive in from chain/API responses. */
export type StroopAmount = bigint | number | string;

function toBigInt(amount: StroopAmount): bigint {
  if (typeof amount === 'bigint') return amount;
  if (typeof amount === 'number') {
    if (!Number.isFinite(amount)) {
      throw new RangeError(`Cannot format non-finite amount: ${amount}`);
    }
    return BigInt(Math.trunc(amount));
  }
  const trimmed = amount.trim();
  if (!/^-?\d+$/.test(trimmed)) {
    throw new RangeError(`Cannot format non-integer stroop amount: "${amount}"`);
  }
  return BigInt(trimmed);
}

/**
 * Convert a raw stroop amount into a decimal display string, e.g.
 * `stroopsToDecimalString(125_000_000n)` -> `"12.5"`.
 *
 * No thousands separators or currency symbol — use `formatCurrency` for a
 * fully display-ready string. Trailing zero fractional digits are trimmed;
 * pass `minFractionDigits` to pad back out (e.g. for a table of amounts
 * that should align on 2 decimals).
 */
export function stroopsToDecimalString(amount: StroopAmount, minFractionDigits = 0): string {
  const stroops = toBigInt(amount);
  const negative = stroops < 0n;
  const abs = negative ? -stroops : stroops;

  const whole = abs / STROOPS_PER_UNIT;
  const fraction = abs % STROOPS_PER_UNIT;

  let fractionStr = fraction.toString().padStart(STROOPS_DECIMALS, '0');
  fractionStr = fractionStr.replace(/0+$/, '');
  if (fractionStr.length < minFractionDigits) {
    fractionStr = fractionStr.padEnd(minFractionDigits, '0');
  }

  const sign = negative ? '-' : '';
  return fractionStr.length > 0 ? `${sign}${whole}.${fractionStr}` : `${sign}${whole}`;
}

/**
 * Format a raw stroop amount as a human-readable "<amount> <SYMBOL>" string,
 * with thousands separators, e.g. `formatCurrency(125_000_000n, 'XLM')` ->
 * `"12.5 XLM"`, `formatCurrency(1_000_000_000_000n, 'USDC')` ->
 * `"100,000 USDC"`.
 */
export function formatCurrency(
  amount: StroopAmount,
  asset: StellarAssetSymbol | string,
  options: { minFractionDigits?: number; locale?: string } = {}
): string {
  const { minFractionDigits = 0, locale = 'en-US' } = options;
  const decimalString = stroopsToDecimalString(amount, minFractionDigits);
  const [wholePart, fractionPart] = decimalString.split('.');

  const negative = wholePart.startsWith('-');
  const wholeDigits = negative ? wholePart.slice(1) : wholePart;
  const groupedWhole = new Intl.NumberFormat(locale, { maximumFractionDigits: 0 }).format(
    BigInt(wholeDigits)
  );

  const numberStr = fractionPart ? `${groupedWhole}.${fractionPart}` : groupedWhole;
  return `${negative ? '-' : ''}${numberStr} ${asset}`;
}

/**
 * Convert a human-entered decimal string (e.g. from a form input, "12.5")
 * into raw stroops. Throws if the input has more than 7 fractional digits
 * (would lose precision) or isn't a valid decimal number.
 */
export function decimalStringToStroops(input: string): bigint {
  const trimmed = input.trim();
  const match = /^(-?)(\d+)(?:\.(\d+))?$/.exec(trimmed);
  if (!match) {
    throw new RangeError(`"${input}" is not a valid decimal amount`);
  }
  const [, sign, wholeStr, fractionStr = ''] = match;
  if (fractionStr.length > STROOPS_DECIMALS) {
    throw new RangeError(
      `"${input}" has more than ${STROOPS_DECIMALS} fractional digits, which Stellar assets cannot represent`
    );
  }
  const paddedFraction = fractionStr.padEnd(STROOPS_DECIMALS, '0');
  const stroops = BigInt(wholeStr) * STROOPS_PER_UNIT + BigInt(paddedFraction || '0');
  return sign === '-' ? -stroops : stroops;
}
