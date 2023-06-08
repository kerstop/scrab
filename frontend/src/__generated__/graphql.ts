/* eslint-disable */
import { TypedDocumentNode as DocumentNode } from '@graphql-typed-document-node/core';
export type Maybe<T> = T | null;
export type InputMaybe<T> = Maybe<T>;
export type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]?: Maybe<T[SubKey]> };
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]: Maybe<T[SubKey]> };
export type MakeEmpty<T extends { [key: string]: unknown }, K extends keyof T> = { [_ in K]?: never };
export type Incremental<T> = T | { [P in keyof T]?: P extends ' $fragmentName' | '__typename' ? T[P] : never };
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: { input: string | number; output: string; }
  String: { input: string; output: string; }
  Boolean: { input: boolean; output: boolean; }
  Int: { input: number; output: number; }
  Float: { input: number; output: number; }
};

export type Cordinate = {
  __typename?: 'Cordinate';
  q: Scalars['Int']['output'];
  r: Scalars['Int']['output'];
  s: Scalars['Int']['output'];
};

export type QueryRoot = {
  __typename?: 'QueryRoot';
  /** Information about the game world */
  world: World;
};

export type Room = {
  __typename?: 'Room';
  cordinate: Cordinate;
  tiles: Array<Tile>;
};

export type Tile = {
  __typename?: 'Tile';
  cordinate: Cordinate;
  isWall: Scalars['Boolean']['output'];
};

export type World = {
  __typename?: 'World';
  room: Room;
};


export type WorldRoomArgs = {
  q: Scalars['Int']['input'];
  r: Scalars['Int']['input'];
  s: Scalars['Int']['input'];
};

export type GetRoomInfoQueryVariables = Exact<{ [key: string]: never; }>;


export type GetRoomInfoQuery = { __typename?: 'QueryRoot', world: { __typename?: 'World', room: { __typename?: 'Room', cordinate: { __typename?: 'Cordinate', q: number, r: number, s: number }, tiles: Array<{ __typename?: 'Tile', isWall: boolean, cordinate: { __typename?: 'Cordinate', q: number, r: number, s: number } }> } } };


export const GetRoomInfoDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"GetRoomInfo"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"world"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"room"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"q"},"value":{"kind":"IntValue","value":"0"}},{"kind":"Argument","name":{"kind":"Name","value":"r"},"value":{"kind":"IntValue","value":"0"}},{"kind":"Argument","name":{"kind":"Name","value":"s"},"value":{"kind":"IntValue","value":"0"}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"cordinate"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"q"}},{"kind":"Field","name":{"kind":"Name","value":"r"}},{"kind":"Field","name":{"kind":"Name","value":"s"}}]}},{"kind":"Field","name":{"kind":"Name","value":"tiles"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"isWall"}},{"kind":"Field","name":{"kind":"Name","value":"cordinate"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"q"}},{"kind":"Field","name":{"kind":"Name","value":"r"}},{"kind":"Field","name":{"kind":"Name","value":"s"}}]}}]}}]}}]}}]}}]} as unknown as DocumentNode<GetRoomInfoQuery, GetRoomInfoQueryVariables>;