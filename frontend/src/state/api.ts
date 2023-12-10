import { createApi, fetchBaseQuery } from "@reduxjs/toolkit/query/react";
import { IScryfallCard } from "scryfall-types";
import errorPng from "../assets/error.png";

export type ApiAutocompleteResponse = {
  data: string[];
};

export type ApiCardResponse = {
  data: IScryfallCard[];
};

export const scryfallApi = createApi({
  reducerPath: "api",
  baseQuery: fetchBaseQuery({ baseUrl: "https://api.scryfall.com" }),
  endpoints: (builder) => ({
    autocomplete: builder.query<ApiAutocompleteResponse, string>({
      query: (query) => {
        const q = encodeURIComponent(query);
        return `/cards/autocomplete?q=${q}&include_extras=true`;
      },
    }),
    card: builder.query<ApiCardResponse, string>({
      query: (name) => {
        const query = `!${JSON.stringify(name)}`;
        const q = encodeURIComponent(query);
        return (
          `/cards/search?q=${q}` +
          `&unique=prints&include_multilingual=false&include_extras=true&order=released`
        );
      },
    }),
  }),
});

export const { useAutocompleteQuery, useCardQuery } = scryfallApi;

export function getDefaultVariantIdx(cards: IScryfallCard[]) {
  // Find the first card that's not promos
  const candidates: { card: IScryfallCard; idx: number }[] = [];
  for (let i = 0; i < cards.length; i++) {
    const promo = cards[i].promo;
    const variation = cards[i].variation;
    const layout = cards[i].layout;
    if (
      !promo &&
      !variation &&
      ![
        "planar",
        "scheme",
        "vanguard",
        "token",
        "double_faced_token",
        "emblem",
        "art_series",
      ].includes(layout)
    ) {
      candidates.push({ card: cards[i], idx: i });
    }
  }

  if (candidates.length === 0) {
    return 0;
  }

  return candidates.sort(
    (a, b) =>
      (a.card.frame_effects?.length ?? 0) - (b.card.frame_effects?.length ?? 0)
  )[0].idx;
}

export function getImageUrl(card: IScryfallCard, faceIdx = 0) {
  if (card.image_uris) {
    return card.image_uris.large;
  } else if (card.card_faces) {
    const face = card.card_faces[faceIdx];
    if (face && face.image_uris) {
      return face.image_uris.large;
    }
  }
  return errorPng;
}
