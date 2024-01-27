
export interface TransactionData {
    senderAddress: string;
    sent: number;
    received: number;
    timestamp: number; 
}

export interface WalletData {
    name: string;
    balance: number;
    address: string;
    transactions: TransactionData[];
}
