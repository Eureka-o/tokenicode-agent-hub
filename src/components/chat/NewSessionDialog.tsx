import { useState, useEffect, useRef } from 'react';
import { open as openDialog } from '@tauri-apps/plugin-dialog';
import { useProviderStore } from '../../stores/providerStore';
import { type RuntimeId, RUNTIME_LABELS } from '../../lib/types/runtime';
import { bridge } from '../../lib/tauri-bridge';

export interface NewSessionConfig {
  runtimeId: RuntimeId;
  providerId: string;
  model: string;
  cwd: string;
  permissionMode: 'ask' | 'plan' | 'auto' | 'bypass';
}

interface NewSessionDialogProps {
  open: boolean;
  onClose: () => void;
  onCreate: (config: NewSessionConfig) => void;
}

type RuntimeStatus = 'checking' | 'detected' | 'not-detected';

const RUNTIME_IDS: RuntimeId[] = ['claude', 'codex', 'gemini', 'opencode', 'custom'];

const PERMISSION_MODES: { id: NewSessionConfig['permissionMode']; label: string }[] = [
  { id: 'ask', label: 'Ask' },
  { id: 'plan', label: 'Plan' },
  { id: 'auto', label: 'Auto' },
  { id: 'bypass', label: 'Bypass' },
];

// Model suggestions per runtime
const MODEL_SUGGESTIONS: Record<RuntimeId, string[]> = {
  claude: ['default', 'claude-sonnet-4-6', 'claude-opus-4-6', 'claude-haiku-4-5-20251001'],
  codex: ['default', 'gpt-4o', 'o4-mini'],
  gemini: ['default', 'gemini-2.5-pro', 'gemini-2.5-flash'],
  opencode: ['default'],
  custom: ['default'],
};

