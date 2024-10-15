import { listen, Event, UnlistenFn } from '@tauri-apps/api/event';
import { WebviewWindow, PhysicalSize } from '@tauri-apps/api/window';
import { useEffect, useState } from 'react';
import { TypographyParameter, WindowAppearanceParameter } from '../parameter';

// interface Keystroke {
//     symbols: Array<string>,
// }
const key_window: WebviewWindow = new WebviewWindow("KeyWindow");
// TypographyParameter
const initFontSize = (): number => {
    return 12;
}
const initFontFamily = (): string => {
    return "SansSerif";
}
const initTextColor = (): string => {
    return "#eeeeee";
}
// WindowAppearanceParameter
const initBackgroundColor = (): string => {
    return "#ff0000";
}
const initBackgroundOpacity = (): number => {
    return 50;
}
const initTransparentToggle = (): boolean => {
    return true;
}
// Keystrokes
const initKeystrokes = (): Array<Array<string>> => {
    return [];
}

const App: React.FC = () => {
    // TypographyParameter: useState
    const [fontsize, setFontSize] = useState<number>(initFontSize);
    const [fontfamily, setFontFamily] = useState<string>(initFontFamily);
    const [textcolor, setTextColor] = useState<string>(initTextColor);

    // WindowAppearanceParameter: useState
    const [backgroundcolor, setBackgroundColor] = useState<string>(initBackgroundColor);
    const [transparenttoggle, setTransparentToggle] = useState<boolean>(initTransparentToggle);
    const [backgroundopacity, setBackgroundOpacity] = useState<number>(initBackgroundOpacity);

    // Keystrokes
    const [keystrokes, setKeystrokes] = useState<Array<Array<string>>>(initKeystrokes);

    // TypegraphyParameter: useEffect
    useEffect(() => {
        let unlisten: UnlistenFn;
        async function f() {
            unlisten = await listen('on-change-typography', (event: Event<TypographyParameter>) => {
                console.log(event.payload);
                setFontSize(event.payload.fontsize);
                setFontFamily(event.payload.fontfamily);
                setTextColor(event.payload.textcolor);
            });
        }
        f();
        return () => {
            if (unlisten) {
                unlisten();
            }
        }
    }, []);
    // WindowAppearanceParameter: useEffect
    useEffect(() => {
        let unlisten: UnlistenFn;
        async function f() {
            unlisten = await listen('on-change-windowappearance', (event: Event<WindowAppearanceParameter>) => {
                setBackgroundColor(event.payload.backgroundcolor);
                setBackgroundOpacity(event.payload.backgroundopacity);
                setTransparentToggle(event.payload.transparantetoggle);
            });
        }
        f();
        return () => {
            if (unlisten) {
                unlisten();
            }
        }
    }, []);
    useEffect(() => {
        let unlisten: UnlistenFn;
        async function f() {
            unlisten = await listen('keyevent', (event: Event<Array<Array<string>>>) => {
                setKeystrokes(event.payload);
                const length = event.payload.length;
                key_window.setSize(new PhysicalSize(150 + length * 32, 100));
            });
        }
        f();
        return () => {
            if (unlisten) {
                unlisten();
            }
        }
    }, []);
    return (
        <div 
            className="flex flex-col justify-center p-1 gap-1"
            style={{
                backgroundColor: `color-mix(in srgb, ${backgroundcolor} ${transparenttoggle ? backgroundopacity : 100}%, transparent)`,
                borderRadius: 5,
            }}
        >
            <div
                data-tauri-drag-region
                className="grabbar w-20 h-1 bg-gray-400 opacity-50 rounded-full transition duration-300 hover:opacity-100"
            >
            </div>
            <div
                className="w-fit min-w-20 min-h-8 flex justify-start"
            >
                {
                    keystrokes.map(keysyms => {
                        return (
                            <div
                                className="flex justify-start p-0.5"
                                style={{
                                    backgroundColor: `color-mix(in srgb, ${backgroundcolor} ${transparenttoggle ? backgroundopacity : 100}%, transparent)`,
                                }}
                            >
                                {
                                    keysyms.map(keysym => {
                                        return (
                                            <div
                                                className="w-fit min-w-8 p-0.5 flex justify-center"
                                                style={{
                                                    backgroundColor: `color-mix(in srgb, ${backgroundcolor} ${transparenttoggle ? backgroundopacity : 100}%, transparent)`,
                                                    color: `${textcolor}`,
                                                    fontSize: `${fontsize}px`,
                                                    fontFamily: `${fontfamily}`,
                                                }}
                                            >
                                                {keysym}
                                            </div>
                                        );
                                    })
                                }
                            </div>
                        );
                    })
                }
            </div>
        </div>
    )
}
export default App;
