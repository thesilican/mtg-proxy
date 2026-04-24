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
import { useLazyCardsQuery } from "../../state/api";
import { QueryStatus } from "@reduxjs/toolkit/query";

export function Print() {
  const print = useAppSelector((s) => s.print);
  const [disabled, setDisabled] = useState(false);
  const [message, setMessage] = useState("");
  const workerRef = useRef<Worker>();

  const [fetchCards] = useLazyCardsQuery();

  useEffect(() => {
    const worker = new PrintWorker();
    workerRef.current = worker;
    const handler = (event: MessageEvent<WorkerResponse>) => {
      const message = event.data;
      if (message.type === "progess") {
        setMessage(message.message);
      } else if (message.type === "success") {
        const url = URL.createObjectURL(message.data);
        const link = document.createElement("a");
        if (message.fileType === "pdf") {
          link.download = `MTG Proxy.pdf`;
        } else {
          link.download = "MTG Proxy.zip";
        }
        link.href = url;
        link.target = "_blank";
        link.click();
        link.remove();
        URL.revokeObjectURL(url);
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
    const { data, status } = await fetchCards({
      ids: print.cards.map((x) => x.id),
    });
    if (status !== QueryStatus.fulfilled) {
      setMessage("Error fetching card data");
      return;
    }
    const cardMap = new Map(data.cards.map((x) => [x.id, x]));
    const reqCards: WorkerRequestCard[] = [];
    for (const card of print.cards) {
      const info = cardMap.get(card.id);
      if (!info) {
        setMessage(`Error fetching card data (${card.id})`);
        return;
      }
      let url: string;
      if (card.face === "front") {
        url = info.images.front_png;
      } else {
        if (!info.images.back_png) {
          setMessage(
            `Error fetching card data (${card.id} missing back image)`,
          );
          return;
        }
        url = info.images.back_png;
      }
      reqCards.push({ count: card.quantity, url });
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
