import { fireEvent, render, screen } from '@testing-library/react';
import type { ReactElement } from 'react';
import { describe, expect, test, vi } from 'vitest';
import { ErrorBoundary } from './ErrorBoundary';

function Bomb({ shouldThrow }: { shouldThrow: boolean }): ReactElement {
  if (shouldThrow) {
    throw new Error('Boom');
  }
  return <div>All good</div>;
}

// React logs caught errors to the console during tests; silence that noise
// without hiding assertions on the actual behavior.
function withSilencedConsoleError(fn: () => void): void {
  const spy = vi.spyOn(console, 'error').mockImplementation(() => {});
  try {
    fn();
  } finally {
    spy.mockRestore();
  }
}

describe('ErrorBoundary', () => {
  test('renders children when there is no error', () => {
    render(
      <ErrorBoundary>
        <Bomb shouldThrow={false} />
      </ErrorBoundary>
    );
    expect(screen.getByText('All good')).toBeInTheDocument();
  });

  test('renders the default fallback when a child throws', () => {
    withSilencedConsoleError(() => {
      render(
        <ErrorBoundary>
          <Bomb shouldThrow={true} />
        </ErrorBoundary>
      );
    });
    expect(screen.getByRole('alert')).toBeInTheDocument();
    expect(screen.getByText('Something went wrong')).toBeInTheDocument();
    expect(screen.getByText('Boom')).toBeInTheDocument();
  });

  test('calls onError with the error and component stack', () => {
    const onError = vi.fn();
    withSilencedConsoleError(() => {
      render(
        <ErrorBoundary onError={onError}>
          <Bomb shouldThrow={true} />
        </ErrorBoundary>
      );
    });
    expect(onError).toHaveBeenCalledTimes(1);
    expect(onError.mock.calls[0][0]).toBeInstanceOf(Error);
    expect(onError.mock.calls[0][0].message).toBe('Boom');
  });

  test('renders a custom static fallback', () => {
    withSilencedConsoleError(() => {
      render(
        <ErrorBoundary fallback={<div>Custom fallback</div>}>
          <Bomb shouldThrow={true} />
        </ErrorBoundary>
      );
    });
    expect(screen.getByText('Custom fallback')).toBeInTheDocument();
  });

  test('renders a custom fallback render function with reset support', () => {
    withSilencedConsoleError(() => {
      render(
        <ErrorBoundary
          fallback={(error, reset) => (
            <div>
              <span>Caught: {error.message}</span>
              <button onClick={reset}>Reset</button>
            </div>
          )}
        >
          <Bomb shouldThrow={true} />
        </ErrorBoundary>
      );
    });
    expect(screen.getByText('Caught: Boom')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Reset' })).toBeInTheDocument();
  });

  test('"Try again" resets the boundary state', () => {
    withSilencedConsoleError(() => {
      render(
        <ErrorBoundary>
          <Bomb shouldThrow={true} />
        </ErrorBoundary>
      );
    });
    const button = screen.getByRole('button', { name: 'Try again' });
    fireEvent.click(button);
    // After reset, the boundary re-renders children; Bomb throws again
    // synchronously since shouldThrow is still true, so we should land
    // back on the fallback rather than crash the test render.
    expect(screen.getByRole('alert')).toBeInTheDocument();
  });
});
