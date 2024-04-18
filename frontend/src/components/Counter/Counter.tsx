import { useAppSelector } from "../../state";
import { text } from "./Counter.css";

export function Counter() {
  const cards = useAppSelector((s) => s.print.cards);
  const count = cards.reduce((a, v) => a + v.quantity, 0);
  const pages = Math.ceil(count / 9);

  const plural = (num: number, text: string) => {
    const s = num === 1 ? "" : "s";
    return `${num} ${text}${s}`;
  };

  return (
    <p className={text}>
      {plural(count, "card")} ({plural(pages, "page")})
    </p>
  );
}
