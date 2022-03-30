import React from "react";
import ReactJson from "react-json-view";

export function SimpleJson({ src }) {
  return (
    <ReactJson
      src={src}
      name={false}
      theme="rjvdefault"
      enableClipboard={false}
      style={{
        fontFamily:
          "ui-monospace,SFMono-Regular,SF Mono,Consolas,Liberation Mono,Menlo,monospace",
      }}
    />
  );
}
