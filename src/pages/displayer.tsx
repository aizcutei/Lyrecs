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
    let lyricNew: Lyric = {
        duration: 1,
        line: {
            length: 0,
            text: "Hello",
            translation: "",
            pronunciation: "",
            word_line: [],
        },
        start: 0
    }
    const [lyric, setLyric] = useState(lyricNew)
    useEffect(() => {
        let interval = setInterval(() => {
        invoke('get_next_inline_lyrics', { fixTime : 0.2 } )
            .then((text) => {
            //if (text == "") {
            //    setLyric("♬ ~ ~ ~");
            //}
            //console.log(text)
            let lyricBody: Lyric = JSON.parse(text as string)
            setLyric(lyricBody);
            }
            )
            .catch((err) => {
            console.log(err);
            }
            );
        }, lyric.duration * 1000);
        return () => clearInterval(interval);
    }, []);


    return (
        <>
            <div data-tauri-drag-region>
                    <p data-tauri-drag-region
                        data-text={lyric.line.text}
                    className={'relative p-1 text-8 whitespace-nowrap text-white font-900 tracking-widest text-outline-1-4-#6ee7b7 after:text-8 after:absolute after:p-1 after:top-0 after:left-0 after:overflow-hidden after:w-49% after:text-gray after:font-900 after:tracking-widest after:text-outline-2-5-yellow after:content-text'}>{lyric.line.text}</p>
            </div>
        </>
    )
}

export default function Displayer() {
    return (<>
        <div data-tauri-drag-region className="flex items-center justify-center">
            <IntervalBody />
        </div>
        </>
    )
}

interface Lyric {
    duration: number,
    line: {
        length: number,
        text: string,
        translation: string,
        pronunciation: string,
        word_line: number[],
    },
    start: number
}
