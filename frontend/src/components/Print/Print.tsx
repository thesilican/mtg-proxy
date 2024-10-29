import { useEffect, useRef, useState } from "react";
import { useAppSelector } from "../../state";
import { Button } from "../common/Button/Button";
import { container, statusMessage } from "./Print.css";
import type {
  WorkerRequest,
  WorkerRequestCard,
  WorkerResponse,
} from "./worker";
import PrintWorker from "./worker?worker";
import { useLazyCardQuery } from "../../state/api";
import { QueryStatus } from "@reduxjs/toolkit/query";

export function Print() {
  const print = useAppSelector((s) => s.print);
  const [disabled, setDisabled] = useState(false);
  const [message, setMessage] = useState("");
  const workerRef = useRef<Worker>();

  const [fetchCards] = useLazyCardQuery();

  useEffect(() => {
    const worker = new PrintWorker();
    workerRef.current = worker;
    const handler = (event: MessageEvent<WorkerResponse>) => {
      const message = event.data;
      if (message.type === "progess") {
        setMessage(message.message);
      } else if (message.type === "success") {
        const pdfUrl = URL.createObjectURL(message.data);
        const link = document.createElement("a");
        if (message.part !== null) {
          link.download = `MTG Proxy ${message.part + 1}.pdf`;
        } else {
          link.download = "MTG Proxy.pdf";
        }
        link.href = pdfUrl;
        link.click();
        link.remove();
        setMessage(`Done!`);
        setDisabled(false);
      } else if (message.type === "failed") {
        setDisabled(false);
      }
    };
    worker.addEventListener("message", handler);
    return () => {
      worker.removeEventListener("message", handler);
      worker.terminate();
    };
  }, []);

  const handleClick = async () => {
    const reqCards: WorkerRequestCard[] = [];
    for (let i = 0; i < print.cards.length; i++) {
      const card = print.cards[i];
      setMessage(`Downloading card metadata (${i + 1} / ${print.cards.length})`);
      const result = await fetchCards(card.name);
      if (result.status !== QueryStatus.fulfilled) {
        setMessage("Error fetching card data");
        return;
      }
      for (const cardData of result.data.cards) {
        if (cardData.id === card.id) {
          const url =
            card.face === 0
              ? cardData.image_front_png
              : cardData.image_back_png!;
          reqCards.push({ count: card.quantity, url });
        }
      }
    }
    const request: WorkerRequest = {
      type: "print",
      cards: reqCards,
      split: print.split,
    };
    workerRef.current?.postMessage(request);
    setDisabled(true);
  };

  return (
    <div className={container}>
      <p className={statusMessage}>{message}</p>
      <Button
        onClick={handleClick}
        disabled={print.cards.length === 0 || disabled}
      >
        Print
      </Button>
    </div>
  );
}
