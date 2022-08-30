import { WebviewWindow } from '@tauri-apps/api/window'
import { Store } from 'tauri-plugin-store-api'
import { useState } from 'react'

export default function Setting() {


    ReadSetting().then( result => {
        console.log(result)
    })


    return (<>
        <body className="">
            <div className="">
                <p>Setting</p>
            </div>
        </body>
        </>
    )
}


async function ReadSetting() {
    const store = new Store('.settings')
    const setting = await store.get('Test-Item')
    return setting
}

interface SettingData {
    //Window Related
    blurEffect: boolean,
    raidus: number,
    opacity: number,

    //Text Related
    textFont: string,
    baseTextSize: number,
    baseTextColor: string,
    baseTextOutlineEnabled: boolean,
    baseTextOutlineColor: string,
    baseTextOutlineSize: number,
    baseTextOutlineBlur: number,
    overTextSize: number,
    overTextColor: string,
    overTextOutlineEnabled: boolean,
    overTextOutlineColor: string,
    overTextOutlineSize: number,

    //Background Related
    backgroundEnabled: boolean,
    backgroundColor: string,
    backgroundImage: string,
    backgroundOpacity: number,

    //Backend Related
    defaultService: string,
}