export function NewSessionDialog({ open: isOpen, onClose, onCreate }: NewSessionDialogProps) {
  const providers = useProviderStore((s) => s.providers);
  const activeProviderId = useProviderStore((s) => s.activeProviderId);

  const [runtimeId, setRuntimeId] = useState<RuntimeId>('claude');
  const [providerId, setProviderId] = useState<string>(activeProviderId ?? '');
  const [model, setModel] = useState('default');
  const [cwd, setCwd] = useState('');
  const [permissionMode, setPermissionMode] = useState<NewSessionConfig['permissionMode']>('ask');
  const [runtimeStatuses, setRuntimeStatuses] = useState<Record<RuntimeId, RuntimeStatus>>({
    claude: 'checking',
    codex: 'not-detected',
    gemini: 'not-detected',
    opencode: 'not-detected',
    custom: 'detected', // custom is always "available"
  });
  const [showModelSuggestions, setShowModelSuggestions] = useState(false);
  const modelSuggestRef = useRef<HTMLDivElement>(null);

  // Sync activeProviderId when dialog opens
  useEffect(() => {
    if (isOpen) {
      setProviderId(activeProviderId ?? providers[0]?.id ?? '');
    }
  }, [isOpen, activeProviderId, providers]);

  // Detect Claude CLI on open
  useEffect(() => {
    if (!isOpen) return;
    setRuntimeStatuses((prev) => ({ ...prev, claude: 'checking' }));
    bridge.checkClaudeCli()
      .then((status) => {
        setRuntimeStatuses((prev) => ({
          ...prev,
          claude: status.installed ? 'detected' : 'not-detected',
        }));
      })
      .catch(() => {
        setRuntimeStatuses((prev) => ({ ...prev, claude: 'not-detected' }));
      });
  }, [isOpen]);

  // Close model suggestions on outside click
  useEffect(() => {
    if (!showModelSuggestions) return;
    const handler = (e: MouseEvent) => {
      if (modelSuggestRef.current && !modelSuggestRef.current.contains(e.target as Node)) {
        setShowModelSuggestions(false);
      }
    };
    document.addEventListener('mousedown', handler);
    return () => document.removeEventListener('mousedown', handler);
  }, [showModelSuggestions]);

  if (!isOpen) return null;

  const handleBrowseCwd = async () => {
    try {
      const selected = await openDialog({ directory: true, multiple: false });
      if (selected && typeof selected === 'string') {
        setCwd(selected);
      }
    } catch {
      // Dialog cancelled or failed
    }
  };

  const handleCreate = () => {
    if (!providerId) return;
    onCreate({ runtimeId, providerId, model: model || 'default', cwd, permissionMode });
  };

  const suggestions = MODEL_SUGGESTIONS[runtimeId];

  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center"
      onClick={(e) => { if (e.target === e.currentTarget) onClose(); }}
    >
      {/* Backdrop */}
      <div className="absolute inset-0 bg-black/50 backdrop-blur-sm" />

      {/* Dialog */}
      <div className="relative z-10 w-full max-w-md bg-bg-card border border-border-subtle rounded-xl shadow-2xl p-6">
        {/* Header */}
        <div className="flex items-center justify-between mb-5">
          <h2 className="text-sm font-semibold text-text-primary">新建会话</h2>
          <button
            onClick={onClose}
            className="text-text-muted hover:text-text-primary transition-smooth p-1 rounded-md hover:bg-bg-secondary"
          >
            <svg width="14" height="14" viewBox="0 0 16 16" fill="none"
              stroke="currentColor" strokeWidth="1.5" strokeLinecap="round">
              <path d="M3 3l10 10M13 3L3 13" />
            </svg>
          </button>
        </div>

        <div className="space-y-4">
          {/* Runtime selector */}
          <div>
            <label className="block text-xs font-medium text-text-muted mb-2">运行时</label>
            <div className="space-y-1.5">
              {RUNTIME_IDS.map((id) => {
                const status = runtimeStatuses[id];
                const isSelected = runtimeId === id;
                return (
                  <label
                    key={id}
                    className={`flex items-center gap-3 px-3 py-2 rounded-lg border cursor-pointer transition-smooth
                      ${isSelected
                        ? 'border-accent/50 bg-accent/5'
                        : 'border-border-subtle bg-bg-secondary/40 hover:bg-bg-secondary'
                      }`}
                  >
                    <input
                      type="radio"
                      name="runtime"
                      value={id}
                      checked={isSelected}
                      onChange={() => {
                        setRuntimeId(id);
                        setModel('default');
                      }}
                      className="sr-only"
                    />
                    <div className={`w-3.5 h-3.5 rounded-full border-2 flex items-center justify-center flex-shrink-0
                      ${isSelected ? 'border-accent' : 'border-border-subtle'}`}>
                      {isSelected && <div className="w-1.5 h-1.5 rounded-full bg-accent" />}
                    </div>
                    <span className={`text-xs flex-1 ${isSelected ? 'text-text-primary font-medium' : 'text-text-muted'}`}>
                      {RUNTIME_LABELS[id]}
                    </span>
                    {status === 'checking' && (
                      <span className="text-xs text-text-muted flex items-center gap-1">
                        <svg className="animate-spin" width="10" height="10" viewBox="0 0 16 16" fill="none"
                          stroke="currentColor" strokeWidth="2">
                          <path d="M8 2a6 6 0 110 12A6 6 0 018 2z" strokeOpacity="0.3" />
                          <path d="M14 8a6 6 0 00-6-6" strokeLinecap="round" />
                        </svg>
                        检测中
                      </span>
                    )}
                    {status === 'detected' && (
                      <span className="text-xs text-green-400 flex items-center gap-1">
                        <svg width="10" height="10" viewBox="0 0 12 12" fill="none"
                          stroke="currentColor" strokeWidth="1.5" strokeLinecap="round">
                          <path d="M2 6l2.5 2.5L10 4" />
                        </svg>
                        已检测
                      </span>
                    )}
                    {status === 'not-detected' && (
                      <span className="text-xs text-text-muted/60 flex items-center gap-1">
                        <svg width="10" height="10" viewBox="0 0 12 12" fill="none"
                          stroke="currentColor" strokeWidth="1.5" strokeLinecap="round">
                          <path d="M3 3l6 6M9 3L3 9" />
                        </svg>
                        未检测
                      </span>
                    )}
                  </label>
                );
              })}
            </div>
          </div>

          {/* Provider selector */}
          <div>
            <label className="block text-xs font-medium text-text-muted mb-1.5">Provider</label>
            {providers.length === 0 ? (
              <p className="text-xs text-text-muted/60 px-3 py-2 bg-bg-secondary/40 rounded-lg border border-border-subtle">
                暂无 Provider — 请先在设置中添加
              </p>
            ) : (
              <select
                value={providerId}
                onChange={(e) => setProviderId(e.target.value)}
                className="w-full px-3 py-2 text-xs bg-bg-secondary/40 border border-border-subtle
                  rounded-lg text-text-primary focus:outline-none focus:border-accent/50
                  cursor-pointer transition-smooth"
              >
                {providers.map((p) => (
                  <option key={p.id} value={p.id} className="bg-bg-card text-text-primary">
                    {p.name}
                  </option>
                ))}
              </select>
            )}
          </div>

          {/* Model input */}
          <div>
            <label className="block text-xs font-medium text-text-muted mb-1.5">模型</label>
            <div ref={modelSuggestRef} className="relative">
              <input
                type="text"
                value={model}
                onChange={(e) => setModel(e.target.value)}
                onFocus={() => setShowModelSuggestions(true)}
                placeholder="default"
                className="w-full px-3 py-2 text-xs bg-bg-secondary/40 border border-border-subtle
                  rounded-lg text-text-primary placeholder:text-text-muted/40
                  focus:outline-none focus:border-accent/50 transition-smooth"
              />
              {showModelSuggestions && suggestions.length > 0 && (
                <div className="absolute top-full left-0 right-0 mt-1 bg-bg-card border border-border-subtle
                  rounded-lg shadow-lg py-1 z-10">
                  {suggestions.map((s) => (
                    <button
                      key={s}
                      type="button"
                      onMouseDown={(e) => {
                        e.preventDefault();
                        setModel(s);
                        setShowModelSuggestions(false);
                      }}
                      className={`w-full text-left px-3 py-1.5 text-xs transition-smooth
                        ${model === s
                          ? 'text-accent bg-accent/5'
                          : 'text-text-muted hover:text-text-primary hover:bg-bg-secondary'
                        }`}
                    >
                      {s}
                    </button>
                  ))}
                </div>
              )}
            </div>
          </div>

          {/* Working directory */}
          <div>
            <label className="block text-xs font-medium text-text-muted mb-1.5">工作目录</label>
            <div className="flex gap-2">
              <input
                type="text"
                value={cwd}
                onChange={(e) => setCwd(e.target.value)}
                placeholder="留空则使用默认目录"
                className="flex-1 px-3 py-2 text-xs bg-bg-secondary/40 border border-border-subtle
                  rounded-lg text-text-primary placeholder:text-text-muted/40
                  focus:outline-none focus:border-accent/50 transition-smooth min-w-0"
              />
              <button
                type="button"
                onClick={handleBrowseCwd}
                className="px-3 py-2 text-xs bg-bg-secondary border border-border-subtle rounded-lg
                  text-text-muted hover:text-text-primary hover:bg-bg-secondary/80
                  transition-smooth flex-shrink-0 flex items-center gap-1.5"
              >
                <svg width="12" height="12" viewBox="0 0 16 16" fill="none"
                  stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round">
                  <path d="M2 4.5A1.5 1.5 0 013.5 3h2.586a1 1 0 01.707.293L8.5 5h4A1.5 1.5 0 0114 6.5v6A1.5 1.5 0 0112.5 14h-9A1.5 1.5 0 012 12.5v-8z" />
                </svg>
                浏览
              </button>
            </div>
          </div>

          {/* Permission mode */}
          <div>
            <label className="block text-xs font-medium text-text-muted mb-2">权限模式</label>
            <div className="grid grid-cols-4 gap-1.5">
              {PERMISSION_MODES.map(({ id, label }) => {
                const isSelected = permissionMode === id;
                const isBypass = id === 'bypass';
                return (
                  <button
                    key={id}
                    type="button"
                    onClick={() => setPermissionMode(id)}
                    className={`px-2 py-1.5 text-xs rounded-lg border transition-smooth text-center
                      ${isSelected
                        ? isBypass
                          ? 'border-warning/50 bg-warning/10 text-warning font-medium'
                          : 'border-accent/50 bg-accent/10 text-accent font-medium'
                        : 'border-border-subtle bg-bg-secondary/40 text-text-muted hover:text-text-primary hover:bg-bg-secondary'
                      }`}
                  >
                    {label}
                  </button>
                );
              })}
            </div>
          </div>
        </div>

        {/* Footer buttons */}
        <div className="flex justify-end gap-2 mt-6">
          <button
            type="button"
            onClick={onClose}
            className="px-4 py-2 text-xs text-text-muted hover:text-text-primary
              bg-bg-secondary/40 hover:bg-bg-secondary border border-border-subtle
              rounded-lg transition-smooth"
          >
            取消
          </button>
          <button
            type="button"
            onClick={handleCreate}
            disabled={!providerId}
            className="px-4 py-2 text-xs text-white bg-accent hover:bg-accent/90
              rounded-lg transition-smooth disabled:opacity-40 disabled:cursor-not-allowed font-medium"
          >
            创建
          </button>
        </div>
      </div>
    </div>
  );
}
