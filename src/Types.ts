
export interface TransactionData {
    tx_id: string;
    sent: number;
    received: number;
    fee: number | null;
}

export interface WalletData {
    name: string;
    balance: number;
    address: string;
}
