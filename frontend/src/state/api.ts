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
  baseQuery: fetchBaseQuery({ baseUrl: "/" }),
  endpoints: (builder) => ({
    autocomplete: builder.query<ApiAutocompleteResponse, string>({
      query: (query) => {
        const q = encodeURIComponent(query);
        return `/api/autocomplete?q=${q}`;
      },
    }),
    card: builder.query<ApiCardResponse, string>({
      query: (name) => {
        const q = encodeURIComponent(name);
        return `/api/search?q=${q}`;
      },
    }),
  }),
});

export const { useAutocompleteQuery, useCardQuery } = scryfallApi;

export function getDefaultVariant(cards: IScryfallCard[]): IScryfallCard {
  // Find the first card that's not promos
  const candidates = [];
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
      candidates.push(cards[i]);
    }
  }

  if (candidates.length === 0) {
    return cards[0];
  }

  return candidates.sort(
    (a, b) => (a.frame_effects?.length ?? 0) - (b.frame_effects?.length ?? 0)
  )[0];
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
