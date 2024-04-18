import cn from "classnames";
import { defaultTheme } from "../../style/global.css";
import { CardInput } from "../CardInput/CardInput";
import { CardPreview } from "../CardPreview/CardPreview";
import { Clear } from "../Clear/Clear";
import { Import } from "../Import/Import";
import { LocalStorage } from "../LocalStorage/LocalStorage";
import { Print } from "../Print/Print";
import { container, footer } from "./App.css";
import { Export } from "../Export/Export";
import { Counter } from "../Counter/Counter";

export function App() {
  return (
    <div className={cn(container, defaultTheme)}>
      <CardInput />
      <CardPreview />
      <div className={footer}>
        <Import />
        <Export />
        <Clear />
        <Counter />
        <Print />
      </div>
      <LocalStorage />
    </div>
  );
}
