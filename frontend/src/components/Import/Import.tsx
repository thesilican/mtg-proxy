import { QueryStatus } from "@reduxjs/toolkit/query";
import {
  ChangeEvent,
  Fragment,
  useCallback,
  useEffect,
  useRef,
  useState,
} from "react";
import { useAppDispatch } from "../../state";
import {
  getPreferredCard,
  useLazyAutocompleteQuery,
  useLazyCardQuery,
} from "../../state/api";
import { printAction } from "../../state/print";
import { Button } from "../common/Button/Button";
import { Dialog } from "../common/Dialog/Dialog";
import { buttonRow, error, textarea } from "./Import.css";

const placeholder = `# Import cards in any of the following formats:
Treasure Cruise
Ledger Shredder (SNC)
Consider (MID) 44
2 Thing in the Ice
4 Arclight Phoenix (RVR)
4 Opt (ELD) 49`;

export function Import() {
  const dispatch = useAppDispatch();
  const [dialogOpen, setDialogOpen] = useState(false);
  const [processing, setProcessing] = useState(false);
  const [text, setText] = useState("");
  const [errors, setErrors] = useState<string[]>([]);
  const ref = useRef<HTMLTextAreaElement>(null);

  const [fetchAutocomplete] = useLazyAutocompleteQuery();
  const [fetchCard] = useLazyCardQuery();

  const handleChange = (e: ChangeEvent<HTMLTextAreaElement>) => {
    setText(e.target.value);
    if (errors) {
      setErrors([]);
    }
  };

  const handleImport = useCallback(async () => {
    setProcessing(true);
    const lines = text.split("\n");
    const errors = [];
    const cards = [];
    for (let i = 0; i < lines.length; i++) {
      const line = lines[i].trim();
      if (line.length === 0) {
        continue;
      }
      if (line.startsWith("#")) {
        continue;
      }
      const match = line.match(
        /^(\d+\s+)?([^(]+)(?:\s+\(([a-zA-Z0-9]+)\))?(?:\s+([a-zA-Z-0-9]+))?$/
      );
      if (match === null) {
        errors.push(`Invalid format: ${JSON.stringify(line)}`);
        continue;
      }
      const count = match[1] ? parseInt(match[1], 10) : 1;
      const name = match[2].trim();
      const setName = match[3];
      const collectorsNumber = match[4];
      const result = await fetchAutocomplete(name);
      if (
        result.status !== QueryStatus.fulfilled ||
        result.data.exact.length === 0
      ) {
        let error = `Unknown card: ${JSON.stringify(name)}`;
        if (result.data && result.data.names.length > 0) {
          error += ` (did you mean '${result.data.names[0]}'?)`;
        }
        errors.push(error);
        continue;
      }
      const cardName = result.data.exact[0];
      const cardResult = await fetchCard(cardName);
      if (cardResult.status !== QueryStatus.fulfilled) {
        errors.push(`Unknown card: ${JSON.stringify(name)}`);
        continue;
      }
      const cardVariants = cardResult.data.cards;
      let preferredCard = getPreferredCard(cardResult.data.cards);
      for (const card of cardVariants) {
        if (
          card.set.toLowerCase() === setName?.toLowerCase() &&
          (!collectorsNumber || card.collector_number == collectorsNumber)
        ) {
          preferredCard = card;
        }
      }
      cards.push({ count, card: preferredCard });
    }
    if (errors.length !== 0) {
      setErrors(errors);
      setProcessing(false);
    } else {
      for (const card of cards) {
        dispatch(
          printAction.add({
            id: card.card.id,
            face: 0,
            name: card.card.name,
            quantity: card.count,
          })
        );
      }
      setProcessing(false);
      setErrors([]);
      setText("");
      setDialogOpen(false);
    }
  }, [dispatch, fetchAutocomplete, fetchCard, text]);

  useEffect(() => {
    const textarea = ref.current;
    const handler = (e: KeyboardEvent) => {
      if ((e.ctrlKey || e.metaKey) && e.code === "Enter") {
        e.preventDefault();
        handleImport();
      }
    };
    textarea?.addEventListener("keydown", handler);
    return () => textarea?.removeEventListener("keydown", handler);
  }, [handleImport]);

  return (
    <>
      <Dialog open={dialogOpen} onClose={() => setDialogOpen(false)}>
        <textarea
          ref={ref}
          className={textarea}
          placeholder={placeholder}
          value={text}
          onChange={handleChange}
        />
        {errors.length > 0 && (
          <p className={error}>
            {errors.map((x, i) => (
              <Fragment key={i}>
                {i !== 0 && <br />}
                {x}
              </Fragment>
            ))}
          </p>
        )}
        <div className={buttonRow}>
          <Button onClick={handleImport} disabled={processing}>
            Import
          </Button>
          <Button variant="secondary" onClick={() => setDialogOpen(false)}>
            Cancel
          </Button>
        </div>
      </Dialog>
      <Button onClick={() => setDialogOpen(true)}>Import</Button>
    </>
  );
}
