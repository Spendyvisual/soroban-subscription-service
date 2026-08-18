import React, { type InputHTMLAttributes, type SelectHTMLAttributes, type TextareaHTMLAttributes } from 'react';
import styles from './FormInput.module.css';

interface BaseProps {
  label?: string;
  error?: string;
}

export interface InputProps extends BaseProps, InputHTMLAttributes<HTMLInputElement> {}
export interface SelectProps extends BaseProps, SelectHTMLAttributes<HTMLSelectElement> {
  children: React.ReactNode;
}
export interface TextareaProps extends BaseProps, TextareaHTMLAttributes<HTMLTextAreaElement> {}
export interface ToggleProps {
  label: string;
  checked: boolean;
  onChange: (checked: boolean) => void;
  id: string;
}

export const Input: React.FC<InputProps> = ({ label, error, className, id, ...props }) => (
  <div className={styles.formGroup}>
    {label && <label htmlFor={id} className={styles.label}>{label}</label>}
    <input
      id={id}
      className={`${styles.input} ${error ? styles.error : ''} ${className || ''}`}
      {...props}
    />
    {error && <span className={styles.errorMsg}>{error}</span>}
  </div>
);

export const Select: React.FC<SelectProps> = ({ label, error, children, className, id, ...props }) => (
  <div className={styles.formGroup}>
    {label && <label htmlFor={id} className={styles.label}>{label}</label>}
    <select
      id={id}
      className={`${styles.select} ${error ? styles.error : ''} ${className || ''}`}
      {...props}
    >
      {children}
    </select>
    {error && <span className={styles.errorMsg}>{error}</span>}
  </div>
);

export const Textarea: React.FC<TextareaProps> = ({ label, error, className, id, ...props }) => (
  <div className={styles.formGroup}>
    {label && <label htmlFor={id} className={styles.label}>{label}</label>}
    <textarea
      id={id}
      className={`${styles.textarea} ${error ? styles.error : ''} ${className || ''}`}
      rows={4}
      {...props}
    />
    {error && <span className={styles.errorMsg}>{error}</span>}
  </div>
);

export const Toggle: React.FC<ToggleProps> = ({ label, checked, onChange, id }) => (
  <label htmlFor={id} className={styles.toggle}>
    <input
      id={id}
      type="checkbox"
      className={styles.toggleInput}
      checked={checked}
      onChange={(e) => onChange(e.target.checked)}
    />
    <span className={styles.toggleTrack} />
    <span className={styles.toggleThumb} />
    {label}
  </label>
);
