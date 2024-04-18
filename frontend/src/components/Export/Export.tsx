import { useState } from "react";
import { useAppSelector } from "../../state";
import { useLazyCardQuery } from "../../state/api";
import { Button } from "../common/Button/Button";
import { Dialog } from "../common/Dialog/Dialog";
import { buttonRow, cancel, textarea } from "./Export.css";
import { QueryStatus } from "@reduxjs/toolkit/query";

export function Export() {
  const cards = useAppSelector((s) => s.print.cards);
  const [dialogOpen, setDialogOpen] = useState(false);
  const [text, setText] = useState("");

  const [fetchCard] = useLazyCardQuery();

  const handleOpen = async () => {
    setDialogOpen(true);
    setText("Loading...");
    const lines = [];
    for (const card of cards) {
      const { id, name, quantity } = card;
      const data = await fetchCard(name, true);
      if (data.status !== QueryStatus.fulfilled) {
        setText("Error fetching card data");
        break;
      }
      let set = "???";
      let collectorNumber = "???";
      for (const card of data.data.cards) {
        if (card.id === id) {
          set = card.set.toUpperCase();
          collectorNumber = card.collector_number;
          break;
        }
      }
      lines.push(`${quantity} ${name} (${set}) ${collectorNumber}`);
    }
    setText(lines.join("\n"));
  };

  return (
    <>
      <Dialog open={dialogOpen} onClose={() => setDialogOpen(false)}>
        <textarea
          className={textarea}
          value={text}
          readOnly
          onClick={(e) => e.currentTarget.select()}
        />
        <div className={buttonRow}>
          <Button className={cancel} onClick={() => setDialogOpen(false)}>
            Close
          </Button>
        </div>
      </Dialog>
      <Button onClick={handleOpen}>Export</Button>
    </>
  );
}
