import cn from "classnames";
import * as globalStyles from "../../global.css";
import { CardInput } from "../CardInput/CardInput";
import { CardPreview } from "../CardPreview/CardPreview";
import { Clear } from "../Clear/Clear";
import { Status } from "../Status/Status";
import { Export } from "../Export/Export";
import { Import } from "../Import/Import";
import { LocalStorage } from "../LocalStorage/LocalStorage";
import { Print } from "../Print/Print";
import * as styles from "./App.css";
import { Split } from "../Split/Split";

export function App() {
  return (
    <div className={cn(styles.app, globalStyles.defaultTheme)}>
      <div className={styles.background} />
      <div className={styles.header}>
        <div className={styles.titleBox}>
          <h1>MTG Proxy Maker</h1>
          <p>
            By <a href="https://thesilican.com">Bryan Chen</a>
          </p>
        </div>
        <CardInput />
      </div>
      <CardPreview />
      <div className={styles.footer}>
        <div className={styles.row}>
          <Import />
          <Export />
          <Split />
          <Clear />
          <Print />
        </div>
        <div className={styles.row2}>
          <Status />
          <p className={styles.artistCredit}>
            Background:{" "}
            <a
              href="https://www.artstation.com/artwork/kD2ozd"
              target="_blank"
              rel="noopener"
            >
              Needleverge Pathway
            </a>{" "}
            by Piotr Dura
          </p>
        </div>
      </div>
      <LocalStorage />
    </div>
  );
}
