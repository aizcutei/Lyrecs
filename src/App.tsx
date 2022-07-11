import { useState } from 'react'
import logo from './logo.svg'
import './App.css'
import { appWindow } from '@tauri-apps/api/window'
import { invoke } from '@tauri-apps/api'

function App() {
  let lyricInLine = 'Say "Hello" to my little world!!'


  invoke('connect_test', {text: "backend"})
      .then((text) => {
        console.log(text)
      })

  invoke('get_next_inline_lyric',{})
      .then((text) => {
        console.log(text)
      })

  return (
    <div className="App">
      <header data-tauri-drag-region className="App-header">
        
        <p data-tauri-drag-region id="lyric">{lyricInLine}</p>
      </header>
    </div>
  )
}

export default App
