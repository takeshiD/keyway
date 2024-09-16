import { listen, Event, UnlistenFn } from '@tauri-apps/api/event';
import { useEffect, useState } from 'react';
import { TypographyParameter, WindowAppearanceParameter } from '../parameter';

interface Keystroke {
    keycode: number,
    symbol: string,
}

const initBackgroundColor = (): string => {
    return "#ff0000";
}
const initBackgroundOpacity = (): number => {
    return 50;
}
const initTransparentToggle = (): boolean => {
    return true;
}
const initKeystrokes = (): Array<Keystroke> => {
    return [];
}

const App: React.FC = () => {
    const [backgroundcolor, setBackgroundColor] = useState<string>(initBackgroundColor);
    const [transparenttoggle, setTransparentToggle] = useState<boolean>(initTransparentToggle);
    const [backgroundopacity, setBackgroundOpacity] = useState<number>(initBackgroundOpacity);
    const [keystrokes, setKeystrokes] = useState<Array<Keystroke>>(initKeystrokes);
    useEffect(() => {
        let unlisten: UnlistenFn;
        async function f() {
            unlisten = await listen('on-change-typography', (event: Event<TypographyParameter>) => {
                console.log(event.payload);
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
            unlisten = await listen('keyevent', (event: Event<Array<Keystroke>>) => {
                setKeystrokes(event.payload)
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
            data-tauri-drag-region
            className="w-full p-3 gap-1 grid grid-cols-[repeat(auto-fit, mimax(200px, 1fr))]"
            style={{
                backgroundColor: `color-mix(in srgb, ${backgroundcolor} ${transparenttoggle ? backgroundopacity : 100}%, transparent)`,
                borderRadius: 10,
            }}
        >
            {
                keystrokes.map(ks => {
                    return (
                        <div
                            className="shadow p-1"
                            style={{
                                backgroundColor: `color-mix(in srgb, ${backgroundcolor} ${transparenttoggle ? backgroundopacity : 100}%, transparent)`,
                                borderRadius: 3,
                            }}
                        >
                            {ks.symbol}
                        </div>
                    );
                })
            }
        </div>
    )
}
export default App;
