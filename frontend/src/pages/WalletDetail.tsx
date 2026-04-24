/**
 * WalletDetail Page
 * 
 * Demonstrates integration of QuickPivotLinks for wallet and contract navigation
 */

import React, { useState } from 'react';
import { QuickPivotLinks, type PivotLink } from '../components/v1/QuickPivotLinks';
import { StatusPill } from '../components/v1/StatusPill';

interface WalletDetailProps {
  walletId?: string;
}

const WalletDetail: React.FC<WalletDetailProps> = ({ walletId = 'wallet_123' }) => {
  const [activeSection, setActiveSection] = useState<string>('overview');

  // Mock data for demonstration
  const pivotLinks: PivotLink[] = [
    {
      id: 'contracts',
      label: 'Related Contracts',
      onClick: () => setActiveSection('contracts'),
      icon: '📄',
      badge: 5,
    },
    {
      id: 'transactions',
      label: 'Transaction History',
      onClick: () => setActiveSection('transactions'),
      icon: '💸',
      badge: 23,
    },
    {
      id: 'analytics',
      label: 'Analytics Dashboard',
      href: `/analytics/wallet/${walletId}`,
      icon: '📊',
      external: true,
    },
    {
      id: 'settings',
      label: 'Wallet Settings',
      onClick: () => setActiveSection('settings'),
      icon: '⚙️',
    },
    {
      id: 'export',
      label: 'Export Data',
      onClick: () => console.log('Export wallet data'),
      icon: '📤',
      disabled: true, // Example of disabled state
    },
  ];

  const renderContent = () => {
    switch (activeSection) {
      case 'contracts':
        return (
          <div className="wallet-detail__content">
            <h2>Related Smart Contracts</h2>
            <p>Contracts associated with this wallet...</p>
            <div style={{ display: 'grid', gap: '1rem', marginTop: '1rem' }}>
              <div style={{ padding: '1rem', border: '1px solid #333', borderRadius: '0.5rem' }}>
                <h3>Prize Pool Contract</h3>
                <p>Contract Address: 0x1234...5678</p>
                <StatusPill tone="success" label="Active" size="compact" />
              </div>
              <div style={{ padding: '1rem', border: '1px solid #333', borderRadius: '0.5rem' }}>
                <h3>Game Logic Contract</h3>
                <p>Contract Address: 0xabcd...efgh</p>
                <StatusPill tone="warning" label="Pending Update" size="compact" />
              </div>
            </div>
          </div>
        );
      case 'transactions':
        return (
          <div className="wallet-detail__content">
            <h2>Transaction History</h2>
            <p>Recent transactions for this wallet...</p>
            <div style={{ display: 'grid', gap: '0.5rem', marginTop: '1rem' }}>
              {Array.from({ length: 5 }, (_, i) => (
                <div key={i} style={{ 
                  display: 'flex', 
                  justifyContent: 'space-between', 
                  padding: '0.75rem', 
                  border: '1px solid #333', 
                  borderRadius: '0.25rem' 
                }}>
                  <span>Transaction #{i + 1}</span>
                  <StatusPill tone="success" label="Confirmed" size="compact" />
                </div>
              ))}
            </div>
          </div>
        );
      case 'settings':
        return (
          <div className="wallet-detail__content">
            <h2>Wallet Settings</h2>
            <p>Configure wallet preferences and security settings...</p>
          </div>
        );
      default:
        return (
          <div className="wallet-detail__content">
            <h2>Wallet Overview</h2>
            <div style={{ display: 'grid', gap: '1rem', marginTop: '1rem' }}>
              <div style={{ padding: '1rem', border: '1px solid #333', borderRadius: '0.5rem' }}>
                <h3>Balance</h3>
                <p style={{ fontSize: '1.5rem', fontWeight: 'bold', color: '#7de2d1' }}>
                  1,234.56 XLM
                </p>
              </div>
              <div style={{ padding: '1rem', border: '1px solid #333', borderRadius: '0.5rem' }}>
                <h3>Status</h3>
                <StatusPill tone="success" label="Active" />
              </div>
            </div>
          </div>
        );
    }
  };

  return (
    <div style={{ padding: '2rem', maxWidth: '1200px', margin: '0 auto' }}>
      <header style={{ marginBottom: '2rem' }}>
        <h1>Wallet Details</h1>
        <p style={{ color: '#a8b5c8', fontFamily: 'monospace' }}>
          {walletId}
        </p>
      </header>

      <div style={{ marginBottom: '2rem' }}>
        <h3 style={{ marginBottom: '1rem', fontSize: '0.875rem', textTransform: 'uppercase', letterSpacing: '0.05em' }}>
          Quick Navigation
        </h3>
        <QuickPivotLinks 
          links={pivotLinks}
          activeId={activeSection}
          testId="wallet-detail-pivot-links"
        />
      </div>

      {renderContent()}
    </div>
  );
};

export default WalletDetail;