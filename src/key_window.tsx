import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import App from './KeyWindow'
import './index.css'

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <App />
  </StrictMode>,
)
