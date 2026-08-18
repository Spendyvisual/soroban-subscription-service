import React from 'react';
import styles from './DataCard.module.css';

export type BadgeStatus = 'active' | 'inactive';

export interface DataCardProps {
  title: string;
  value?: string | number;
  subtitle?: string;
  badge?: { label: string; status: BadgeStatus };
  footer?: React.ReactNode;
  hoverable?: boolean;
  children?: React.ReactNode;
  onClick?: () => void;
}

export const DataCard: React.FC<DataCardProps> = ({
  title,
  value,
  subtitle,
  badge,
  footer,
  hoverable,
  children,
  onClick,
}) => (
  <div
    className={`${styles.card} ${hoverable ? styles.hoverable : ''}`}
    onClick={onClick}
    role={onClick ? 'button' : undefined}
    tabIndex={onClick ? 0 : undefined}
    onKeyDown={onClick ? (e) => e.key === 'Enter' && onClick() : undefined}
  >
    <div className={styles.header}>
      <h3 className={styles.title}>{title}</h3>
      {badge && (
        <span className={`${styles.badge} ${styles[badge.status]}`}>
          {badge.label}
        </span>
      )}
    </div>

    {value !== undefined && (
      <p className={styles.value}>{value}</p>
    )}
    {subtitle && <p className={styles.subtitle}>{subtitle}</p>}

    {children}

    {footer && <div className={styles.footer}>{footer}</div>}
  </div>
);
