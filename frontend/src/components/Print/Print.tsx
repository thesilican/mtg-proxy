import { useEffect, useRef, useState } from "react";
import { useAppSelector } from "../../state";
import { Button } from "../common/Button/Button";
import { container, printRow, separator, statusMessage } from "./Print.css";
import type { WorkerRequest, WorkerResponse } from "./worker";
import PrintWorker from "./worker?worker";

export function Print() {
  const cards = useAppSelector((s) => s.print.cards);
  const [disabled, setDisabled] = useState(false);
  const [message, setMessage] = useState("");
  const workerRef = useRef<Worker>();

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
        link.download = "MTG Proxy.pdf";
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
    const request: WorkerRequest = {
      type: "print",
      cards: cards.map((card) => {
        const id = card.id;
        const face = card.face === 1 ? "back" : "front";
        return {
          count: card.quantity,
          url: `https://api.scryfall.com/cards/${id}?format=image&version=png&face=${face}`,
        };
      }),
    };
    workerRef.current?.postMessage(request);
    setDisabled(true);
  };

  return (
    <div className={container}>
      <div className={separator} />
      <div className={printRow}>
        <Button onClick={handleClick} disabled={cards.length === 0 || disabled}>
          Print
        </Button>
        <p className={statusMessage}>{message}</p>
      </div>
    </div>
  );
}
