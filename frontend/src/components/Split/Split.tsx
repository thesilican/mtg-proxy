import { ChangeEvent, useEffect, useState } from "react";
import { Button } from "../common/Button/Button";
import { Dialog } from "../common/Dialog/Dialog";
import { Input } from "../common/Input/Input";
import { buttonRow, close, dialog, disabled, helperText, numberInput } from "./Split.css";
import { useAppDispatch, useAppSelector } from "../../state";
import { printAction } from "../../state/print";
import cn from "classnames";

const DEFAULT_SPLIT_COUNT = 3;

export function Split() {
  const dispatch = useAppDispatch();
  const [open, setOpen] = useState(false);
  const split = useAppSelector((s) => s.print.split);
  const [splitCount, setSplitCount] = useState(DEFAULT_SPLIT_COUNT.toString());

  useEffect(() => {
    if (split) {
      setSplitCount(split.toString());
    }
  }, [open, split]);

  const handleSplitPdfsClick = () => {
    let value = parseInt(splitCount);
    if (isNaN(value)) {
      value = DEFAULT_SPLIT_COUNT;
      setSplitCount(DEFAULT_SPLIT_COUNT.toString());
    }
    if (split) {
      dispatch(printAction.setSplit(null));
    } else {
      dispatch(printAction.setSplit(value));
    }
  };

  const handleSplitCountsChange = (e: ChangeEvent<HTMLInputElement>) => {
    setSplitCount(e.target.value);
    const value = e.target.valueAsNumber;
    if (!isNaN(value)) {
      dispatch(printAction.setSplit(value));
    }
  };

  return (
    <>
      <Dialog className={dialog} open={open} onClose={() => setOpen(false)}>
        <label>
          Split PDFs:{" "}
          <input
            type="checkbox"
            checked={!!split}
            onChange={handleSplitPdfsClick}
          />
        </label>
        <label className={cn(!split && disabled)}>
          Pages per PDF:{" "}
          <Input
            disabled={!split}
            type="number"
            className={numberInput}
            value={splitCount}
            min={1}
            max={100}
            onChange={handleSplitCountsChange}
          />
        </label>
        <p className={helperText}>{split ? `(~${split * 10}MB per PDF)` : ""}</p>
        <div className={buttonRow}>
          <Button className={close} onClick={() => setOpen(false)}>
            Close
          </Button>
        </div>
      </Dialog>
      <Button onClick={() => setOpen(true)}>Split</Button>
    </>
  );
}
