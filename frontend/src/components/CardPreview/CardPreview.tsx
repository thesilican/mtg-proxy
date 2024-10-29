import { QueryStatus } from "@reduxjs/toolkit/query";
import { ChangeEvent, useEffect, useState } from "react";
import loadingPng from "../../assets/loading.png";
import { useAppDispatch, useAppSelector } from "../../state";
import { useCardQuery } from "../../state/api";
import { printAction } from "../../state/print";
import { Button } from "../common/Button/Button";
import { Input } from "../common/Input/Input";
import {
  bottom,
  cardStyle,
  container,
  img,
  number,
  placeholder,
  select,
  spacer,
  top,
} from "./CardPreview.css";

export function CardPreview() {
  const cards = useAppSelector((state) => state.print.cards);
  return (
    <div className={container}>
      {cards.map((card, idx, arr) => {
        // Count how many names
        let count = 0;
        for (let i = 0; i < idx; i++) {
          if (arr[i].name == card.name) {
            count++;
          }
        }
        return <Card key={`${card.name}-${count}`} idx={idx} />;
      })}
      {cards.length === 0 && (
        <p className={placeholder}>
          To get started, search for a card to add or import a list of cards.
        </p>
      )}
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

  const { data: cardData, status } = useCardQuery(card.name);

  const variants = cardData?.cards ?? [];
  const activeVariant = cardData?.cards.find((x) => x.id === card.id);
  const isDfc = activeVariant ? !!activeVariant.image_back_png : undefined;
  const imgSrc =
    activeVariant && status === QueryStatus.fulfilled
      ? card.face === 0
        ? activeVariant.image_front_large
        : activeVariant.image_back_large!
      : loadingPng;

  useEffect(() => {
    const num = parseInt(quantity, 10);
    if (!isNaN(num)) {
      dispatch(printAction.update({ idx: props.idx, card: { quantity: num } }));
    }
  }, [dispatch, props.idx, quantity]);

  const handleVariantChange = (e: ChangeEvent<HTMLSelectElement>) => {
    dispatch(
      printAction.update({
        idx: props.idx,
        card: {
          id: e.target.value,
        },
      })
    );
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
        {isDfc && (
          <Button
            size="small"
            className="material-symbols-outlined"
            onClick={handleFlip}
          >
            flip
          </Button>
        )}
        <Button
          variant="danger"
          size="small"
          className={"material-symbols-outlined"}
          onClick={handleRemove}
        >
          close
        </Button>
      </div>
      <img className={img} width={175} height={245} src={imgSrc} />
      <div className={bottom}>
        <select
          className={select}
          value={card.id}
          onChange={handleVariantChange}
        >
          {variants.map((card) => (
            <option key={card.id} value={card.id}>
              {`${card.set_name} (${card.collector_number})`}
            </option>
          ))}
        </select>
      </div>
    </div>
  );
}
