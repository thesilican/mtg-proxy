import { ChangeEvent, useEffect, useMemo, useState } from "react";
import { useAppDispatch, useAppSelector } from "../../state";
import { isDfc, useCardsQuery } from "../../state/api";
import { printAction } from "../../state/print";
import { Button } from "../common/Button/Button";
import { Input } from "../common/Input/Input";
import * as styles from "./CardPreview.css";
import loadingPng from "../../assets/loading.png";

export function CardPreview() {
  const cards = useAppSelector((state) => state.print.cards);

  return (
    <div className={styles.wrapper}>
      {cards.length === 0 ? (
        <p className={styles.placeholder}>
          To get started, search for a card to add or import a list of cards.
        </p>
      ) : (
        <div className={styles.container}>
          {cards.map((card, idx, arr) => {
            // Count which instance of the card id
            let count = 0;
            for (let i = 0; i < idx; i++) {
              if (arr[i].name == card.name) {
                count++;
              }
            }
            return <CardItem key={`${card.name}-${count}`} idx={idx} />;
          })}
        </div>
      )}
    </div>
  );
}

function CardItem({ idx }: { idx: number }) {
  const card = useAppSelector((state) => state.print.cards[idx]);
  const dispatch = useAppDispatch();
  const [quantity, setQuantity] = useState(card.quantity.toString());
  useEffect(() => {
    setQuantity(card.quantity.toString());
  }, [card.quantity]);

  const { data } = useCardsQuery({ name: card.name });

  const cards = useMemo(() => data?.cards ?? [], [data]);
  const activeCard = cards.find((x) => x.id === card.id);

  const imgSrc =
    card.face === "front"
      ? activeCard?.images.front_jpg
      : activeCard?.images.back_jpg;

  const handleUpdateQuantity = (e: ChangeEvent<HTMLInputElement>) => {
    setQuantity(e.target.value);
    const num = parseInt(e.target.value, 10);
    if (!isNaN(num)) {
      dispatch(printAction.update({ idx, card: { quantity: num } }));
    }
  };

  const handleVariantChange = (e: ChangeEvent<HTMLSelectElement>) => {
    dispatch(
      printAction.update({
        idx,
        card: { id: e.target.value },
      }),
    );
  };

  const handleFlip = () => {
    dispatch(
      printAction.update({
        idx,
        card: { face: card.face === "front" ? "back" : "front" },
      }),
    );
  };

  const handleRemove = () => {
    dispatch(printAction.remove(idx));
  };

  return (
    <div className={styles.card}>
      <div className={styles.top}>
        <Input
          className={styles.number}
          type="number"
          value={quantity}
          min={1}
          max={999}
          onChange={handleUpdateQuantity}
        />
        <div className={styles.spacer} />
        {activeCard && isDfc(activeCard) && (
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
      <img
        className={styles.img}
        width={175}
        height={245}
        src={imgSrc ?? loadingPng}
        alt={activeCard?.name}
      />
      <div className={styles.bottom}>
        <select
          className={styles.select}
          value={card.id}
          onChange={handleVariantChange}
        >
          {cards.map((card) => (
            <option key={card.id} value={card.id}>
              {`${card.set_name} (${card.collector_number})`}
            </option>
          ))}
        </select>
      </div>
    </div>
  );
}
