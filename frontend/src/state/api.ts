import { createApi, fetchBaseQuery } from "@reduxjs/toolkit/query/react";

export type ApiAutocompleteResponse = {
  names: string[];
  exact: string[];
};

export type ImageUris = {
  large: string;
  png: string;
};

export type BackendCard = {
  id: string;
  name: string;
  images: ImageUris[];
  set: string;
  set_name: string;
  collector_number: string;
  released_at: string;
  preferred: boolean;
};

export type ApiCardResponse = {
  cards: BackendCard[];
};

export const scryfallApi = createApi({
  reducerPath: "api",
  baseQuery: fetchBaseQuery({ baseUrl: import.meta.env.BASE_URL }),
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

export const {
  useAutocompleteQuery,
  useLazyAutocompleteQuery,
  useCardQuery,
  useLazyCardQuery,
} = scryfallApi;

export function getPreferredCard(cards: BackendCard[]): BackendCard {
  for (const card of cards) {
    if (card.preferred) {
      return card;
    }
  }
  return cards[0];
}
