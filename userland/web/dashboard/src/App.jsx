import { useState, useEffect } from 'react'
import './App.css'

function App() {
  const [status, setStatus] = useState('SOVEREIGN_OK')
  const [vram, setVram] = useState(0)

  useEffect(() => {
    const interval = setInterval(() => {
      setVram(Math.floor(Math.random() * 1024))
    }, 1000)
    return () => clearInterval(interval)
  }, [])

  return (
    <div className="sovereign-container">
      <header>
        <h1>Σ SIGMAOS SOVEREIGN DASHBOARD</h1>
        <div className="pulse"></div>
      </header>
      
      <main className="grid">
        <div className="card">
          <h3>Kernel Status</h3>
          <p className="green">{status}</p>
        </div>
        
        <div className="card">
          <h3>VRAM Usage</h3>
          <p className="cyan">{vram} MB / 16 GB</p>
        </div>

        <div className="card">
          <h3>Active Silos</h3>
          <p className="gold">124 Shards</p>
        </div>

        <div className="card">
          <h3>Network Logic</h3>
          <p className="blue">Reliable UDP (ACK_ENABLED)</p>
        </div>
      </main>

      <footer>
        <button onClick={() => setStatus('PURGED')}>TRIGGER AMNESIC SCRUB</button>
      </footer>
    </div>
  )
}

export default App
