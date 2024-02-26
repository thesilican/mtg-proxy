import { useState } from "react";
import { useAppSelector } from "../../state";
import { resolveLocalUrl } from "../../util";
import { Button } from "../common/Button/Button";
import { container, printRow, separator } from "./Print.css";

export function Print() {
  const cards = useAppSelector((s) => s.print.cards);
  const [message, setMessage] = useState("Print");

  const handleClick = async () => {
    const cardOptions = cards.map((x) => ({
      id: x.ids[x.variant],
      face: x.face === 0 ? "front" : "back",
      quantity: x.quantity,
    }));
    setMessage("Printing...");
    const result = await fetch(resolveLocalUrl("api/print"), {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        cards: cardOptions,
      }),
    }).catch((err) => {
      console.error(err);
      return null;
    });
    if (!result || result.status !== 200) {
      setMessage("Error printing");
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
