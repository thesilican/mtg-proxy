import { PrintJob } from "mtg-print";

export type WorkerRequestCard = {
  count: number;
  url: string;
};

export type WorkerRequest = {
  type: "print";
  cards: WorkerRequestCard[];
};

export type WorkerResponse =
  | {
      type: "progess";
      message: string;
    }
  | {
      type: "success";
      data: Blob;
    }
  | {
      type: "failed";
      message: string;
    };

function progress(message: string) {
  self.postMessage({
    type: "progess",
    message,
  } as WorkerResponse);
}

self.addEventListener("message", async (event: MessageEvent<WorkerRequest>) => {
  const job = new PrintJob();
  job.add_callback((message: string) => {
    progress(message);
  });
  try {
    const cards = event.data.cards;
    for (let i = 0; i < cards.length; i++) {
      progress(`Downloading images (${i + 1} / ${cards.length})`);
      const image = await fetch(cards[i].url);
      const png = new Uint8Array(await (await image.blob()).arrayBuffer());
      job.add_card(cards[i].count, png);
    }
    const output = job.run();
    const pdf = new Blob([output], { type: "application/pdf" });
    self.postMessage({
      type: "success",
      data: pdf,
    } as WorkerResponse);
  } catch (error) {
    self.postMessage({
      type: "failed",
      message: `Error: ${error}`,
    } as WorkerResponse);
  } finally {
    job.free();
  }
});
