import { useEffect } from "react";
import { useAppDispatch, useAppSelector } from "../../state";
import { printAction } from "../../state/print";

const STORAGE_KEY = "mtg-proxy";

export function LocalStorage() {
  const dispatch = useAppDispatch();
  const print = useAppSelector((s) => s.print);

  useEffect(() => {
    const text = localStorage.getItem(STORAGE_KEY);
    if (text !== null) {
      try {
        const { cards, split } = JSON.parse(text);
        dispatch(printAction.load({ cards, split }));
      } catch {
        // Oopsie daisy
      }
    }
  }, [dispatch]);

  useEffect(() => {
    const text = JSON.stringify(print);
    localStorage.setItem(STORAGE_KEY, text);
  }, [print]);

  return null;
}
