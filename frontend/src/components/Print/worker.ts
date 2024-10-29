import { PrintJob } from "mtg-print";
import { chunk } from "../../util";

export type WorkerRequestCard = {
  count: number;
  url: string;
};

export type WorkerRequest = {
  type: "print";
  cards: WorkerRequestCard[];
  split: number | null;
};

export type WorkerResponse =
  | {
      type: "progess";
      message: string;
    }
  | {
      type: "success";
      data: Blob;
      part: number | null;
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

async function downloadImages(
  cards: WorkerRequestCard[],
): Promise<Uint8Array[]> {
  progress(`Downloading images (0 / ${cards.length})`);
  let count = 0;
  const cardPngs = [];
  const jobs = cards.map((card) =>
    fetch(card.url)
      .then((result) => result.arrayBuffer())
      .then((buffer) => {
        progress(`Downloading images (${++count} / ${cards.length})`);
        return new Uint8Array(buffer);
      }),
  );
  // Download in groups of 8
  const chunks = chunk(jobs, 8);
  for (const chunk of chunks) {
    const results = await Promise.all(chunk);
    cardPngs.push(...results);
  }
  return cardPngs;
}

self.addEventListener("message", async (event: MessageEvent<WorkerRequest>) => {
  const cards = event.data.cards;
  const split = event.data.split;
  let cardPngs;
  try {
    cardPngs = await downloadImages(cards);
  } catch (error) {
    self.postMessage({
      type: "failed",
      message: `Error: ${error}`,
    } as WorkerResponse);
    return;
  }

  const cardCount = cards.reduce((a, v) => a + v.count, 0);
  const jobSize = split ? split * 9 : cardCount;
  type Part = { id: number; count: number };
  const partitions: Part[][] = [];
  let partition: Part[] = [];
  let counter = 0;
  for (let id = 0; id < cards.length; id++) {
    for (let j = 0; j < cards[id].count; j++) {
      if (partition.length && partition[partition.length - 1].id == id) {
        partition[partition.length - 1].count++;
      } else {
        partition.push({ id, count: 1 });
      }
      counter++;
      if (counter === jobSize) {
        counter = 0;
        partitions.push(partition);
        partition = [];
      }
    }
  }
  if (partition.length > 0) {
    partitions.push(partition);
  }

  for (let i = 0; i < partitions.length; i++) {
    const job = new PrintJob();
    try {
      for (const { count, id } of partitions[i]) {
        job.add_card(count, cardPngs[id]);
      }
      job.add_callback((message: string) => {
        progress(message);
      });
      const output = job.run();
      const pdf = new Blob([output], { type: "application/pdf" });
      self.postMessage({
        type: "success",
        data: pdf,
        part: split ? i : null,
      } as WorkerResponse);
    } catch (error) {
      self.postMessage({
        type: "failed",
        message: `Error: ${error}`,
      } as WorkerResponse);
      return;
    } finally {
      job.free();
    }
  }
});
