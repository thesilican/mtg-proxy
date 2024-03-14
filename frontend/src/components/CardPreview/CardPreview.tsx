import { QueryStatus } from "@reduxjs/toolkit/query";
import cn from "classnames";
import { ChangeEvent, useEffect, useState } from "react";
import loadingPng from "../../assets/loading.png";
import { useAppDispatch, useAppSelector } from "../../state";
import { getImageUrl, useCardQuery } from "../../state/api";
import { printAction } from "../../state/print";
import { Button } from "../common/Button/Button";
import { Input } from "../common/Input/Input";
import {
  bottom,
  cardStyle,
  container,
  flip,
  img,
  number,
  remove,
  select,
  spacer,
  top,
} from "./CardPreview.css";

export function CardPreview() {
  const cards = useAppSelector((state) => state.print.cards);
  return (
    <div className={container}>
      {cards.map((_, i) => (
        <Card key={i} idx={i} />
      ))}
      {cards.length === 0 && <p>To get started, search for a card to add.</p>}
    </div>
  );
}

type CardProps = {
  idx: number;
};

function Card(props: CardProps) {
  const dispatch = useAppDispatch();
  const card = useAppSelector((state) => state.print.cards)[props.idx];
  const [quantity, setQuantity] = useState(card.quantity?.toString());

  const { data: cardData, status: cardStatus } = useCardQuery(card.name);

  const variantNames =
    cardData?.data.map((x) => `${x.set_name} (${x.collector_number})`) ?? [];

  const activeVariant = cardData?.data[card.variant];
  const isDFC = activeVariant
    ? (activeVariant?.card_faces?.length ?? 1) > 1
    : undefined;
  const imgSrc =
    activeVariant && cardStatus === QueryStatus.fulfilled
      ? getImageUrl(activeVariant, card.face)
      : loadingPng;

  useEffect(() => {
    const num = parseInt(quantity, 10);
    if (!isNaN(num)) {
      dispatch(printAction.update({ idx: props.idx, card: { quantity: num } }));
    }
  }, [dispatch, props.idx, quantity]);

  const handleVariantChange = (e: ChangeEvent<HTMLSelectElement>) => {
    const variant = parseInt(e.target.value, 10);
    dispatch(printAction.update({ idx: props.idx, card: { variant } }));
  };

  const handleFlip = () => {
    dispatch(
      printAction.update({ idx: props.idx, card: { face: 1 - card.face } })
    );
  };

  const handleRemove = () => {
    dispatch(printAction.remove(props.idx));
  };

  return (
    <div className={cardStyle}>
      <div className={top}>
        <Input
          className={number}
          type="number"
          value={quantity}
          min={1}
          max={999}
          onChange={(e) => setQuantity(e.target.value)}
        />
        <div className={spacer} />
        {isDFC && (
          <Button
            className={cn("material-symbols-outlined", flip)}
            onClick={handleFlip}
          >
            flip
          </Button>
        )}
        <Button
          className={cn("material-symbols-outlined", remove)}
          onClick={handleRemove}
        >
          close
        </Button>
      </div>
      <img className={img} width={175} height={245} src={imgSrc} />
      <div className={bottom}>
        <select
          className={select}
          value={card.variant}
          onChange={handleVariantChange}
        >
          {variantNames.map((val, i) => (
            <option key={i} value={i}>
              {val}
            </option>
          ))}
        </select>
      </div>
    </div>
  );
}
