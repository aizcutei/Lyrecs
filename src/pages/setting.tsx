import { WebviewWindow } from '@tauri-apps/api/window'
import { Store } from 'tauri-plugin-store-api'
import { useRef, useState } from 'react'
import Grid from '@mui/material/Grid'
import { BottomNavigation, BottomNavigationAction, Box, Button, ButtonBase, makeStyles, MenuItem, Paper, Select, Slider, Switch, TextField, useTheme } from '@mui/material'
import { Stack } from '@mui/system'


function DebugSetting() {
    return(
    <div className="">
        <div className="">
            <p id="title">Setting</p>
            <button onClick={() => ReadSetting()}>Read Setting</button>
            <button onClick={() => WriteSetting()}>Write Setting</button>
        </div>
    </div>
    )
}

function WindowSetting() {
    return(<>
        <Grid container alignItems="center" justifyContent="center" spacing={2}>
            <Grid item xs={2}></Grid>
            <Grid item display='flex' justifyContent="center" xs={4}>
                <p>Blur Effect</p>
            </Grid>
            <Grid item display='flex' justifyContent="center" xs={4}>
                <Switch/>
            </Grid>
            <Grid item xs={2}></Grid>

            <Grid item xs={2}></Grid>
            <Grid item display='flex' justifyContent="center" xs={4}>
                <p>Opacity</p>
            </Grid>
            <Grid item display='flex' justifyContent="center" xs={4}>
                <Switch/>
            </Grid>
            <Grid item xs={2}></Grid>

            <Grid item xs={2}></Grid>
            <Grid item display='flex' justifyContent="center" xs={4}>
                <p>Rounded Corner</p>
            </Grid>
            <Grid item display='flex' justifyContent="center" xs={4}>
                <Slider
                valueLabelDisplay="auto"
                step={1}
                min={0}
                max={20}/>
            </Grid>
            <Grid item xs={2}></Grid>

            <Grid item xs={2}></Grid>
            <Grid item display='flex' justifyContent="center" xs={4}>
                <p>Background Image</p>
            </Grid>
            <Grid item display='flex' justifyContent="center" xs={4}>
                <Switch/>
            </Grid>
            <Grid item xs={2}></Grid>

            <Grid item xs={2}></Grid>
            <Grid item display='flex' justifyContent="center" xs={4}>
                <p>Image Path</p>
            </Grid>
            <Grid item display='flex' justifyContent="center" xs={4}>
            <Button variant="contained" component="label">
                Choose File
            <input hidden accept="image/*" multiple type="file" />
            </Button>
            </Grid>
            <Grid item xs={2}></Grid>

            <Grid item xs={2}></Grid>
            <Grid item display='flex' justifyContent="center" xs={4}>
                <p>Background Color</p>
            </Grid>
            <Grid item display='flex' justifyContent="center" xs={4}>
                <Switch/>
            </Grid>
            <Grid item xs={2}></Grid>

            <Grid item xs={2}></Grid>
            <Grid item display='flex' justifyContent="center" xs={4}>
                <p>Choose Color</p>
            </Grid>
            <Grid item display='flex' justifyContent="center" xs={4}>

            </Grid>
            <Grid item xs={2}></Grid>
        </Grid>

    </>)
}


export default function Setting() {
    const [value, setValue] = useState(0)


    return (<>
    <div className='container bg-blueGray/10 p-4'>

        <WindowSetting />


    </div>
    <Paper sx={{ position: 'fixed', bottom: 0, left: 0, right: 0 }} elevation={3}>
            <BottomNavigation
                showLabels
                value={value}
                onChange={(value, newValue) => {
                    setValue(newValue)
                }}>
                <BottomNavigationAction label="Window" />
                <BottomNavigationAction label="Lyric" />
                <BottomNavigationAction label="General" />
            </BottomNavigation>
    </Paper>
    </>)
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

    //Background Related
    backgroundEnabled: boolean,
    backgroundColor: string,
    backgroundImage: string,
    backgroundOpacity: number,

    //Text Related
    textFont: string,
    textSize: number,
    baseTextColor: string,
    baseTextOutlineEnabled: boolean,
    baseTextOutlineColor: string,
    baseTextOutlineSize: number,
    baseTextOutlineBlur: number,
    overTextColor: string,
    overTextOutlineEnabled: boolean,
    overTextOutlineColor: string,
    overTextOutlineSize: number,

    //Backend Related
    proxyEnabled: boolean,
    proxyAddress: string,
    defaultService: string,

}
