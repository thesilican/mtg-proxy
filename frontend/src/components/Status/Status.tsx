import { useAppSelector } from "../../state";
import * as styles from "./Status.css";

export function Status() {
  const { cards, split } = useAppSelector((s) => s.print);
  const count = cards.reduce((a, v) => a + v.quantity, 0);
  const pages = Math.ceil(count / 9);
  const pdfs = Math.ceil(pages / (split ?? 1));

  return (
    <p className={styles.text}>
      {plural(count, "card")}, {plural(pages, "page")}
      {split && `, ${plural(pdfs, "PDF")} (${plural(split, "page")}/PDF)`}
    </p>
  );
}

function plural(num: number, text: string) {
  const s = num === 1 ? "" : "s";
  return `${num} ${text}${s}`;
}
