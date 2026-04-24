import { useMemo, useState } from "react";
import { useAppSelector } from "../../state";
import { useCardsQuery } from "../../state/api";
import { Button } from "../common/Button/Button";
import { Dialog } from "../common/Dialog/Dialog";
import { buttonRow, cancel, textarea } from "./Export.css";

export function Export() {
  const cards = useAppSelector((s) => s.print.cards);
  const [open, setOpen] = useState(false);

  const { data, isLoading } = useCardsQuery(
    {
      ids: cards.map(({ id }) => id),
    },
    {
      skip: !open,
    },
  );
  const cardsMap = useMemo(
    () => new Map((data?.cards ?? []).map((card) => [card.id, card])),
    [data],
  );

  const text = useMemo(() => {
    if (isLoading) {
      return "Loading...";
    }
    if (!data) {
      return "An unexpected error has occurred.";
    }
    const lines = [];
    for (const { quantity, id } of cards) {
      const apiCard = cardsMap.get(id);
      if (!apiCard) {
        lines.push("???");
        continue;
      }
      const name = apiCard.flavor_name ?? apiCard.name;
      const set = apiCard.set.toUpperCase();
      const collectorNumber = apiCard.collector_number;
      lines.push(`${quantity} ${name} (${set}) ${collectorNumber}`);
    }
    return lines.join("\n");
  }, [cards, cardsMap, data, isLoading]);

  return (
    <>
      <Dialog open={open} onClose={() => setOpen(false)}>
        <textarea
          className={textarea}
          value={text}
          readOnly
          onClick={(e) => e.currentTarget.select()}
        />
        <div className={buttonRow}>
          <Button className={cancel} onClick={() => setOpen(false)}>
            Close
          </Button>
        </div>
      </Dialog>
      <Button onClick={() => setOpen(true)}>Export</Button>
    </>
  );
}
