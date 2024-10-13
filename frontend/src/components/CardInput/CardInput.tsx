import { QueryStatus } from "@reduxjs/toolkit/query";
import cn from "classnames";
import { FormEvent, useEffect, useRef, useState } from "react";
import loadingPng from "../../assets/loading.png";
import { useAppDispatch } from "../../state";
import {
  getPreferredCard,
  useAutocompleteQuery,
  useCardQuery,
} from "../../state/api";
import { printAction } from "../../state/print";
import { Button } from "../common/Button/Button";
import { Input } from "../common/Input/Input";
import {
  active,
  autocomplete,
  container,
  dropdown,
  entry,
  hidden,
  img,
  inputField,
  subtitle,
  title,
  wrapper,
} from "./CardInput.css";

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

  const { data: autocompleteData, status: autocompleteStatus } =
    useAutocompleteQuery(input);

  const autocompleteNames = autocompleteData?.names ?? [];
  const activeName = autocompleteNames[activeIdx] as string | undefined;

  const { data: activeCardData, status: activeCardStatus } = useCardQuery(
    activeName ?? "",
    {
      skip: activeName === undefined,
    }
  );

  const allFulfilled =
    autocompleteStatus === QueryStatus.fulfilled &&
    activeCardStatus === QueryStatus.fulfilled;

  let imageSrc: string | undefined;
  if (activeCardData && activeCardData.cards.length > 0) {
    const card = getPreferredCard(activeCardData.cards);
    if (card.name === activeName) {
      imageSrc = card.image_front_large;
    } else {
      imageSrc = loadingPng;
    }
  }

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === "ArrowDown") {
      e.preventDefault();
      if (autocompleteNames.length > 0) {
        setActiveIdx((activeIdx + 1) % autocompleteNames.length);
      }
    } else if (e.key === "ArrowUp") {
      if (autocompleteNames.length > 0) {
        setActiveIdx(
          (activeIdx + autocompleteNames.length - 1) % autocompleteNames.length
        );
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
    if (
      !allFulfilled ||
      !activeCardData ||
      activeCardData.cards.length === 0 ||
      !activeName
    ) {
      return;
    }
    setInput("");
    setActiveIdx(0);
    const name = activeName;
    const id = getPreferredCard(activeCardData.cards).id;
    dispatch(
      printAction.add({
        name,
        quantity: 1,
        face: 0,
        id,
      })
    );
  };

  return (
    <div className={wrapper}>
      <h1 className={title}>MTG Proxy Maker</h1>
      <p className={subtitle}>
        By <a href="https://thesilican.com">Bryan Chen</a>
      </p>
      <form className={container} onSubmit={handleSubmit}>
        <Input
          ref={ref}
          className={inputField}
          type="text"
          placeholder="Enter a card name"
          value={input}
          onChange={(e) => setInput(e.target.value)}
          onKeyDown={handleKeyDown}
          autoFocus
        />
        <Button type="submit">Add</Button>
        <AutoComplete
          activeIdx={activeIdx}
          entries={autocompleteNames}
          status={autocompleteStatus}
          imageSrc={imageSrc}
          onClick={handleClick}
        />
      </form>
    </div>
  );
}

type AutoCompleteProps = {
  status: QueryStatus;
  entries: string[];
  activeIdx: number;
  imageSrc: string | undefined;
  onClick: (idx: number) => void;
};

function AutoComplete(props: AutoCompleteProps) {
  const isHidden = props.entries.length === 0;
  const imageSrc = props.imageSrc ?? loadingPng;

  return (
    <div className={cn(autocomplete, isHidden && hidden)}>
      <div className={dropdown}>
        {props.entries.map((val, i) => {
          const isActive = i === props.activeIdx;
          return (
            <span
              key={i}
              className={cn(entry, isActive && active)}
              ref={(e) => isActive && e?.scrollIntoView({ block: "nearest" })}
              onClick={() => props.onClick(i)}
            >
              {val}
            </span>
          );
        })}
      </div>
      <img
        className={cn(img, false && hidden)}
        width={146}
        height={204}
        src={imageSrc}
      />
    </div>
  );
}
