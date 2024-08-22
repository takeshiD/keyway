import { appWindow } from "@tauri-apps/api/window"
import { useState } from "react"

const TitleBar: React.FC = () => {
  return (
    <div data-tauri-drag-region className="grid grid-cols1">
      <div className="bg-gray-50" id="titlebar-close" onClick={() => appWindow.hide()}>
        <img src="https://api.iconify.design/mdi:close.svg" alt="close" />
      </div>
    </div >
  )
}

const Behavior: React.FC = () => {
  const [timeout, setTimeout] = useState(500);
  const [mousevisible, setMouseVisible] = useState(false);
  const [modvisible, setModVisible] = useState(false);
  return (
    <div className="grid grid-cols-4 gap-4 m-4">
      <div className="col-span-1">Timeout</div>
      <div className="col-span-3 flex flex-row gap-2">
        <input
          type="range"
          min={50}
          max={2000}
          step={50}
          value={timeout}
          className="range range-sm range-primary"
          onChange={(e) => {
            setTimeout(parseInt(e.target.value))
          }}
        />
        <span>{timeout}ms</span>
      </div>
      <div className="col-span-1">Mouse</div>
      <div className="col-span-3 flex flex-row gap-2">
        <input
          type="checkbox"
          className="toggle toggle-primary"
          checked={mousevisible}
          onChange={(e) => {
            setMouseVisible(e.target.checked)
          }}
        />
        <span>
          {mousevisible
            ? "Visible"
            : "NoVisible"
          }
        </span>
      </div>
      <div className="col-span-1">Modifier</div>
      <div className="col-span-3 flex flex-row gap-2">
        <input
          type="checkbox"
          className="toggle toggle-primary"
          checked={modvisible}
          onChange={(e) => {
            setModVisible(e.target.checked)
          }}
        />
        <span>
          {modvisible
            ? "Visible"
            : "NoVisible"
          }
        </span>
      </div>
    </div>
  )
}
const Typography: React.FC = () => {
  const [fontsize, setFontSize] = useState(12);
  const [fontfamily, setFontFamily] = useState("SansSerif");
  const [textcolor, setTextColor] = useState("#e0e0e0");
  return (
    <div className="grid grid-cols-4 gap-4 m-4">
      {/************ Fontfamily ************/}
      <div className="col-span-1">Fontfamily</div>
      <div className="col-span-3 flex flex-row gap-2">
        <select
          className="select select-sm select-bordered w-full"
          value={fontfamily}
          onChange={e => {
            setFontFamily(e.target.value)
          }}
        >
          <option value="SansSerif">SansSerif</option>
          <option value="Monospace">Monospace</option>
          <option value="Consolas">Consolas</option>
        </select>
      </div>
      {/************ Fontsize ************/}
      <div className="col-span-1">Fontsize</div>
      <div className="col-span-3 flex flex-row gap-2">
        <input
          type="range"
          min={8}
          max={32}
          step={1}
          value={fontsize}
          className="range range-sm range-primary"
          onChange={(e) => {
            setFontSize(parseInt(e.target.value))
          }}
        />
        <span>{fontsize}</span>
      </div>
      {/************ TextColor ************/}
      <div className="col-span-1">TextColor</div>
      <div className="col-span-3 flex flex-row gap-2">
        <input
          type="color"
          value={textcolor}
          onChange={e => {
            setTextColor(e.target.value)
          }}
        />
        <span>{textcolor}</span>
      </div>
    </div>
  )
}

const WindowAppearance: React.FC = () => {
  const [fontsize, setFontSize] = useState(12);
  const [fontfamily, setFontFamily] = useState("SansSerif");
  const [backgroundcolor, setBackgroundColor] = useState("#e0e0e0");
  const [transparenttoggle, setTransparentToggle] = useState(true);
  const [backgroundopacity, setBackgroundOpacity] = useState(0.5);
  return (
    <div className="grid grid-cols-4 gap-4 m-4">
      {/************ BackgroundColor ************/}
      <div className="col-span-1">BackgroundColor</div>
      <div className="col-span-3 flex flex-row gap-2">
        <input
          type="color"
          value={backgroundcolor}
          onChange={e => {
            setBackgroundColor(e.target.value)
          }}
        />
        <span>{backgroundcolor}</span>
      </div>
      {/************ BackgroundTransparent ************/}
      <div className="col-span-1">Transparent</div>
      <div className="col-span-3 flex flex-row gap-2">
        <input
          type="checkbox"
          className="toggle toggle-primary"
          checked={transparenttoggle}
          onChange={e => {
            setTransparentToggle(e.target.checked)
          }}
        />
        <input
          type="range"
          className="range range-primary range-sm"
          min={0.0}
          max={1.0}
          step={0.01}
          value={backgroundopacity}
          disabled={!transparenttoggle}
          onChange={e => {
            setBackgroundOpacity(parseFloat(e.target.value))
          }}
        />
        <span>{backgroundopacity}</span>
      </div>
    </div>
  )
}
const App: React.FC = () => {
  return (
    <div className="flex flex-col px-2">
      <div tabIndex={0} className="collapse collapse-open bg-base-200 border my-2">
        <div className="collapse-title text-xl font-medium">Behavior</div>
        <div className="collapse-content">
          <Behavior />
        </div>
      </div>
      <div tabIndex={1} className="collapse collapse-open bg-base-200 border my-2">
        <div className="collapse-title text-xl font-medium">Typography</div>
        <div className="collapse-content">
          <Typography />
        </div>
      </div>
      <div tabIndex={2} className="collapse collapse-open bg-base-200 border my-2">
        <div className="collapse-title text-xl font-medium">Window Appearance</div>
        <div className="collapse-content">
          <WindowAppearance />
        </div>
      </div>
    </div>
  )
}

export default App;
