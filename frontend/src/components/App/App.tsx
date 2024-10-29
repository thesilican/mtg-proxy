import cn from "classnames";
import { defaultTheme } from "../../style/global.css";
import { CardInput } from "../CardInput/CardInput";
import { CardPreview } from "../CardPreview/CardPreview";
import { Clear } from "../Clear/Clear";
import { Counter } from "../Counter/Counter";
import { Export } from "../Export/Export";
import { Import } from "../Import/Import";
import { LocalStorage } from "../LocalStorage/LocalStorage";
import { Print } from "../Print/Print";
import { container, footer } from "./App.css";
import { Split } from "../Split/Split";

export function App() {
  return (
    <div className={cn(container, defaultTheme)}>
      <CardInput />
      <CardPreview />
      <div className={footer}>
        <Import />
        <Export />
        <Split />
        <Clear />
        <Counter />
        <Print />
      </div>
      <LocalStorage />
    </div>
  );
}
