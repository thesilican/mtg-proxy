import { useState } from "react";
import { useAppDispatch, useAppSelector } from "../../state";
import { printAction } from "../../state/print";
import { Button } from "../common/Button/Button";
import { Dialog } from "../common/Dialog/Dialog";
import * as styles from "./Clear.css";

export function Clear() {
  const dispatch = useAppDispatch();
  const cardCount = useAppSelector((s) => s.print.cards.length);
  const [open, setOpen] = useState(false);

  const handleClick = () => {
    dispatch(printAction.clear());
    setOpen(false);
  };

  return (
    <>
      <Dialog
        className={styles.dialog}
        open={open}
        onClose={() => setOpen(false)}
      >
        <p className={styles.message}>Remove all cards?</p>
        <div className={styles.buttonRow}>
          <Button variant="danger" onClick={handleClick}>
            Clear
          </Button>
          <Button className={styles.cancel} onClick={() => setOpen(false)}>
            Cancel
          </Button>
        </div>
      </Dialog>
      <Button
        variant="danger"
        disabled={cardCount === 0}
        onClick={() => setOpen(true)}
      >
        Clear
      </Button>
    </>
  );
}
