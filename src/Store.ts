import { writable, type Writable } from 'svelte/store'
import { type WalletData } from './Types'

export const walletsNamesStore: Writable<string[]> = writable([])
export const walletsCacheStore: Writable<WalletData[]> = writable([])

