import { useState, useEffect } from 'react'

interface HealthStatus {
  healthy: boolean
}

function App() {
  const [health, setHealth] = useState<HealthStatus | null>(null)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    fetch('/api/health')
      .then((res) => res.json())
      .then((data) => setHealth(data))
      .catch((err) => setError(err.message))
  }, [])

  return (
    <div style={{ padding: '2rem', fontFamily: 'system-ui, sans-serif' }}>
      <h1>{{project_name}}</h1>
      <p>Welcome to your new project!</p>

      <h2>Backend Health</h2>
      {error ? (
        <p style={{ color: 'red' }}>Error: {error}</p>
      ) : health ? (
        <p style={{ color: health.healthy ? 'green' : 'red' }}>
          Status: {health.healthy ? 'Healthy' : 'Unhealthy'}
        </p>
      ) : (
        <p>Loading...</p>
      )}
    </div>
  )
}

export default App
