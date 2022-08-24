import { WebviewWindow } from '@tauri-apps/api/window'
import { useState } from 'react'

export default function Setting() {

    let [focusClassName, setFocusClassName] = useState("titlebar focus")

    FocusMonitor().then( result => {
        if (result) {
            setFocusClassName("titlebar focus")
        }else{
            setFocusClassName("titlebar")
        }
    })

    return (<>
        <div>
            <div data-tauri-drag-region className={focusClassName}>
                <div className="traffic-lights">
                    <button className="traffic-light traffic-light-close" id="close" onClick={() => Close()}></button>
                    <button className="traffic-light traffic-light-minimize" id="minimize" onClick={() => Minimize()}></button>
                    <button className="traffic-light traffic-light-maximize" id="maximize" onClick={() => Maxinize()}></button>
                </div>
            </div>
        </div>

        <body className="setting-body">
            <div className="setting-container">
                <p>Setting</p>
            </div>
        </body>
        </>
    )
}

function Close() {

    const settingWindow = WebviewWindow.getByLabel('setting')

    if (settingWindow) {
        settingWindow.close()
    }
}

async function Maxinize() {

    const settingWindow = WebviewWindow.getByLabel('setting')

    if (settingWindow) {
        if (await settingWindow.isMaximized()) {
            settingWindow.toggleMaximize()
        }else{
            settingWindow.toggleMaximize()
        }
    }
}

function Minimize() {
    const settingWindow = WebviewWindow.getByLabel('setting')

    if (settingWindow) {
        settingWindow.minimize()
    }
}

async function FocusMonitor() {
    let [focus, setFocus] = useState(false)

    const settingWindow = WebviewWindow.getByLabel('setting')

    if (settingWindow) {
        const unlisten = await settingWindow.onFocusChanged(({ payload: focused }) => {
            setFocus(focused)
        });
    }
    return focus
}
