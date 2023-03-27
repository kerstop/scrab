import { useState } from 'react'
import './App.scss'
import ViewWindow from '../ViewWindow/ViewWindow'
import Controls from '../Controls/Controls'

function App() {

  return (
    <div className="App">
      <ViewWindow/>
      <Controls/>
    </div>
  )
}

export default App
