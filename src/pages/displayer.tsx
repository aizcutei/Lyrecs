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
        <>
            <div data-tauri-drag-region>
                    <p data-tauri-drag-region
                    className={'relative text-10 text-white font-900 tracking-widest text-outline-1-4-#6ee7b7 after:text-10 after:absolute after:top-0 after:left-0 after:overflow-hidden after:w-10 after:text-gray after:font-900 after:tracking-widest after:text-outline-2-5-yellow after:content-text-' + lyric}>{lyric}</p>
            </div>
        </>
    )
}

export default function Displayer() {
    return (<>
        <div data-tauri-drag-region className="flex items-center justify-center w-full h-full bg-white:50">
            <IntervalBody />
        </div>
        </>
    )
}
