import { Component, type ErrorInfo, type ReactNode } from 'react';

interface ErrorBoundaryProps {
  children: ReactNode;
  /**
   * Optional custom fallback. Can be a fixed node, or a render function
   * that receives the caught error and a reset callback (e.g. to let the
   * user retry without a full page reload).
   */
  fallback?: ReactNode | ((error: Error, reset: () => void) => ReactNode);
  /** Called with the error and React's component stack, e.g. for logging. */
  onError?: (error: Error, errorInfo: ErrorInfo) => void;
}

interface ErrorBoundaryState {
  error: Error | null;
}

/**
 * Catches render/lifecycle errors in its subtree and shows a fallback UI
 * instead of unmounting the whole app (Issue #39).
 *
 * Error boundaries only catch errors during rendering, in lifecycle
 * methods, and in constructors of the tree below them — not in event
 * handlers, async code, or errors thrown in the boundary itself. Wrap
 * top-level routed sections (Provider Dashboard, Subscriber Portal) so a
 * crash in one doesn't take down the whole app shell.
 */
export class ErrorBoundary extends Component<ErrorBoundaryProps, ErrorBoundaryState> {
  state: ErrorBoundaryState = { error: null };

  static getDerivedStateFromError(error: Error): ErrorBoundaryState {
    return { error };
  }

  componentDidCatch(error: Error, errorInfo: ErrorInfo): void {
    this.props.onError?.(error, errorInfo);
  }

  reset = (): void => {
    this.setState({ error: null });
  };

  render(): ReactNode {
    const { error } = this.state;
    if (!error) {
      return this.props.children;
    }

    const { fallback } = this.props;
    if (typeof fallback === 'function') {
      return fallback(error, this.reset);
    }
    if (fallback) {
      return fallback;
    }

    return <DefaultErrorFallback error={error} onReset={this.reset} />;
  }
}

function DefaultErrorFallback({ error, onReset }: { error: Error; onReset: () => void }): ReactNode {
  return (
    <div
      className="card"
      role="alert"
      style={{ margin: '2rem auto', maxWidth: 480, textAlign: 'center' }}
    >
      <h2 style={{ marginBottom: '0.75rem' }}>Something went wrong</h2>
      <p style={{ color: 'var(--text-secondary)', marginBottom: '1.5rem' }}>
        {error.message || 'An unexpected error occurred while rendering this section.'}
      </p>
      <button type="button" className="btn-primary" onClick={onReset}>
        Try again
      </button>
    </div>
  );
}
