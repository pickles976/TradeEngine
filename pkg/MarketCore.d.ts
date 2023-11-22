/* tslint:disable */
/* eslint-disable */
/**
* @returns {string}
*/
export function test(): string;
/**
*/
export class MarketWrapper {
  free(): void;
/**
* @returns {MarketWrapper}
*/
  static new(): MarketWrapper;
/**
* @returns {string}
*/
  test(): string;
/**
* @param {string} order_request_str
* @returns {string}
*/
  test_serialization(order_request_str: string): string;
/**
* @param {string} order_request_str
* @returns {string}
*/
  buy(order_request_str: string): string;
/**
* @param {string} order_request_str
* @returns {string}
*/
  sell(order_request_str: string): string;
/**
* @param {string} item
* @returns {string}
*/
  query_ledger(item: string): string;
/**
* @returns {string}
*/
  dump(): string;
/**
* @param {string} item
* @param {string} order
* @returns {string}
*/
  cancel_order(item: string, order: string): string;
/**
* @param {string} item
* @returns {string}
*/
  get_best_buying_price(item: string): string;
/**
* @param {string} item
* @returns {string}
*/
  get_best_selling_price(item: string): string;
}
