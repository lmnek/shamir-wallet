
export interface TransactionData {
    tx_id: string;
    senderAddress: string;
    sent: number;
    received: number;
    timestamp: number; 
}

export interface WalletData {
    name: string;
    balance: number;
    address: string;
}
