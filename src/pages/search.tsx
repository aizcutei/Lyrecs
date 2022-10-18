import {
    TableContainer,
    TextField,
    Table,
    TableHead,
    Select,
    TableRow,
    TableBody,
    TableCell,
    Button,
    MenuItem,
    Grid,
    createTheme,
} from "@mui/material";
import Paper from "@mui/material/Paper";
import { invoke } from "@tauri-apps/api";
import React, { useRef, useState } from "react";

interface SearchResult {
    title: string;
    artist: string;
    source: string;
}

function createData(
    title: string,
    artist: string,
    source: string,
    preview: string
) {
    return { title, artist, source, preview };
}

const rows: string[] = [];

function doEvent(obj, event) {
    var event = new Event(event, { target: obj, bubbles: true });
    return obj ? obj.dispatchEvent(event) : false;
}

export default function Search() {
    let title = "";
    let artist = "";
    const [source, setSource] = useState("");
    const [titleRef, artistRef] = [
        useRef<HTMLInputElement>(),
        useRef<HTMLInputElement>(),
    ];
    const [searchResults, setSearchResults] = useState([{} as SearchResult]);
    return (
        <div>
            <Grid container justifyContent="space-around" alignItems="flex-end">
                <Grid item xs={12} sm={4} md={4} lg={5} xl={6}>
                    <TextField
                        id="search_song_title"
                        label="Title"
                        variant="filled"
                        inputRef={titleRef}
                        size="small"
                        fullWidth
                    />
                </Grid>
                <Grid item xs={12} sm={4} md={4} lg={5} xl={6}>
                    <TextField
                        id="search_song_artist"
                        label="Artist"
                        variant="filled"
                        inputRef={artistRef}
                        size="small"
                        fullWidth
                    />
                </Grid>
                <Grid item xs={12} sm={2} md={2} lg={1} xl={6}>
                    <Select
                        id="search_source"
                        value={source}
                        label="Source"
                        onChange={(e) => {
                            setSource(e.target.value);
                            console.log(source);
                        }}
                        size="small"
                        fullWidth
                    >
                        <MenuItem value={1}>Netease</MenuItem>
                        <MenuItem value={2}>QQMusic</MenuItem>
                        <MenuItem value={3}>Kugou</MenuItem>
                    </Select>
                </Grid>
                {/* A search button that is a little smaller than the textfield */}
                <Grid item xs={12} sm={2} md={1} lg={1} xl={6}>
                    <Button
                        variant="outlined"
                        onClick={() => {
                            console.log(
                                titleRef.current?.value,
                                artistRef.current?.value
                            );
                            invoke("search", {
                                title: titleRef.current?.value,
                                artist: artistRef.current?.value,
                            }).then((value) => {
                                let searchResults: SearchResult[] = JSON.parse(
                                    value as string
                                );
                                setSearchResults(searchResults);
                                console.log(value);
                            });
                        }}
                    >
                        Search
                    </Button>
                </Grid>
            </Grid>
            <Grid container justifyContent="space-around">
                <Grid item xs={8} sm={9} md={9} lg={10} xl={10}>
                    <CompactTable
                        title={["Title", "Artist", "Source"]}
                        data={searchResults}
                    ></CompactTable>
                </Grid>
                <Grid item xs={4} sm={3} md={3} lg={2} xl={2}>
                    <TextField
                        id="lyric_preview"
                        inputProps={{ readOnly: true }}
                        multiline
                        fullWidth
                        hidden
                    ></TextField>
                </Grid>
            </Grid>
        </div>
    );
}

//title: String[], data: Object[]
export function CompactTable(props: { title: String[]; data: Object[] }) {
    let [currentSelected, setCurrentSelected] = useState(-1);
    return (
        <TableContainer component={Paper}>
            <Table size="small">
                <TableHead>
                    <TableRow>
                        {Object.values(props.title).map((item) => (
                            <TableCell>{item}</TableCell>
                        ))}
                    </TableRow>
                </TableHead>
                <TableBody>
                    {props.data.map((item, index) => {
                        let fields = Object.values(item);
                        return (
                            <TableRow
                                key={index}
                                selected={currentSelected == index}
                                hover
                                onClick={(e) => {
                                    setCurrentSelected(index);
                                    invoke("search_lyric", {
                                        index: index,
                                    }).then((text) => {
                                        var preview =
                                            document.getElementById(
                                                "lyric_preview"
                                            );
                                        preview.value = text as string;
                                        doEvent(preview, "input");
                                    });
                                }}
                            >
                                {fields.map((item) => (
                                    <TableCell>{item}</TableCell>
                                ))}
                            </TableRow>
                        );
                    })}
                </TableBody>
            </Table>
        </TableContainer>
    );
}
