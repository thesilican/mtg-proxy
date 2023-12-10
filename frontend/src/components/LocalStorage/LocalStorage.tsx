import { useEffect, useState } from "react";
import { useAppDispatch, useAppSelector } from "../../state";
import { printAction } from "../../state/print";

const STORAGE_KEY = "mtg-proxy";

export function LocalStorage() {
  const dispatch = useAppDispatch();
  const cards = useAppSelector((s) => s.print.cards);
  const [loaded, setLoaded] = useState(false);

  useEffect(() => {
    const text = localStorage.getItem(STORAGE_KEY);
    if (text !== null) {
      const { cards } = JSON.parse(text);
      dispatch(printAction.load(cards));
    }
    setLoaded(true);
  }, []);

  useEffect(() => {
    if (loaded) {
      const obj = {
        cards,
      };
      const text = JSON.stringify(obj);
      localStorage.setItem(STORAGE_KEY, text);
    }
  }, [loaded, cards]);

  return null;
}
