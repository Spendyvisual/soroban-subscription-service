import React, { createContext, useContext, useCallback, useEffect, type ReactNode } from 'react';
import {
  getAddress,
  getNetwork,
  isConnected,
  setAllowed,
} from '@stellar/freighter-api';
import { useAppStore } from '../store';

interface WalletContextValue {
  walletAddress: string | null;
  network: string | null;
  connect: () => Promise<void>;
  disconnect: () => void;
  isFreighterInstalled: boolean;
}

const WalletContext = createContext<WalletContextValue | null>(null);

export const WalletProvider: React.FC<{ children: ReactNode }> = ({ children }) => {
  const { walletAddress, network, setWallet, disconnectWallet, addToast } = useAppStore();

  const checkInstalled = async (): Promise<boolean> => {
    try {
      const result = await isConnected();
      return result.isConnected;
    } catch {
      return false;
    }
  };

  const connect = useCallback(async () => {
    const installed = await checkInstalled();
    if (!installed) {
      addToast('error', 'Freighter wallet not found. Please install it from freighter.app');
      return;
    }
    try {
      await setAllowed();
      const addrResult = await getAddress();
      const netResult = await getNetwork();
      if (addrResult.error || netResult.error) {
        addToast('error', 'Could not connect. Please approve the request in Freighter.');
        return;
      }
      setWallet(addrResult.address, netResult.network);
      addToast('success', `Wallet connected: ${addrResult.address.slice(0, 6)}...${addrResult.address.slice(-4)}`);
    } catch {
      addToast('error', 'Failed to connect wallet. Please try again.');
    }
  }, [setWallet, addToast]);

  const disconnect = useCallback(() => {
    disconnectWallet();
    addToast('info', 'Wallet disconnected.');
  }, [disconnectWallet, addToast]);

  useEffect(() => {
    (async () => {
      const installed = await checkInstalled();
      if (!installed) return;
      try {
        const addrResult = await getAddress();
        const netResult = await getNetwork();
        if (!addrResult.error && !netResult.error && addrResult.address) {
          setWallet(addrResult.address, netResult.network);
        }
      } catch {
        // silently fail on auto-reconnect
      }
    })();
  }, [setWallet]);

  return (
    <WalletContext.Provider value={{ walletAddress, network, connect, disconnect, isFreighterInstalled: true }}>
      {children}
    </WalletContext.Provider>
  );
};

export const useWallet = (): WalletContextValue => {
  const ctx = useContext(WalletContext);
  if (!ctx) throw new Error('useWallet must be used inside WalletProvider');
  return ctx;
};
