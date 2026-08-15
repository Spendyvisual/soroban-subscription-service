import { Routes, Route, Link } from 'react-router-dom';
import { ErrorBoundary } from './components/ErrorBoundary';

function Landing() {
  return (
    <div className="container" style={{ textAlign: 'center', marginTop: '100px' }}>
      <h1 style={{ fontSize: '3rem', marginBottom: '1rem' }}>Soroban Subscription Service</h1>
      <p style={{ color: 'var(--text-secondary)', marginBottom: '3rem', fontSize: '1.25rem' }}>
        A native, standardized, reusable primitive for recurring subscription payments on Stellar.
      </p>
      <div style={{ display: 'flex', gap: '1rem', justifyContent: 'center' }}>
        <Link to="/provider" className="btn-primary">Provider Dashboard</Link>
        <Link to="/portal" className="btn-primary" style={{ backgroundColor: 'var(--bg-elevated)', border: '1px solid var(--border-color)' }}>Subscriber Portal</Link>
      </div>
    </div>
  );
}

function ProviderDashboard() {
  return (
    <div className="container" style={{ padding: '2rem 0' }}>
      <h2>Provider Dashboard</h2>
      <p style={{ color: 'var(--text-secondary)' }}>Manage your plans and view revenue here.</p>
    </div>
  );
}

function SubscriberPortal() {
  return (
    <div className="container" style={{ padding: '2rem 0' }}>
      <h2>Subscriber Portal</h2>
      <p style={{ color: 'var(--text-secondary)' }}>Browse plans and manage your active subscriptions.</p>
    </div>
  );
}

function App() {
  return (
    <Routes>
      <Route path="/" element={<Landing />} />
      <Route
        path="/provider/*"
        element={
          <ErrorBoundary>
            <ProviderDashboard />
          </ErrorBoundary>
        }
      />
      <Route
        path="/portal/*"
        element={
          <ErrorBoundary>
            <SubscriberPortal />
          </ErrorBoundary>
        }
      />
    </Routes>
  );
}

export default App;
