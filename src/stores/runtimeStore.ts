import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';
import { type RuntimeId, RUNTIME_LABELS } from '../lib/types/runtime';

export interface RuntimeInfo {
  id: RuntimeId;
  name: string;
  detected: boolean;
  version: string;
}

interface RuntimeState {
  runtimes: RuntimeInfo[];
  selectedRuntimeId: RuntimeId;
  isLoading: boolean;

  fetchRuntimes: () => Promise<void>;
  setSelectedRuntime: (id: RuntimeId) => void;
  getRuntimeName: (id: RuntimeId) => string;
}

export const useRuntimeStore = create<RuntimeState>()((set, get) => ({
  runtimes: [],
  selectedRuntimeId: 'claude',
  isLoading: false,

  fetchRuntimes: async () => {
    set({ isLoading: true });
    try {
      const runtimes = await invoke<RuntimeInfo[]>('list_runtimes');
      set({ runtimes, isLoading: false });
    } catch (e) {
      console.error('[runtimeStore] fetchRuntimes failed:', e);
      set({ isLoading: false });
    }
  },

  setSelectedRuntime: (id) => {
    set({ selectedRuntimeId: id });
  },

  getRuntimeName: (id) => {
    const { runtimes } = get();
    const found = runtimes.find((r) => r.id === id);
    if (found) return found.name;
    return RUNTIME_LABELS[id] ?? id;
  },
}));
