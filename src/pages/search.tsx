import { invoke } from "@tauri-apps/api";
import React, { useRef, useState } from "react";

interface SearchResult {
  title: string;
  artist: string;
  source: string;
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
  const searchInputRef = useRef(null);
  const [searchResults, setSearchResults] = useState([]);
  return (
    <div className="h-full w-full">
      <div className="grid grid-cols-12 gap-2 content-center">
        <div className="col-span-8 justify-self-stretch">
          <input
            id="search_song_title"
            className="w-full self-end h-full px-0.5 my-0.4 mx-0.5 border border-gray rounded bg-gray-100"
            type="text"
            ref={searchInputRef}
          />
        </div>

        <div className="col-span-2 justify-self-stretch">
          <select
            id="search_source"
            className="w-full self-end h-full border mx-0.5 my-0.8 border-gray rounded"
            onChange={(e) => {
              setSource(e.target.value);
              console.log(source);
            }}
          >
            <option value={1}>Netease</option>
            <option value={2}>QQ</option>
            <option value={3}>Kugou</option>
          </select>
        </div>
        {/* A search button that is a little smaller than the textfield */}
        <div className="col-span-2 justify-self-stretch">
          <button
            type="button"
            className="transition duration-100 active:bg-blue-800  bg-blue-500 w-full h-full my-0.8 self-stretch border rounded text-white border-blue-600 hover:bg-blue-8 focus:ring"
            onClick={() => {
              console.log(searchInputRef.current?.value);
              invoke("search", {
                content: searchInputRef.current?.value,
              }).then((value) => {
                let searchResults: SearchResult[] = JSON.parse(value as string);
                let s: String[][] = searchResults.map((i) => {
                  return [i.title, i.artist, i.source];
                });
                setSearchResults(s);

                console.log(value);
              });
            }}
          >
            Search
          </button>
        </div>

        <div className="col-span-8 w-full mx-1 my-0.5 border  justify-self-stretch">
          <CompactTable
            title={["Title", "Artist", "Source"]}
            data={searchResults}
          ></CompactTable>
        </div>
        <div className="col-span-4  w-full">
          <textarea
            id="lyric_preview"
            className="self-center w-full text-left rounded text-white py-0 bg-blue-300 h-full mx-0.45"
            readOnly
          ></textarea>
        </div>
      </div>
    </div>
  );
}

//title: String[], data: Object[]
export function CompactTable(props: { title: String[]; data: String[][] }) {
  let [currentSelected, setCurrentSelected] = useState(-1);
  const selectedStyle = {
    backgroundColor: "rgba(165, 243, 252, 255)",
  };
  return (
    <table className="w-full h-full  text-left rounded">
      <thead className="border-b-1 border-black">
        <tr>
          {props.title.map((item) => (
            <th className="p-2">{item}</th>
          ))}
        </tr>
      </thead>
      <tbody id="compact-table-body">
        {props.data.map((ritem, index) => {
          return (
            <tr
              className="hover:bg-cyan-100"
              key={index}
              style={currentSelected == index ? selectedStyle : {}}
              //   selected={currentSelected == index}
              onClick={(e) => {
                setCurrentSelected(index);
                invoke("search_lyric", {
                  index: index,
                }).then((text) => {
                  var preview = document.getElementById("lyric_preview");
                  preview.value = text as string;
                  doEvent(preview, "input");
                });
              }}
            >
              {ritem.map((item) => (
                <td className="p-1">{item}</td>
              ))}
            </tr>
          );
        })}
      </tbody>
    </table>
  );
}
