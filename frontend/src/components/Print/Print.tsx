import { useState } from "react";
import { useAppSelector } from "../../state";
import { resolveLocalUrl } from "../../util";
import { Button } from "../common/Button/Button";
import { container, printRow, separator } from "./Print.css";

export function Print() {
  const cards = useAppSelector((s) => s.print.cards);
  const [message, setMessage] = useState("Print");

  const handleClick = async () => {
    const cardIds = cards.flatMap((x) =>
      Array(x.quantity).fill(x.ids[x.variant])
    );
    setMessage("Printing...");
    const result = await fetch(resolveLocalUrl("api/print"), {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        cards: cardIds,
      }),
    }).catch((err) => {
      console.error(err);
      setMessage("Error printing");
      return null;
    });
    if (!result) {
      return;
    }
    setMessage("Print");
    const blob = await result.blob();
    const pdf = new Blob([blob], { type: "application/pdf" });
    const pdfUrl = URL.createObjectURL(pdf);
    const link = document.createElement("a");
    link.download = "MTG Proxy.pdf";
    link.href = pdfUrl;
    link.click();
  };

  return (
    <div className={container}>
      <div className={separator} />
      <div className={printRow}>
        <Button onClick={handleClick} disabled={cards.length === 0}>
          {message}
        </Button>
      </div>
    </div>
  );
}
