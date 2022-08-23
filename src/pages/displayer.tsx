import { invoke } from "@tauri-apps/api";
import { useEffect, useState } from "react";

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
        }, 5000);
        return () => clearInterval(interval);
    }, []);


    return (
        <p data-tauri-drag-region id="lyric">{lyric}</p>
    )
}

export default function Displayer() {
    return (
        <div data-tauri-drag-region className="App-header">
            <IntervalBody />
        </div>
    )
}
