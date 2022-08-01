import { useState, useEffect, useReducer } from 'react'
import logo from './logo.svg'
import './App.css'
import { appWindow } from '@tauri-apps/api/window'
import { invoke } from '@tauri-apps/api'

//old reducer way
{/*
function reducer(state: {nextLyric: string}) {
  let nextLyric = invoke('get_next_inline_lyric')

  invoke('get_next_inline_lyric',{})
    .then((text) => {
      //@ts-ignore
      nextLyric = text;
    })
    .catch((err) => {
      console.log(err);
    }
    );
  console.log("???" + nextLyric)
  return {nextLyric: nextLyric}
}
*/}

function IntervalBody() {
{/*
  const [state, dispatch] = useReducer(reducer, { nextLyric: "Hello" });
  useEffect(() => {
    setInterval(() => {
      dispatch();
    }, 5000);
  }, []);
*/}

  const [lyric, setLyric] = useState("Hello")
  useEffect(() => {
    let interval = setInterval(() => {
      invoke('get_next_inline_lyric', { fixTime : 0.2 } )
        .then((text) => {
          if (text == "") {
            setLyric("♬ ~ ~ ~");
          }
          setLyric(text as string);
        }
        )
        .catch((err) => {
          console.log(err);
        }
        );
    }, 500);
    return () => clearInterval(interval);
  }, []);


  return (
    <p data-tauri-drag-region id="lyric">{lyric}</p>
  )
}

function App() {
  let lyricInLine = 'Say "Hello" to my little world!!'

  invoke('connect_test', {text: "backend"})
      .then((text) => {
        console.log(text)
      })

  return (
    <div className="App">
      <header data-tauri-drag-region className="App-header">
        
        <IntervalBody />
      </header>
    </div>
  )
}

export default App
