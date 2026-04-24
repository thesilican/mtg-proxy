import cn from "classnames";
import { FormEvent, useEffect, useMemo, useRef, useState } from "react";
import { useAppDispatch } from "../../state";
import { ApiCard, useSearchQuery } from "../../state/api";
import { printAction } from "../../state/print";
import { Button } from "../common/Button/Button";
import { Input } from "../common/Input/Input";
import * as styles from "./CardInput.css";

export function CardInput() {
  const dispatch = useAppDispatch();
  const ref = useRef<HTMLInputElement>(null);
  const [input, setInput] = useState("");
  const [activeIdx, setActiveIdx] = useState(0);

  useEffect(() => {
    if (!ref.current) {
      return;
    }
    const input = ref.current;
    const handler = (e: KeyboardEvent) => {
      if (e.key === "/" && e.target === document.body) {
        input.focus();
        e.preventDefault();
      }
    };
    document.addEventListener("keydown", handler);
    return () => document.removeEventListener("keydown", handler);
  }, []);

  const { data } = useSearchQuery({ q: input });
  const cards = useMemo(() => data?.cards ?? [], [data]);
  const activeCard = cards[activeIdx] ?? null;

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === "ArrowDown") {
      e.preventDefault();
      if (cards.length > 0) {
        setActiveIdx((activeIdx + 1) % cards.length);
      }
    } else if (e.key === "ArrowUp") {
      if (cards.length > 0) {
        setActiveIdx((activeIdx + cards.length - 1) % cards.length);
      }
    }
  };

  const handleClick = (idx: number) => {
    if (activeIdx === idx) {
      handleSubmit();
    } else {
      setActiveIdx(idx);
    }
  };

  const handleSubmit = (e?: FormEvent) => {
    e?.preventDefault();
    if (!activeCard) {
      return;
    }
    setInput("");
    setActiveIdx(0);
    dispatch(
      printAction.add({
        id: activeCard.id,
        name: activeCard.name,
        quantity: 1,
        face: "front",
      }),
    );
  };

  return (
    <form className={styles.container} onSubmit={handleSubmit}>
      <Input
        ref={ref}
        className={styles.inputField}
        type="text"
        placeholder="Enter a card name"
        value={input}
        onChange={(e) => {
          setActiveIdx(0);
          setInput(e.target.value);
        }}
        onKeyDown={handleKeyDown}
        autoFocus
      />
      <Button type="submit">Add</Button>
      <AutoComplete
        cards={cards}
        activeCard={activeCard}
        onClick={handleClick}
      />
    </form>
  );
}

function AutoComplete({
  cards,
  activeCard,
  onClick,
}: {
  cards: ApiCard[];
  activeCard: ApiCard | null;
  onClick: (idx: number) => void;
}) {
  return (
    <div
      className={cn(styles.autocomplete, cards.length === 0 && styles.hidden)}
    >
      <div className={styles.dropdown}>
        {cards.map((card, i) => {
          const isActive = card.id === activeCard?.id;
          return (
            <span
              key={card.id}
              className={cn(styles.entry, isActive && styles.active)}
              ref={(e) => isActive && e?.scrollIntoView({ block: "nearest" })}
              onClick={() => onClick(i)}
            >
              {card.name}
              {card.flavor_name && ` (${card.flavor_name})`}
            </span>
          );
        })}
      </div>
      {activeCard && (
        <img
          className={styles.img}
          width={146}
          height={204}
          src={activeCard.images.front_jpg}
        />
      )}
    </div>
  );
}
