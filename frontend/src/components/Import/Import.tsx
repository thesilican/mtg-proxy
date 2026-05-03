import { Fragment, useState } from "react";
import { Action } from "redux";
import { useAppDispatch } from "../../state";
import { ImportCard, useLazyImportQuery } from "../../state/api";
import { printAction } from "../../state/print";
import { Button } from "../common/Button/Button";
import { Dialog } from "../common/Dialog/Dialog";
import * as styles from "./Import.css";

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
  const [text, setText] = useState("");
  const [errors, setErrors] = useState<string[]>([]);
  const [processing, setProcessing] = useState(false);

  const [fetchImport] = useLazyImportQuery();

  const handleImport = async () => {
    setErrors([]);
    setProcessing(true);
    const errors = [];
    try {
      const counts = [];
      const cards: ImportCard[] = [];
      for (const line of text.split("\n")) {
        const trimmed = line.trim();
        if (
          trimmed.startsWith("#") ||
          trimmed.length === 0 ||
          trimmed === "SIDEBOARD:"
        ) {
          continue;
        }
        const match = line.match(
          /^(?:(\d+)x?\s+)?([^(]+)(?:\(([a-zA-Z0-9]+)\))?(?:\s*([-a-zA-Z-0-9_★†Φ]+))?[^(]*$/,
        );
        if (match === null) {
          errors.push(`Invalid format: ${JSON.stringify(line)}`);
          continue;
        }
        const count = match[1] ? parseInt(match[1], 10) : 1;
        const name = match[2].trim();
        const set = match[3];
        const collector_number = match[4];
        counts.push(count);
        cards.push({ set, name, collector_number });
      }
      const { data } = await fetchImport({ cards });
      if (!data) {
        errors.push("Unable to connect to server.");
        return;
      }
      if (data.results.length !== counts.length) {
        errors.push("There was an error importing cards (length mismatch).");
        return;
      }
      const actions: Action[] = [];
      for (let i = 0; i < data.results.length; i++) {
        const result = data.results[i];
        if (!result.success) {
          errors.push(result.message);
        } else {
          const card = result.card;
          actions.push(
            printAction.add({
              id: card.id,
              face: "front",
              name: card.name,
              quantity: counts[i],
            }),
          );
        }
      }
      if (errors.length === 0) {
        for (const action of actions) {
          dispatch(action);
        }
        setDialogOpen(false);
        setText("");
      }
    } catch (error) {
      errors.push(`Error importing cards: ${error}`);
    } finally {
      setErrors(errors);
      setProcessing(false);
    }
  };

  return (
    <>
      <Dialog open={dialogOpen} onClose={() => setDialogOpen(false)}>
        <form
          onSubmit={(e) => {
            e.preventDefault();
            handleImport();
          }}
        >
          <textarea
            className={styles.textarea}
            placeholder={placeholder}
            value={text}
            onChange={(e) => {
              setText(e.target.value);
              setErrors([]);
            }}
          />
          {errors.length > 0 && (
            <p className={styles.error}>
              {errors.map((x, i) => (
                <Fragment key={i}>
                  {i !== 0 && <br />}
                  {x}
                </Fragment>
              ))}
            </p>
          )}
          <div className={styles.buttonRow}>
            <Button type="submit" disabled={processing}>
              {processing ? "Importing..." : "Import"}
            </Button>
            <Button variant="secondary" onClick={() => setDialogOpen(false)}>
              Cancel
            </Button>
          </div>
        </form>
      </Dialog>
      <Button onClick={() => setDialogOpen(true)}>Import</Button>
    </>
  );
}
