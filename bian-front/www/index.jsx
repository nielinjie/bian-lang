import ReactDOM from "react-dom";
import React from "react";

import { ReflexContainer, ReflexSplitter, ReflexElement } from "react-reflex";
import CodeEditor from "@uiw/react-textarea-code-editor";
import { SimpleJson } from "./SimpleJson";

/////////////////////////////////////////////////////////
// Re-Flex Basic vertical layout non-resizable
//
/////////////////////////////////////////////////////////
let wasmAdapter = undefined;

function App() {
  function compute(c) {
    console.log("code is ", c);
    if (wasmAdapter) {
      return wasmAdapter.compute_and_represent(c);
    } else {
      return [{ waiting: "wasm not ready" }, { waiting: "wasm not ready" }];
    }
  }
  const [code, setCode] = React.useState();
  const [result, setResult] = React.useState({ waiting: "coding" });
  return (
    <ReflexContainer orientation="horizontal">
      <ReflexElement>
        <header>
          <span className="title-big">Bian Pad</span>
          {/* <span className="title">a code demo pad for bian-lang</span> */}
          <span className="title-small">
            a code demo pad for bian-lang{" "}
            <a target="_blank" href="https://github.com/nielinjie/bian-lang">
              https://github.com/nielinjie/bian-lang
            </a>
          </span>
        </header>
      </ReflexElement>
      <ReflexElement>
        <ReflexContainer orientation="vertical" style={{ height: "90vh" }}>
          <ReflexElement className="left-pane">
            <div className="pane-content" style={{ height: "100%" }}>
              <div className="title">Code</div>

              <CodeEditor
                value={code}
                language="js"
                placeholder="Please enter Bian-lang code."
                onChange={(evn) => {
                  const c = evn.target.value;
                  setCode(c);
                  let re = compute(c);
                  // console.log(re);
                  setResult(re);
                }}
                padding={15}
                style={{
                  fontSize: 17,
                  // backgroundColor: "#f5f5f5",
                  fontFamily:
                    "ui-monospace,SFMono-Regular,SF Mono,Consolas,Liberation Mono,Menlo,monospace",
                  height: "100%",
                  lineHeight: "1.5",
                }}
              />
            </div>
          </ReflexElement>
          <ReflexSplitter />
          <ReflexElement className="right-pane">
            <ReflexContainer orientation="horizontal">
              <ReflexElement className="right-up-pane">
                <div className="pane-content">
                  <div className="title">Compiling</div>
                  <SimpleJson src={result[0]} />
                </div>
              </ReflexElement>
              <ReflexSplitter />
              <ReflexElement className="right-down-pane" flex={0.3}>
                <div className="pane-content">
                  <div className="title">Result</div>
                  <SimpleJson src={result[1]} />
                </div>
              </ReflexElement>
            </ReflexContainer>
          </ReflexElement>
        </ReflexContainer>
      </ReflexElement>
    </ReflexContainer>
  );
}

ReactDOM.render(<App />, document.getElementById("react-root"));
import("./w.js")
  .then((a) => (wasmAdapter = a))
  .catch((e) => console.error("Error importing `w.js`:", e));
