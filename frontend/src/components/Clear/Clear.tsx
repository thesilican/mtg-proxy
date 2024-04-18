import { useState } from "react";
import { useAppDispatch, useAppSelector } from "../../state";
import { printAction } from "../../state/print";
import { Button } from "../common/Button/Button";
import { Dialog } from "../common/Dialog/Dialog";
import { buttonRow, cancel, dialog, message } from "./Clear.css";

export function Clear() {
  const dispatch = useAppDispatch();
  const cardCount = useAppSelector((s) => s.print.cards.length);
  const [dialogOpen, setDialogOpen] = useState(false);

  const handleClick = () => {
    dispatch(printAction.clear());
    setDialogOpen(false);
  };

  return (
    <>
      <Dialog
        className={dialog}
        open={dialogOpen}
        onClose={() => setDialogOpen(false)}
      >
        <p className={message}>Delete all cards?</p>
        <div className={buttonRow}>
          <Button variant="danger" onClick={handleClick}>
            Clear
          </Button>
          <Button className={cancel} onClick={() => setDialogOpen(false)}>
            Cancel
          </Button>
        </div>
      </Dialog>
      <Button
        variant="danger"
        disabled={cardCount === 0}
        onClick={() => setDialogOpen(true)}
      >
        Clear
      </Button>
    </>
  );
}
