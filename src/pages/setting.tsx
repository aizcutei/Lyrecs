import { WebviewWindow } from '@tauri-apps/api/window'
import { Store } from 'tauri-plugin-store-api'
import { useState } from 'react'
import { Col, Row, Switch } from 'antd'

export default function Setting() {

    return (<>
    <Row>
        <Col span={24}>
            <h1 className='text-center'>Setting</h1>
        </Col>
    </Row>
    <Row>
        <Col span={4}>
            <p>毛玻璃效果</p>
        </Col>
        <Col span={4}>
            <Switch defaultChecked={true} />
        </Col>x
        <Col span={4}>
            <p>背景颜色</p>
        </Col>
        <Col span={4}>
            <Switch defaultChecked={true} />
        </Col>
        <Col span={4}>
            <p>字体颜色</p>
        </Col>
        <Col span={4}>
            <Switch defaultChecked={true} />
        </Col>
    </Row>
        <body className="">
            <div className="">
                <p id="title">Setting</p>
                <button onClick={() => ReadSetting()}>Read Setting</button>
                <button onClick={() => WriteSetting()}>Write Setting</button>
            </div>
        </body>
        </>
    )
}


async function ReadSetting() {
    const store = new Store('.settings')
    store.load()
    const setting = await store.get('Test-Item')
    if (setting ){
        let t = document.getElementById("title") as HTMLElement
        t.innerHTML = "Setting: " + setting
    }
    console.log(setting)
    return setting
}

function WriteSetting() {
    const store = new Store('.settings')
    store.set('Test-Item', 'Test-Value2')
    store.save()
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
