import cn from "classnames";
import { defaultTheme } from "../../style/global.css";
import { CardInput } from "../CardInput/CardInput";
import { CardPreview } from "../CardPreview/CardPreview";
import { LocalStorage } from "../LocalStorage/LocalStorage";
import { Print } from "../Print/Print";
import { container, title } from "./App.css";

export function App() {
  return (
    <div className={cn(container, defaultTheme)}>
      <h1 className={title}>MTG Proxy Maker</h1>
      <CardInput />
      <CardPreview />
      <Print />
      <LocalStorage />
    </div>
  );
}
