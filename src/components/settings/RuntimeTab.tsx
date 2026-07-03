import { useEffect } from 'react';
import { useRuntimeStore, type RuntimeInfo } from '../../stores/runtimeStore';
import { RUNTIME_ICONS, RUNTIME_LABELS, type RuntimeId } from '../../lib/types/runtime';

/** Backend may include an optional path field not yet reflected in the base type. */
type RuntimeInfoWithPath = RuntimeInfo & { path?: string };

export function RuntimeTab() {
  const runtimes = useRuntimeStore((s) => s.runtimes) as RuntimeInfoWithPath[];
  const isLoading = useRuntimeStore((s) => s.isLoading);
  const fetchRuntimes = useRuntimeStore((s) => s.fetchRuntimes);

  useEffect(() => {
    fetchRuntimes();
  }, [fetchRuntimes]);

  return (
    <div className="space-y-4">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-2">
          <h3 className="text-[13px] font-medium text-text-primary">运行时</h3>
          <span className="text-xs text-text-tertiary">{runtimes.length}</span>
        </div>
        <button
          onClick={() => fetchRuntimes()}
          disabled={isLoading}
          className="p-1.5 rounded hover:bg-bg-secondary text-text-tertiary transition-smooth
            disabled:opacity-50 disabled:cursor-not-allowed"
          title="刷新运行时状态"
        >
          <svg
            width="14"
            height="14"
            viewBox="0 0 12 12"
            fill="none"
            stroke="currentColor"
            strokeWidth="1.5"
            className={isLoading ? 'animate-spin' : ''}
          >
            <path d="M1 6a5 5 0 019-2M11 6a5 5 0 01-9 2" />
            <path d="M10 1v3h-3M2 11V8h3" />
          </svg>
        </button>
      </div>

      {/* Runtime cards */}
      {isLoading && runtimes.length === 0 ? (
        <div className="flex items-center justify-center py-8">
          <div className="w-5 h-5 border-2 border-accent/30 border-t-accent rounded-full animate-spin" />
        </div>
      ) : runtimes.length === 0 ? (
        <p className="text-[13px] text-text-tertiary text-center py-6">
          暂无运行时信息
        </p>
      ) : (
        <div className="space-y-2">
          {runtimes.map((runtime) => (
            <RuntimeCard
              key={runtime.id}
              id={runtime.id as RuntimeId}
              name={runtime.name}
              detected={runtime.detected}
              version={runtime.version}
              path={runtime.path}
            />
          ))}
        </div>
      )}

      {/* Custom CLI note */}
      <div className="rounded-lg border border-border-subtle bg-bg-secondary/30 px-3 py-2.5">
        <p className="text-xs text-text-muted leading-relaxed">
          <span className="font-medium text-text-primary">Custom CLI</span>
          {' '}在新建会话时配置
        </p>
      </div>
    </div>
  );
}

function RuntimeCard({
  id,
  name,
  detected,
  version,
  path,
}: {
  id: RuntimeId;
  name: string;
  detected: boolean;
  version?: string;
  path?: string;
}) {
  const icon = RUNTIME_ICONS[id] ?? '⚙️';
  const label = name || RUNTIME_LABELS[id] || id;

  return (
    <div className="px-3 py-2.5 rounded-lg border border-border-subtle
      bg-bg-secondary/20 hover:bg-bg-secondary transition-smooth">
      <div className="flex items-center gap-3">
        {/* Icon */}
        <span className="text-base leading-none flex-shrink-0 w-5 text-center" aria-hidden="true">
          {icon}
        </span>

        {/* Name */}
        <span className="flex-1 text-[13px] font-medium text-text-primary truncate">
          {label}
        </span>

        {/* Status */}
        <span
          className={`text-xs font-medium flex-shrink-0 ${
            detected ? 'text-green-500' : 'text-red-400'
          }`}
        >
          {detected
            ? `✅ 已检测${version ? ` ${version}` : ''}`
            : '❌ 未检测'}
        </span>
      </div>

      {/* Path — shown only when detected and available */}
      {detected && path && (
        <p className="mt-1 pl-8 text-[11px] text-text-tertiary font-mono truncate" title={path}>
          {path}
        </p>
      )}
    </div>
  );
}
