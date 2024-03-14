import { QueryStatus } from "@reduxjs/toolkit/query";
import cn from "classnames";
import { FormEvent, KeyboardEvent, useState } from "react";
import loadingPng from "../../assets/loading.png";
import { useAppDispatch } from "../../state";
import {
  getDefaultVariantIdx,
  getImageUrl,
  useAutocompleteQuery,
  useCardQuery,
} from "../../state/api";
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
  wrapper,
} from "./CardInput.css";
import { printAction } from "../../state/print";

export function CardInput() {
  const dispatch = useAppDispatch();
  const [input, setInput] = useState("");
  const [activeIdx, setActiveIdx] = useState(0);

  const { data: autocompleteData, status: autocompleteStatus } =
    useAutocompleteQuery(input);

  const entries = autocompleteData?.data ?? [];
  const activeCardName = entries[activeIdx] as string | undefined;

  const { data: activeCardData, status: activeCardStatus } = useCardQuery(
    activeCardName ?? "",
    {
      skip: activeCardName === undefined,
    }
  );

  const allFulfilled =
    autocompleteStatus === QueryStatus.fulfilled &&
    activeCardStatus === QueryStatus.fulfilled;

  let imageSrc: string | undefined;
  if (allFulfilled && activeCardData && activeCardData.data.length > 0) {
    const idx = getDefaultVariantIdx(activeCardData.data);
    const card = activeCardData.data[idx];
    imageSrc = getImageUrl(card);
  }

  const handleKeyDown = (e: KeyboardEvent) => {
    if (e.key === "ArrowDown") {
      e.preventDefault();
      if (entries.length > 0) {
        setActiveIdx((activeIdx + 1) % entries.length);
      }
    } else if (e.key === "ArrowUp") {
      if (entries.length > 0) {
        setActiveIdx((activeIdx + entries.length - 1) % entries.length);
      }
    }
  };

  const handleClick = (idx: number) => {
    setActiveIdx(idx);
  };

  const handleSubmit = (e: FormEvent) => {
    e.preventDefault();
    if (
      !allFulfilled ||
      !activeCardData ||
      activeCardData.data.length === 0 ||
      !activeCardName
    ) {
      return;
    }
    setInput("");
    const name = activeCardName;
    const ids = activeCardData.data.map((x) => x.id);
    dispatch(
      printAction.add({
        name,
        quantity: 1,
        variant: getDefaultVariantIdx(activeCardData.data),
        face: 0,
        ids,
      })
    );
  };

  return (
    <div className={wrapper}>
      <form className={container} onSubmit={handleSubmit}>
        <Input
          className={inputField}
          type="text"
          placeholder="Enter a card name"
          value={input}
          onChange={(e) => setInput(e.target.value)}
          onKeyDown={handleKeyDown}
        />
        <Button type="submit">Add</Button>
        <AutoComplete
          activeIdx={activeIdx}
          entries={entries}
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

export function AutoComplete(props: AutoCompleteProps) {
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
