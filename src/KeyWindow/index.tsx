import { listen, Event, UnlistenFn } from '@tauri-apps/api/event';
import { useEffect, useState } from 'react';
import { TypographyParameter, WindowAppearanceParameter } from '../parameter';

const initBackgroundColor = (): string => {
  return "#ff0000";
}
const initBackgroundOpacity = (): number => {
  return 50;
}
const initTransparentToggle = (): boolean => {
  return true;
}

const App: React.FC = () => {
  const [backgroundcolor, setBackgroundColor] = useState(initBackgroundColor);
  const [transparenttoggle, setTransparentToggle] = useState(initTransparentToggle);
  const [backgroundopacity, setBackgroundOpacity] = useState(initBackgroundOpacity);
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
      unlisten = await listen('keyevent', (event) => {
        console.log(event);
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
      className="w-full h-screen flex flex-row"
      style={{
        backgroundColor: `color-mix(in srgb, ${backgroundcolor} ${transparenttoggle ? backgroundopacity : 100}%, transparent)`,
        borderRadius: 10,
      }}
    >
      <button className="btn btn-primary">KeyWindow</button>
    </div>
  )
}
export default App;
