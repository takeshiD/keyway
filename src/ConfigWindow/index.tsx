import { WebviewWindow } from "@tauri-apps/api/window"
import { useState } from "react"
import { BehaviorParameter, TypographyParameter, WindowAppearanceParameter } from "../parameter";

// const TitleBar: React.FC = () => {
//   return (
//     <div data-tauri-drag-region className="grid grid-cols1">
//       <div className="bg-gray-50" id="titlebar-close" onClick={() => appWindow.hide()}>
//         <img src="https://api.iconify.design/mdi:close.svg" alt="close" />
//       </div>
//     </div >
//   )
// }

const config_window: WebviewWindow = new WebviewWindow("ConfigWindow");

const Behavior: React.FC = () => {
    const [timeout, setTimeout] = useState(500);
    const [mousevisible, setMouseVisible] = useState(false);
    const [modvisible, setModVisible] = useState(false);
    const behavior_param: BehaviorParameter = {
        timeout: timeout,
        mousevisible: mousevisible,
        modvisible: modvisible
    };
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
                    onChange={e => {
                        behavior_param.timeout = parseInt(e.target.value);
                        config_window.emit("on-change-behavior", behavior_param);
                        setTimeout(parseInt(e.target.value));
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
                        behavior_param.mousevisible = mousevisible;
                        config_window.emit("on-change-behavior", behavior_param);
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
                        behavior_param.modvisible = modvisible;
                        config_window.emit("on-change-behavior", behavior_param);
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
    const typography_param: TypographyParameter = {
        fontsize: fontsize,
        fontfamily: fontfamily,
        textcolor: textcolor,
    };
    return (
        <div className="grid grid-cols-4 gap-4 m-4">
            {/************ Fontfamily ************/}
            <div className="col-span-1">Fontfamily</div>
            <div className="col-span-3 flex flex-row gap-2">
                <select
                    className="select select-sm select-bordered w-full"
                    value={fontfamily}
                    onChange={e => {
                        typography_param.fontfamily = e.target.value;
                        config_window.emit("on-change-typography", typography_param);
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
                        typography_param.fontsize = parseInt(e.target.value);
                        config_window.emit("on-change-typography", typography_param);
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
                        typography_param.textcolor = e.target.value;
                        config_window.emit("on-change-typography", typography_param);
                        setTextColor(e.target.value)
                    }}
                />
                <span>{textcolor}</span>
            </div>
        </div>
    )
}

const WindowAppearance: React.FC = () => {
    const [backgroundcolor, setBackgroundColor] = useState("#e0e0e0");
    const [transparenttoggle, setTransparentToggle] = useState(true);
    const [backgroundopacity, setBackgroundOpacity] = useState(50);
    const windowappearance_param: WindowAppearanceParameter = {
        backgroundcolor: backgroundcolor,
        transparantetoggle: transparenttoggle,
        backgroundopacity: backgroundopacity,
    };
    return (
        <div className="grid grid-cols-4 gap-4 m-4">
            {/************ BackgroundColor ************/}
            <div className="col-span-1">BackgroundColor</div>
            <div className="col-span-3 flex flex-row gap-2">
                <input
                    type="color"
                    value={backgroundcolor}
                    onChange={e => {
                        windowappearance_param.backgroundcolor = e.target.value;
                        config_window.emit("on-change-windowappearance", windowappearance_param);
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
                        windowappearance_param.transparantetoggle = e.target.checked;
                        config_window.emit("on-change-windowappearance", windowappearance_param);
                        setTransparentToggle(e.target.checked)
                    }}
                />
                <input
                    type="range"
                    className="range range-primary range-sm"
                    min={0}
                    max={100}
                    step={1}
                    value={backgroundopacity}
                    disabled={!transparenttoggle}
                    onChange={e => {
                        windowappearance_param.backgroundopacity = parseInt(e.target.value);
                        config_window.emit("on-change-windowappearance", windowappearance_param);
                        setBackgroundOpacity(parseInt(e.target.value))
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
