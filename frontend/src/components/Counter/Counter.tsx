import { useAppSelector } from "../../state";
import { text } from "./Counter.css";

export function Counter() {
  const { cards, split } = useAppSelector((s) => s.print);
  const count = cards.reduce((a, v) => a + v.quantity, 0);
  const pages = Math.ceil(count / 9);

  const plural = (num: number, text: string) => {
    const s = num === 1 ? "" : "s";
    return `${num} ${text}${s}`;
  };

  const splitCount = split
    ? Math.ceil(cards.reduce((a, v) => a + v.quantity, 0) / 9 / split)
    : 0;

  return (
    <p className={text}>
      {plural(count, "card")}
      {", "}
      {split ? `${splitCount} x ${split} pages` : plural(pages, "page")}
    </p>
  );
}
