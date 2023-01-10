import { invoke } from "@tauri-apps/api";
import React, { useRef, useState } from "react";
import { LegacyRef } from "react";

interface SearchResult {
  title: string;
  artist: string;
  source: string;
}

const rows: string[] = [];

function doEvent(obj: HTMLElement, event: string) {
  var e = new Event(event, { bubbles: true });
  return obj ? obj.dispatchEvent(e) : false;
}

export default function Search() {
  let title = "";
  let artist = "";
  const [source, setSource] = useState("");
  const searchInputRef = useRef<HTMLInputElement>(null);
  const [tableSelectIdx, setTableSelectIdx] = useState(-1);
  const [searchResults, setSearchResults] = useState([[""]]);
  const [preview, setPreview] = useState("");
  return (
    <div className="container h-full w-full min-w700px">
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
        <div className="col-span-1  justify-self-stretch">
          <button
            type="button"
            className="transition duration-100 text-0.1 disabled:bg-gray-4 disabled:hover:bg-gray-5  active:bg-blue-800  bg-blue-500 w-full h-full my-0.8 self-stretch border rounded text-white border-blue-600 hover:bg-blue-8 focus:ring"
            onClick={() => {
              console.log(searchInputRef.current?.value);
              invoke("search", {
                content: searchInputRef.current?.value,
              }).then((value) => {
                let searchResults: SearchResult[] = JSON.parse(value as string);
                let s: string[][] = searchResults.map((i) => {
                  return [i.title, i.artist, i.source];
                });
                setSearchResults(s);
                setTableSelectIdx(-1);
                console.log(value);
              });
            }}
          >
            Search
          </button>
        </div>
        <div className="col-span-1 mr-0.8 justify-self-stretch">
          <button
            type="button"
            className="transition duration-100 text-0.1  disabled:bg-gray-4 disabled:hover:bg-gray-5 bg-blue-500 w-full h-full my-0.8 self-stretch border rounded text-white border-blue-600  focus:ring"
            disabled={tableSelectIdx < 0}
            onClick={() => {
              invoke("apply_search_result", {
                index: tableSelectIdx,
              }).catch((e) => {
                console.error(e);
              });
            }}
          >
            Apply
          </button>
        </div>
        <div className="col-span-8 w-full mx-1 my-0.5 border justify-self-stretch">
          <CompactTable
            title={["Title", "Artist", "Source"]}
            data={searchResults}
            setIdx={setTableSelectIdx}
            setPreview={setPreview}
          ></CompactTable>
        </div>
        <div className="col-span-4  w-full">
          <textarea
            id="lyric_preview"
            className="self-center w-full text-left rounded text-white py-0 bg-blue-300 h-full mx-0.45"
            readOnly
            value={preview}
          ></textarea>
        </div>
      </div>
    </div>
  );
}

//title: String[], data: Object[]
export function CompactTable(props: {
  title: String[];
  data: String[][];
  setIdx: React.Dispatch<React.SetStateAction<number>>;
  setPreview: React.Dispatch<React.SetStateAction<string>>;
}) {
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
              onClick={() => {
                setCurrentSelected(index);
                props.setIdx(index);
                invoke("search_lyric", {
                  index: index,
                })
                  .then((text) => {
                    props.setPreview(text as string);
                  })
                  .catch((e) => {
                    props.setPreview("");
                    console.error(e);
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
