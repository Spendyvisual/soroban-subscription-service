import React from 'react';
import { createPortal } from 'react-dom';
import { useAppStore } from '../store';
import styles from './Toast.module.css';

export const ToastContainer: React.FC = () => {
  const { toasts, removeToast } = useAppStore();

  if (toasts.length === 0) return null;

  return createPortal(
    <div className={styles.toastContainer}>
      {toasts.map((toast) => (
        <div key={toast.id} className={`${styles.toast} ${styles[toast.type]}`}>
          <span>{toast.message}</span>
          <button className={styles.closeBtn} onClick={() => removeToast(toast.id)}>
            &times;
          </button>
        </div>
      ))}
    </div>,
    document.body
  );
};
