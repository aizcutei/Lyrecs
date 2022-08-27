import { useState, useEffect, useReducer } from 'react'
import logo from './logo.svg'
import './App.css'
import { appWindow } from '@tauri-apps/api/window'
import { invoke } from '@tauri-apps/api'
import { BrowserRouter, Routes, Route } from "react-router-dom";
import Setting from './pages/setting'
import Displayer from './pages/displayer'
import Search from './pages/search'

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

function App() {
    let lyricInLine = 'Say "Hello" to my little world!!'

    invoke('connect_test', {text: "backend"})
        .then((text) => {
            console.log(text)
        })

    return (
        <div>
            <header>
                <Routes>
                    <Route path="/" element={<Displayer />} />
                    <Route path="/setting" element={<Setting />} />
                    <Route path="/search" element={<Search />} />
                </Routes>
            </header>
        </div>
    )
}

export default App
